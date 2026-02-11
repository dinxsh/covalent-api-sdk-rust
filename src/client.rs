use crate::services::{ServiceContext, BalanceService, TransactionService, NftService, BaseService, PricingService, SecurityService, BitcoinService, AllChainsService};
use crate::{Error, RateLimitConfig, CacheConfig, MetricsCollector, validation::Validator};
use reqwest::Client as HttpClient;
use std::sync::Arc;
use std::time::Duration;

/// Configuration options for the GoldRush client.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL for the GoldRush API.
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
            base_url: "https://api.covalenthq.com".to_string(),
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
///
/// Use the service methods to access grouped API endpoints:
///
/// ```rust,no_run
/// use goldrush_sdk::{GoldRushClient, ClientConfig, Chain};
///
/// # async fn example() -> Result<(), goldrush_sdk::Error> {
/// let client = GoldRushClient::new("cqt_your_api_key_here", ClientConfig::default())?;
///
/// // Access balance endpoints
/// let balances = client.balance_service()
///     .get_token_balances_for_wallet_address(Chain::EthereumMainnet, "0x...", None)
///     .await?;
///
/// // Access transaction endpoints
/// let txs = client.transaction_service()
///     .get_all_transactions_for_address(Chain::EthereumMainnet, "0x...", None)
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct GoldRushClient {
    ctx: Arc<ServiceContext>,
}

impl GoldRushClient {
    /// Create a new GoldRush client with the provided API key and configuration.
    pub fn new<S: Into<String>>(api_key: S, config: ClientConfig) -> Result<Self, Error> {
        let api_key = api_key.into();

        Validator::validate_api_key(&api_key)?;
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

        let ctx = Arc::new(ServiceContext {
            http,
            api_key,
            config,
            metrics,
        });

        Ok(Self { ctx })
    }

    /// Create a new GoldRush client with default configuration.
    pub fn with_key<S: Into<String>>(api_key: S) -> Result<Self, Error> {
        Self::new(api_key, ClientConfig::default())
    }

    /// Get access to the metrics collector (if enabled).
    pub fn metrics(&self) -> Option<&Arc<MetricsCollector>> {
        self.ctx.metrics.as_ref()
    }

    /// Access balance-related endpoints.
    pub fn balance_service(&self) -> BalanceService {
        BalanceService::new(Arc::clone(&self.ctx))
    }

    /// Access transaction-related endpoints.
    pub fn transaction_service(&self) -> TransactionService {
        TransactionService::new(Arc::clone(&self.ctx))
    }

    /// Access NFT-related endpoints.
    pub fn nft_service(&self) -> NftService {
        NftService::new(Arc::clone(&self.ctx))
    }

    /// Access base/utility endpoints (blocks, logs, chains, gas prices).
    pub fn base_service(&self) -> BaseService {
        BaseService::new(Arc::clone(&self.ctx))
    }

    /// Access pricing endpoints.
    pub fn pricing_service(&self) -> PricingService {
        PricingService::new(Arc::clone(&self.ctx))
    }

    /// Access security/approval endpoints.
    pub fn security_service(&self) -> SecurityService {
        SecurityService::new(Arc::clone(&self.ctx))
    }

    /// Access Bitcoin-specific endpoints.
    pub fn bitcoin_service(&self) -> BitcoinService {
        BitcoinService::new(Arc::clone(&self.ctx))
    }

    /// Access cross-chain endpoints.
    pub fn all_chains_service(&self) -> AllChainsService {
        AllChainsService::new(Arc::clone(&self.ctx))
    }
}
