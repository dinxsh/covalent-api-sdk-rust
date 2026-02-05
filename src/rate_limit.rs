use crate::{Error, Result};
use backoff::{ExponentialBackoff, ExponentialBackoffBuilder};
use std::time::Duration;
use tracing::{debug, warn, instrument};

/// Rate limiting configuration for API requests.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per second.
    pub max_requests_per_second: f64,
    /// Burst capacity for short-term spikes.
    pub burst_capacity: u32,
    /// Enable exponential backoff for failed requests.
    pub enable_backoff: bool,
    /// Maximum retry attempts.
    pub max_retries: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests_per_second: 10.0, // Conservative default
            burst_capacity: 20,
            enable_backoff: true,
            max_retries: 3,
        }
    }
}

/// Token bucket rate limiter.
#[derive(Debug)]
pub struct RateLimiter {
    config: RateLimitConfig,
    tokens: std::sync::Arc<tokio::sync::Mutex<f64>>,
    last_refill: std::sync::Arc<tokio::sync::Mutex<std::time::Instant>>,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            tokens: std::sync::Arc::new(tokio::sync::Mutex::new(config.burst_capacity as f64)),
            last_refill: std::sync::Arc::new(tokio::sync::Mutex::new(std::time::Instant::now())),
            config,
        }
    }

    /// Check if a request can proceed, applying rate limiting if necessary.
    pub async fn acquire(&self) -> Result<()> {
        self.acquire_internal().await
    }
    
    #[instrument(skip(self), fields(max_rps = %self.config.max_requests_per_second))]
    async fn acquire_internal(&self) -> Result<()> {
        let mut tokens = self.tokens.lock().await;
        let mut last_refill = self.last_refill.lock().await;
        
        let now = std::time::Instant::now();
        let time_elapsed = now.duration_since(*last_refill);
        
        // Refill tokens based on elapsed time
        let tokens_to_add = time_elapsed.as_secs_f64() * self.config.max_requests_per_second;
        *tokens = (*tokens + tokens_to_add).min(self.config.burst_capacity as f64);
        *last_refill = now;
        
        if *tokens >= 1.0 {
            *tokens -= 1.0;
            debug!(tokens_remaining = %*tokens, "Rate limit check passed");
            Ok(())
        } else {
            let wait_time = Duration::from_millis(
                ((1.0 - *tokens) / self.config.max_requests_per_second * 1000.0) as u64
            );
            
            warn!(
                wait_time_ms = %wait_time.as_millis(),
                "Rate limit exceeded, waiting"
            );
            
            // Release locks before waiting
            drop(tokens);
            drop(last_refill);
            
            tokio::time::sleep(wait_time).await;
            Box::pin(self.acquire_internal()).await
        }
    }
}

/// Create exponential backoff strategy for retries.
pub fn create_backoff_strategy(config: &RateLimitConfig) -> ExponentialBackoff {
    ExponentialBackoffBuilder::new()
        .with_initial_interval(Duration::from_millis(100))
        .with_max_interval(Duration::from_secs(30))
        .with_multiplier(2.0)
        .with_max_elapsed_time(Some(Duration::from_secs(300))) // 5 minutes max
        .build()
}

/// Retry a request with exponential backoff.
#[instrument(skip(operation), fields(max_retries = %max_retries))]
pub async fn retry_with_backoff<F, T>(
    operation: F,
    max_retries: u32,
) -> Result<T>
where
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send + 'static>>,
    T: Send + 'static,
{
    let backoff = ExponentialBackoffBuilder::new()
        .with_initial_interval(Duration::from_millis(100))
        .with_max_interval(Duration::from_secs(30))
        .with_multiplier(2.0)
        .build();
    
    let mut attempts = 0;
    let mut current_wait = Duration::from_millis(100);
    
    loop {
        attempts += 1;
        
        match operation().await {
            Ok(result) => {
                debug!(attempts = %attempts, "Request succeeded");
                return Ok(result);
            }
            Err(err) => {
                if attempts >= max_retries {
                    warn!(attempts = %attempts, "Max retries exceeded");
                    return Err(err);
                }
                
                warn!(
                    attempt = %attempts,
                    wait_time_ms = %current_wait.as_millis(),
                    error = %err,
                    "Request failed, retrying with backoff"
                );
                
                tokio::time::sleep(current_wait).await;
                current_wait = std::cmp::min(current_wait * 2, Duration::from_secs(30));
            }
        }
    }
}