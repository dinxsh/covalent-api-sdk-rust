use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, instrument};

/// Metrics collector for SDK operations.
#[derive(Debug)]
pub struct MetricsCollector {
    /// Total number of requests made.
    request_count: AtomicU64,
    /// Total number of successful requests.
    success_count: AtomicU64,
    /// Total number of failed requests.
    error_count: AtomicU64,
    /// Cache hit count.
    cache_hits: AtomicU64,
    /// Cache miss count.
    cache_misses: AtomicU64,
    /// Rate limit hit count.
    rate_limit_hits: AtomicU64,
    /// Response time tracking.
    response_times: Arc<RwLock<ResponseTimeTracker>>,
    /// Error breakdown by type.
    error_breakdown: Arc<RwLock<HashMap<String, u64>>>,
    /// Request breakdown by endpoint.
    endpoint_stats: Arc<RwLock<HashMap<String, EndpointStats>>>,
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            request_count: AtomicU64::new(0),
            success_count: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            rate_limit_hits: AtomicU64::new(0),
            response_times: Arc::new(RwLock::new(ResponseTimeTracker::new())),
            error_breakdown: Arc::new(RwLock::new(HashMap::new())),
            endpoint_stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Record a successful request.
    #[instrument(skip(self), fields(endpoint = %endpoint, response_time_ms = %response_time.as_millis()))]
    pub async fn record_success(&self, endpoint: &str, response_time: Duration) {
        self.request_count.fetch_add(1, Ordering::Relaxed);
        self.success_count.fetch_add(1, Ordering::Relaxed);
        
        // Update response times
        self.response_times.write().await.add_sample(response_time);
        
        // Update endpoint stats
        let mut endpoint_stats = self.endpoint_stats.write().await;
        let stats = endpoint_stats.entry(endpoint.to_string()).or_insert_with(EndpointStats::new);
        stats.request_count += 1;
        stats.success_count += 1;
        stats.total_response_time += response_time;
        
        debug!("Success metrics recorded");
    }
    
    /// Record a failed request.
    #[instrument(skip(self), fields(endpoint = %endpoint, error_type = %error_type))]
    pub async fn record_error(&self, endpoint: &str, error_type: &str) {
        self.request_count.fetch_add(1, Ordering::Relaxed);
        self.error_count.fetch_add(1, Ordering::Relaxed);
        
        // Update error breakdown
        let mut error_breakdown = self.error_breakdown.write().await;
        *error_breakdown.entry(error_type.to_string()).or_insert(0) += 1;
        
        // Update endpoint stats
        let mut endpoint_stats = self.endpoint_stats.write().await;
        let stats = endpoint_stats.entry(endpoint.to_string()).or_insert_with(EndpointStats::new);
        stats.request_count += 1;
        stats.error_count += 1;
        
        debug!("Error metrics recorded");
    }
    
    /// Record a cache hit.
    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
        debug!("Cache hit recorded");
    }
    
    /// Record a cache miss.
    pub fn record_cache_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
        debug!("Cache miss recorded");
    }
    
    /// Record a rate limit hit.
    pub fn record_rate_limit_hit(&self) {
        self.rate_limit_hits.fetch_add(1, Ordering::Relaxed);
        debug!("Rate limit hit recorded");
    }
    
    /// Get comprehensive metrics summary.
    pub async fn get_metrics(&self) -> MetricsSummary {
        let response_times = self.response_times.read().await;
        let error_breakdown = self.error_breakdown.read().await.clone();
        let endpoint_stats = self.endpoint_stats.read().await.clone();
        
        MetricsSummary {
            request_count: self.request_count.load(Ordering::Relaxed),
            success_count: self.success_count.load(Ordering::Relaxed),
            error_count: self.error_count.load(Ordering::Relaxed),
            cache_hits: self.cache_hits.load(Ordering::Relaxed),
            cache_misses: self.cache_misses.load(Ordering::Relaxed),
            rate_limit_hits: self.rate_limit_hits.load(Ordering::Relaxed),
            avg_response_time: response_times.average(),
            p95_response_time: response_times.p95(),
            p99_response_time: response_times.p99(),
            error_breakdown,
            endpoint_stats,
        }
    }
    
    /// Reset all metrics.
    pub async fn reset(&self) {
        self.request_count.store(0, Ordering::Relaxed);
        self.success_count.store(0, Ordering::Relaxed);
        self.error_count.store(0, Ordering::Relaxed);
        self.cache_hits.store(0, Ordering::Relaxed);
        self.cache_misses.store(0, Ordering::Relaxed);
        self.rate_limit_hits.store(0, Ordering::Relaxed);
        
        self.response_times.write().await.reset();
        self.error_breakdown.write().await.clear();
        self.endpoint_stats.write().await.clear();
        
        info!("Metrics reset");
    }
}

/// Track response times with percentile calculations.
#[derive(Debug)]
struct ResponseTimeTracker {
    samples: Vec<Duration>,
    max_samples: usize,
}

impl ResponseTimeTracker {
    fn new() -> Self {
        Self {
            samples: Vec::new(),
            max_samples: 1000, // Keep last 1000 samples
        }
    }
    
    fn add_sample(&mut self, duration: Duration) {
        self.samples.push(duration);
        
        // Keep only the most recent samples
        if self.samples.len() > self.max_samples {
            self.samples.remove(0);
        }
    }
    
    fn average(&self) -> Duration {
        if self.samples.is_empty() {
            return Duration::ZERO;
        }
        
        let total: Duration = self.samples.iter().sum();
        total / self.samples.len() as u32
    }
    
    fn p95(&self) -> Duration {
        self.percentile(0.95)
    }
    
    fn p99(&self) -> Duration {
        self.percentile(0.99)
    }
    
    fn percentile(&self, percentile: f64) -> Duration {
        if self.samples.is_empty() {
            return Duration::ZERO;
        }
        
        let mut sorted = self.samples.clone();
        sorted.sort();
        
        let index = ((sorted.len() as f64 - 1.0) * percentile) as usize;
        sorted.get(index).copied().unwrap_or(Duration::ZERO)
    }
    
    fn reset(&mut self) {
        self.samples.clear();
    }
}

/// Statistics for individual endpoints.
#[derive(Debug, Clone)]
pub struct EndpointStats {
    pub request_count: u64,
    pub success_count: u64,
    pub error_count: u64,
    pub total_response_time: Duration,
}

impl EndpointStats {
    fn new() -> Self {
        Self {
            request_count: 0,
            success_count: 0,
            error_count: 0,
            total_response_time: Duration::ZERO,
        }
    }
    
    pub fn average_response_time(&self) -> Duration {
        if self.request_count == 0 {
            Duration::ZERO
        } else {
            self.total_response_time / self.request_count as u32
        }
    }
    
    pub fn success_rate(&self) -> f64 {
        if self.request_count == 0 {
            0.0
        } else {
            (self.success_count as f64) / (self.request_count as f64) * 100.0
        }
    }
}

/// Complete metrics summary.
#[derive(Debug, Clone)]
pub struct MetricsSummary {
    pub request_count: u64,
    pub success_count: u64,
    pub error_count: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub rate_limit_hits: u64,
    pub avg_response_time: Duration,
    pub p95_response_time: Duration,
    pub p99_response_time: Duration,
    pub error_breakdown: HashMap<String, u64>,
    pub endpoint_stats: HashMap<String, EndpointStats>,
}

impl MetricsSummary {
    /// Calculate success rate percentage.
    pub fn success_rate(&self) -> f64 {
        if self.request_count == 0 {
            0.0
        } else {
            (self.success_count as f64) / (self.request_count as f64) * 100.0
        }
    }
    
    /// Calculate cache hit rate percentage.
    pub fn cache_hit_rate(&self) -> f64 {
        let total_cache_requests = self.cache_hits + self.cache_misses;
        if total_cache_requests == 0 {
            0.0
        } else {
            (self.cache_hits as f64) / (total_cache_requests as f64) * 100.0
        }
    }
    
    /// Get formatted metrics string for logging.
    pub fn format_for_logging(&self) -> String {
        format!(
            "SDK Metrics - Requests: {} ({}% success), Cache Hit Rate: {:.1}%, Avg Response: {}ms",
            self.request_count,
            self.success_rate(),
            self.cache_hit_rate(),
            self.avg_response_time.as_millis()
        )
    }
}

/// Timer utility for measuring operation duration.
#[derive(Debug)]
pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
    
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}