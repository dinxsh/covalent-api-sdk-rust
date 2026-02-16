//! # GoldRush SDK
//!
//! A Rust client library for the GoldRush blockchain data APIs.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use goldrush_sdk::{GoldRushClient, ClientConfig, Chain};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = GoldRushClient::new("cqt_your_api_key_here", ClientConfig::default())?;
//!
//!     let balances = client
//!         .balance_service()
//!         .get_token_balances_for_wallet_address(Chain::EthereumMainnet, "0x123...", None)
//!         .await?;
//!
//!     println!("{:?}", balances);
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod http;
pub mod models;
pub mod services;

/// Comprehensive Chain enum for all GoldRush-supported blockchain networks.
pub mod chains;

/// Shared types (QuoteCurrency, GasEventType, re-exports Chain).
pub mod types;

/// Streaming module for WebSocket-based real-time data subscriptions.
#[cfg(feature = "streaming")]
pub mod streaming;

// Production readiness modules
mod tracing;
mod rate_limit;
mod cache;
mod validation;
mod metrics;
mod circuit_breaker;
mod security;

// Core exports
pub use client::{GoldRushClient, ClientConfig};
pub use error::{Error, Result};
pub use chains::Chain;
pub use types::{QuoteCurrency, GasEventType};

// Service exports
pub use services::balance_service::{BalancesOptions, PortfolioOptions, Erc20TransfersOptions, TokenHoldersOptions, HistoricalBalancesOptions, NativeBalanceOptions};
pub use services::transaction_service::{TxOptions, SingleTxOptions, TransactionSummaryOptions, TimeBucketOptions};
pub use services::nft_service::NftOptions;
pub use services::base_service::{BlockHeightsOptions, LogEventsByAddressOptions, LogEventsByTopicOptions};
pub use services::pricing_service::PricingOptions;
pub use services::all_chains_service::{MultiChainTxOptions, MultiChainBalancesOptions};
pub use services::{BalanceService, TransactionService, NftService, BaseService, PricingService, SecurityService, BitcoinService, AllChainsService};

#[cfg(feature = "streaming")]
pub use services::StreamingService;

// Production readiness exports
pub use tracing::{RequestId, TracingContext};
pub use rate_limit::{RateLimitConfig, RateLimiter};
pub use cache::{CacheConfig, CacheStats, MemoryCache};
pub use validation::{Validator, Sanitizer};
pub use metrics::{MetricsCollector, MetricsSummary, EndpointStats, Timer};
pub use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitBreakerStats, CircuitBreakerExecutor, CircuitState};
pub use security::{SecurityConfig, SecurityManager, SecurityContext};

// Model exports
pub use models::{
    ApiResponse, Pagination, PaginationLinks,
    balances::{BalanceItem, BalancesData, BalancesResponse, Erc20TransferItem, Erc20TransfersData, Erc20TransfersResponse, TokenHolderItem, TokenHoldersData, TokenHoldersResponse, HistoricalBalanceItem, HistoricalBalancesData, HistoricalBalancesResponse, NativeTokenBalanceData, NativeTokenBalanceResponse},
    transactions::{TransactionItem, TransactionsData, TransactionsResponse, TransactionResponse, TransactionSummaryData, TransactionSummaryResponse, TimeBucketData, TimeBucketResponse},
    nfts::{NftItem, NftsData, NftsResponse, NftMetadataItem, NftMetadataResponse, ChainCollectionsResponse, NftTransactionsResponse, TraitsResponse, AttributesResponse, TraitsSummaryResponse, FloorPricesResponse, VolumeResponse, SalesCountResponse, OwnershipCheckResponse},
    base::{BlockResponse, ResolvedAddressResponse, BlockHeightsResponse, LogsResponse, AllChainsResponse, AllChainStatusResponse, AddressActivityResponse, GasPricesResponse},
    pricing::{TokenPricesResponse, PoolSpotPricesResponse},
    approvals::{ApprovalsResponse, NftApprovalsResponse},
    bitcoin::{BtcHdWalletResponse, BtcTransactionsResponse},
    all_chains::{MultiChainTransactionsResponse, MultiChainBalancesResponse},
};
