//! # GoldRush SDK
//!
//! A Rust client library for the GoldRush blockchain data APIs.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use goldrush_sdk::{GoldRushClient, ClientConfig, chains};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = GoldRushClient::new("your-api-key", ClientConfig::default())?;
//!     
//!     let balances = client
//!         .get_token_balances_for_wallet_address(chains::ethereum::MAINNET, "0x123...", None)
//!         .await?;
//!     
//!     println!("{:?}", balances);
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod http;
mod models;
mod balances;
mod transactions;
mod nfts;

/// Chain constants for commonly used networks
pub mod chains;

// Production readiness modules
mod tracing;
mod rate_limit;
mod cache;
mod validation;
mod metrics;
mod circuit_breaker;
mod security;

pub use client::{GoldRushClient, ClientConfig};
pub use error::{Error, Result};
pub use balances::BalancesOptions;
pub use transactions::{TxOptions, TransactionsPageIter};
pub use nfts::{NftOptions, NftsPageIter};

// Production readiness exports
pub use tracing::{RequestId, TracingContext};
pub use rate_limit::{RateLimitConfig, RateLimiter};
pub use cache::{CacheConfig, CacheStats, MemoryCache};
pub use validation::{Validator, Sanitizer};
pub use metrics::{MetricsCollector, MetricsSummary, EndpointStats, Timer};
pub use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitBreakerStats, CircuitBreakerExecutor, CircuitState};
pub use security::{SecurityConfig, SecurityManager, SecurityContext};

pub use models::{
    balances::{BalanceItem, BalancesData, BalancesResponse},
    transactions::{TransactionItem, TransactionsData, TransactionsResponse, TransactionResponse},
    nfts::{NftItem, NftsData, NftsResponse, NftMetadataItem, NftMetadataResponse},
};