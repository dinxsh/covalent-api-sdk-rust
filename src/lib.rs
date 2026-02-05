//! # GoldRush SDK
//!
//! A Rust client library for the GoldRush blockchain data APIs.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use goldrush_sdk::{GoldRushClient, ClientConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = GoldRushClient::new("your-api-key", ClientConfig::default())?;
//!     
//!     let balances = client
//!         .get_token_balances_for_wallet_address("eth-mainnet", "0x123...", None)
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

pub use client::{GoldRushClient, ClientConfig};
pub use error::Error;
pub use balances::BalancesOptions;
pub use transactions::TxOptions;
pub use nfts::NftOptions;

pub use models::{
    balances::{BalanceItem, BalancesData, BalancesResponse},
    transactions::{TransactionItem, TransactionsData, TransactionsResponse, TransactionResponse},
    nfts::{NftItem, NftsData, NftsResponse, NftMetadataItem, NftMetadataResponse},
};