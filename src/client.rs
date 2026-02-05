use crate::{Error, RateLimitConfig, CacheConfig, MetricsCollector, validation::Validator};
use reqwest::Client as HttpClient;
use std::sync::Arc;
use std::time::Duration;

/// Configuration options for the GoldRush client.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL for the GoldRush API.
    /// TODO: Confirm exact base URL with maintainers - currently using docs example
    pub base_url: String,
    
    /// Request timeout duration.
    pub timeout: Duration,
    
    /// Maximum number of retry attempts for failed requests.
    pub max_retries: u8,
    
    /// User agent string for requests.
    pub user_agent: String,
    
    /// Rate limiting configuration.
    pub rate_limit: RateLimitConfig,
    
    /// Caching configuration.
    pub cache: CacheConfig,
    
    /// Enable request/response logging.
    pub enable_logging: bool,
    
    /// Enable metrics collection.
    pub enable_metrics: bool,
    
    /// Connection pool size.
    pub connection_pool_size: usize,
    
    /// Keep-alive timeout for connections.
    pub keep_alive_timeout: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            // TODO: Confirm exact base URL from official GoldRush docs
            base_url: "https://api.goldrush.dev".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            user_agent: format!("goldrush-sdk-rs/{}", env!("CARGO_PKG_VERSION")),
            rate_limit: RateLimitConfig::default(),
            cache: CacheConfig::default(),
            enable_logging: true,
            enable_metrics: true,
            connection_pool_size: 10,
            keep_alive_timeout: Duration::from_secs(90),
        }
    }
}

impl ClientConfig {
    /// Create a new ClientConfig with custom base URL.
    pub fn new<S: Into<String>>(base_url: S) -> Self {
        Self {
            base_url: base_url.into(),
            ..Default::default()
        }
    }

    /// Set the timeout for requests.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the maximum number of retry attempts.
    pub fn with_max_retries(mut self, max_retries: u8) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Set a custom user agent.
    pub fn with_user_agent<S: Into<String>>(mut self, user_agent: S) -> Self {
        self.user_agent = user_agent.into();
        self
    }
}

/// The main GoldRush client for interacting with the API.
pub struct GoldRushClient {
    pub(crate) http: HttpClient,
    pub(crate) api_key: String,
    pub(crate) config: ClientConfig,
    pub(crate) metrics: Option<Arc<MetricsCollector>>,
}

impl GoldRushClient {
    /// Create a new GoldRush client with the provided API key and configuration.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your GoldRush API key
    /// * `config` - Client configuration options
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use goldrush_sdk::{GoldRushClient, ClientConfig};
    ///
    /// let client = GoldRushClient::new("your-api-key", ClientConfig::default())?;
    /// # Ok::<(), goldrush_sdk::Error>(())
    /// ```
    pub fn new<S: Into<String>>(api_key: S, config: ClientConfig) -> Result<Self, Error> {
        let api_key = api_key.into();
        
        // Validate API key
        Validator::validate_api_key(&api_key)?;
        
        // Validate base URL
        Validator::validate_url(&config.base_url)?;

        let http = HttpClient::builder()
            .user_agent(&config.user_agent)
            .timeout(config.timeout)
            .pool_max_idle_per_host(config.connection_pool_size)
            .pool_idle_timeout(config.keep_alive_timeout)
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .build()?;

        let metrics = if config.enable_metrics {
            Some(Arc::new(MetricsCollector::new()))
        } else {
            None
        };

        Ok(Self { 
            http, 
            api_key, 
            config, 
            metrics 
        })
    }

    /// Create a new GoldRush client with default configuration.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your GoldRush API key
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use goldrush_sdk::GoldRushClient;
    ///
    /// let client = GoldRushClient::with_key("your-api-key")?;
    /// # Ok::<(), goldrush_sdk::Error>(())
    /// ```
    pub fn with_key<S: Into<String>>(api_key: S) -> Result<Self, Error> {
        Self::new(api_key, ClientConfig::default())
    }

    /// Get access to the metrics collector (if enabled).
    pub fn metrics(&self) -> Option<&Arc<MetricsCollector>> {
        self.metrics.as_ref()
    }
}