use crate::{Error, Result};
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn, instrument};

/// Circuit breaker states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Circuit is closed, requests flow normally.
    Closed,
    /// Circuit is open, requests are rejected immediately.
    Open,
    /// Circuit is half-open, allowing limited requests to test recovery.
    HalfOpen,
}

impl std::fmt::Display for CircuitState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircuitState::Closed => write!(f, "CLOSED"),
            CircuitState::Open => write!(f, "OPEN"),
            CircuitState::HalfOpen => write!(f, "HALF_OPEN"),
        }
    }
}

/// Configuration for the circuit breaker.
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of consecutive failures before opening the circuit.
    pub failure_threshold: u32,
    /// Duration to keep circuit open before attempting recovery.
    pub timeout: Duration,
    /// Number of successful requests needed to close circuit from half-open.
    pub success_threshold: u32,
    /// Time window for counting failures.
    pub failure_time_window: Duration,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            timeout: Duration::from_secs(30),
            success_threshold: 3,
            failure_time_window: Duration::from_secs(60),
        }
    }
}

/// Circuit breaker for preventing cascading failures.
#[derive(Debug)]
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    state: Arc<RwLock<CircuitState>>,
    failure_count: AtomicU64,
    success_count: AtomicU64,
    last_failure_time: Arc<RwLock<Option<Instant>>>,
    last_state_change: Arc<RwLock<Instant>>,
    /// Recent failures within the time window
    recent_failures: Arc<RwLock<Vec<Instant>>>,
}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            failure_count: AtomicU64::new(0),
            success_count: AtomicU64::new(0),
            last_failure_time: Arc::new(RwLock::new(None)),
            last_state_change: Arc::new(RwLock::new(Instant::now())),
            recent_failures: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Check if a request can proceed.
    #[instrument(skip(self))]
    pub async fn can_proceed(&self) -> bool {
        let current_state = *self.state.read().await;
        
        match current_state {
            CircuitState::Closed => {
                debug!("Circuit closed, allowing request");
                true
            }
            CircuitState::Open => {
                let should_attempt_reset = self.should_attempt_reset().await;
                if should_attempt_reset {
                    self.transition_to_half_open().await;
                    debug!("Circuit transitioning to half-open, allowing request");
                    true
                } else {
                    debug!("Circuit open, rejecting request");
                    false
                }
            }
            CircuitState::HalfOpen => {
                debug!("Circuit half-open, allowing limited request");
                true
            }
        }
    }
    
    /// Record a successful operation.
    #[instrument(skip(self))]
    pub async fn record_success(&self) {
        let current_state = *self.state.read().await;
        self.success_count.fetch_add(1, Ordering::Relaxed);
        
        debug!(state = %current_state, "Recording success");
        
        match current_state {
            CircuitState::HalfOpen => {
                let success_count = self.success_count.load(Ordering::Relaxed);
                if success_count >= self.config.success_threshold as u64 {
                    self.transition_to_closed().await;
                }
            }
            CircuitState::Open => {
                // Success in open state should not happen, but if it does,
                // it might indicate the service is recovering
                warn!("Unexpected success in open circuit state");
            }
            CircuitState::Closed => {
                // Normal operation, no state change needed
            }
        }
    }
    
    /// Record a failed operation.
    #[instrument(skip(self), fields(error = %error))]
    pub async fn record_failure<E: std::fmt::Display>(&self, error: E) {
        let now = Instant::now();
        let current_state = *self.state.read().await;
        
        debug!(state = %current_state, error = %error, "Recording failure");
        
        self.failure_count.fetch_add(1, Ordering::Relaxed);
        *self.last_failure_time.write().await = Some(now);
        
        // Add to recent failures
        {
            let mut recent_failures = self.recent_failures.write().await;
            recent_failures.push(now);
            
            // Clean up old failures outside the time window
            let cutoff = now - self.config.failure_time_window;
            recent_failures.retain(|&failure_time| failure_time > cutoff);
        }
        
        // Check if we should open the circuit
        let recent_failure_count = self.recent_failures.read().await.len();
        
        if current_state != CircuitState::Open 
            && recent_failure_count >= self.config.failure_threshold as usize {
            self.transition_to_open().await;
        }
    }
    
    /// Get current circuit breaker statistics.
    pub async fn stats(&self) -> CircuitBreakerStats {
        let state = *self.state.read().await;
        let recent_failures = self.recent_failures.read().await.len();
        let last_state_change = *self.last_state_change.read().await;
        
        CircuitBreakerStats {
            state,
            total_failures: self.failure_count.load(Ordering::Relaxed),
            total_successes: self.success_count.load(Ordering::Relaxed),
            recent_failures: recent_failures as u64,
            time_in_current_state: last_state_change.elapsed(),
            failure_rate: self.calculate_failure_rate().await,
        }
    }
    
    /// Reset the circuit breaker to closed state.
    #[instrument(skip(self))]
    pub async fn reset(&self) {
        info!("Manually resetting circuit breaker");
        self.transition_to_closed().await;
        self.failure_count.store(0, Ordering::Relaxed);
        self.success_count.store(0, Ordering::Relaxed);
        *self.last_failure_time.write().await = None;
        self.recent_failures.write().await.clear();
    }
    
    /// Check if enough time has passed to attempt reset from open state.
    async fn should_attempt_reset(&self) -> bool {
        let last_state_change = *self.last_state_change.read().await;
        last_state_change.elapsed() >= self.config.timeout
    }
    
    /// Transition to closed state.
    async fn transition_to_closed(&self) {
        let mut state = self.state.write().await;
        if *state != CircuitState::Closed {
            info!(previous_state = %*state, "Circuit breaker transitioning to CLOSED");
            *state = CircuitState::Closed;
            *self.last_state_change.write().await = Instant::now();
            self.success_count.store(0, Ordering::Relaxed);
        }
    }
    
    /// Transition to open state.
    async fn transition_to_open(&self) {
        let mut state = self.state.write().await;
        if *state != CircuitState::Open {
            warn!(previous_state = %*state, "Circuit breaker transitioning to OPEN");
            *state = CircuitState::Open;
            *self.last_state_change.write().await = Instant::now();
        }
    }
    
    /// Transition to half-open state.
    async fn transition_to_half_open(&self) {
        let mut state = self.state.write().await;
        if *state != CircuitState::HalfOpen {
            info!(previous_state = %*state, "Circuit breaker transitioning to HALF_OPEN");
            *state = CircuitState::HalfOpen;
            *self.last_state_change.write().await = Instant::now();
            self.success_count.store(0, Ordering::Relaxed);
        }
    }
    
    /// Calculate current failure rate.
    async fn calculate_failure_rate(&self) -> f64 {
        let recent_failures = self.recent_failures.read().await;
        let total_failures = self.failure_count.load(Ordering::Relaxed);
        let total_successes = self.success_count.load(Ordering::Relaxed);
        let total_requests = total_failures + total_successes;
        
        if total_requests == 0 {
            0.0
        } else {
            (recent_failures.len() as f64) / (total_requests as f64) * 100.0
        }
    }
}

/// Circuit breaker statistics.
#[derive(Debug, Clone)]
pub struct CircuitBreakerStats {
    pub state: CircuitState,
    pub total_failures: u64,
    pub total_successes: u64,
    pub recent_failures: u64,
    pub time_in_current_state: Duration,
    pub failure_rate: f64,
}

impl CircuitBreakerStats {
    /// Check if the circuit breaker is healthy.
    pub fn is_healthy(&self) -> bool {
        matches!(self.state, CircuitState::Closed) && self.failure_rate < 5.0
    }
    
    /// Get a human-readable status string.
    pub fn status_string(&self) -> String {
        format!(
            "Circuit: {} | Failures: {}/{} | Rate: {:.1}% | Uptime: {}s",
            self.state,
            self.recent_failures,
            self.total_failures + self.total_successes,
            self.failure_rate,
            self.time_in_current_state.as_secs()
        )
    }
}

/// Wrapper for executing operations through a circuit breaker.
pub struct CircuitBreakerExecutor {
    circuit_breaker: Arc<CircuitBreaker>,
}

impl CircuitBreakerExecutor {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            circuit_breaker: Arc::new(CircuitBreaker::new(config)),
        }
    }
    
    /// Execute an operation through the circuit breaker.
    #[instrument(skip(self, operation))]
    pub async fn execute<F, T, E>(&self, operation: F) -> Result<T>
    where
        F: std::future::Future<Output = std::result::Result<T, E>>,
        E: std::fmt::Display + std::error::Error + Send + Sync + 'static,
    {
        // Check if request can proceed
        if !self.circuit_breaker.can_proceed().await {
            return Err(Error::Config("Circuit breaker is open, request rejected".to_string()));
        }
        
        // Execute the operation
        match operation.await {
            Ok(result) => {
                self.circuit_breaker.record_success().await;
                Ok(result)
            }
            Err(error) => {
                self.circuit_breaker.record_failure(&error).await;
                Err(Error::Config(format!("Operation failed: {}", error)))
            }
        }
    }
    
    /// Get circuit breaker statistics.
    pub async fn stats(&self) -> CircuitBreakerStats {
        self.circuit_breaker.stats().await
    }
    
    /// Reset the circuit breaker.
    pub async fn reset(&self) {
        self.circuit_breaker.reset().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};
    
    #[tokio::test]
    async fn test_circuit_breaker_states() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            timeout: Duration::from_millis(100),
            success_threshold: 1,
            failure_time_window: Duration::from_secs(10),
        };
        
        let cb = CircuitBreaker::new(config);
        
        // Initially closed
        assert!(cb.can_proceed().await);
        
        // Record failures to open circuit
        cb.record_failure("test error 1").await;
        cb.record_failure("test error 2").await;
        
        // Should be open now
        assert!(!cb.can_proceed().await);
        
        // Wait for timeout and transition to half-open
        sleep(Duration::from_millis(150)).await;
        assert!(cb.can_proceed().await);
        
        // Record success to close circuit
        cb.record_success().await;
        assert!(cb.can_proceed().await);
        
        let stats = cb.stats().await;
        assert_eq!(stats.state, CircuitState::Closed);
    }
}