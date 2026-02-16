//! Streaming Configuration
//!
//! Configuration options for WebSocket connections and streaming behavior.

use std::sync::Arc;
use std::time::Duration;

use crate::error::GoldRushError;

/// Configuration for streaming connections
#[derive(Clone)]
pub struct StreamingConfig {
    /// WebSocket endpoint URL
    pub ws_url: String,

    /// Function to determine if reconnection should be attempted
    pub should_retry: Arc<dyn Fn(u32) -> bool + Send + Sync>,

    /// Maximum number of reconnection attempts (0 = unlimited)
    pub max_reconnect_attempts: u32,

    /// Timeout for establishing WebSocket connection
    pub connection_timeout: Duration,

    /// Interval between ping messages for connection health check
    pub ping_interval: Duration,

    /// Timeout waiting for pong response
    pub pong_timeout: Duration,

    /// Automatically resubscribe after reconnection
    pub auto_resubscribe: bool,

    /// Callback invoked when connection is being established
    pub on_connecting: Option<Arc<dyn Fn() + Send + Sync>>,

    /// Callback invoked when connection is established
    pub on_connected: Option<Arc<dyn Fn() + Send + Sync>>,

    /// Callback invoked when connection is closed
    pub on_closed: Option<Arc<dyn Fn() + Send + Sync>>,

    /// Callback invoked when an error occurs
    pub on_error: Option<Arc<dyn Fn(&GoldRushError) + Send + Sync>>,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            ws_url: "wss://gr-staging-v2.streaming.covalenthq.com/graphql".to_string(),
            should_retry: Arc::new(|attempt| attempt < 5),
            max_reconnect_attempts: 5,
            connection_timeout: Duration::from_secs(30),
            ping_interval: Duration::from_secs(30),
            pong_timeout: Duration::from_secs(10),
            auto_resubscribe: true,
            on_connecting: None,
            on_connected: None,
            on_closed: None,
            on_error: None,
        }
    }
}

impl StreamingConfig {
    /// Creates a new configuration builder
    pub fn builder() -> StreamingConfigBuilder {
        StreamingConfigBuilder::new()
    }
}

/// Builder for StreamingConfig
pub struct StreamingConfigBuilder {
    config: StreamingConfig,
}

impl StreamingConfigBuilder {
    /// Creates a new builder with default values
    pub fn new() -> Self {
        Self {
            config: StreamingConfig::default(),
        }
    }

    /// Sets the WebSocket URL
    pub fn ws_url(mut self, url: impl Into<String>) -> Self {
        self.config.ws_url = url.into();
        self
    }

    /// Sets the retry policy
    pub fn should_retry<F>(mut self, f: F) -> Self
    where
        F: Fn(u32) -> bool + Send + Sync + 'static,
    {
        self.config.should_retry = Arc::new(f);
        self
    }

    /// Sets the maximum reconnection attempts
    pub fn max_reconnect_attempts(mut self, max: u32) -> Self {
        self.config.max_reconnect_attempts = max;
        self
    }

    /// Sets the connection timeout
    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.config.connection_timeout = timeout;
        self
    }

    /// Sets the ping interval
    pub fn ping_interval(mut self, interval: Duration) -> Self {
        self.config.ping_interval = interval;
        self
    }

    /// Sets the pong timeout
    pub fn pong_timeout(mut self, timeout: Duration) -> Self {
        self.config.pong_timeout = timeout;
        self
    }

    /// Enables or disables automatic resubscription after reconnection
    pub fn auto_resubscribe(mut self, enabled: bool) -> Self {
        self.config.auto_resubscribe = enabled;
        self
    }

    /// Sets the on_connecting callback
    pub fn on_connecting<F>(mut self, f: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.config.on_connecting = Some(Arc::new(f));
        self
    }

    /// Sets the on_connected callback
    pub fn on_connected<F>(mut self, f: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.config.on_connected = Some(Arc::new(f));
        self
    }

    /// Sets the on_closed callback
    pub fn on_closed<F>(mut self, f: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.config.on_closed = Some(Arc::new(f));
        self
    }

    /// Sets the on_error callback
    pub fn on_error<F>(mut self, f: F) -> Self
    where
        F: Fn(&GoldRushError) + Send + Sync + 'static,
    {
        self.config.on_error = Some(Arc::new(f));
        self
    }

    /// Builds the configuration
    pub fn build(self) -> StreamingConfig {
        self.config
    }
}

impl Default for StreamingConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = StreamingConfig::default();
        assert!(config.ws_url.contains("streaming.covalenthq.com"));
        assert_eq!(config.max_reconnect_attempts, 5);
        assert!(config.auto_resubscribe);
    }

    #[test]
    fn test_builder_pattern() {
        let config = StreamingConfig::builder()
            .ws_url("wss://custom.url")
            .max_reconnect_attempts(10)
            .auto_resubscribe(false)
            .build();

        assert_eq!(config.ws_url, "wss://custom.url");
        assert_eq!(config.max_reconnect_attempts, 10);
        assert!(!config.auto_resubscribe);
    }

    #[test]
    fn test_custom_retry_policy() {
        let config = StreamingConfig::builder()
            .should_retry(|attempt| attempt < 3)
            .build();

        assert!((config.should_retry)(0));
        assert!((config.should_retry)(2));
        assert!(!(config.should_retry)(3));
    }
}
