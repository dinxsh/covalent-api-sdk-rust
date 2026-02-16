//! Streaming API Models
//!
//! Request and response types for all streaming endpoints, matching TypeScript SDK exactly.

use serde::{Deserialize, Serialize};

/// Supported blockchain networks for streaming
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamingChain {
    BaseMainnet,
    SolanaMainnet,
    SonicMainnet,
    EthMainnet,
    BscMainnet,
    HypercoreMainnet,
    HyperevmMainnet,
    MonadMainnet,
    PolygonMainnet,
    MegaethMainnet,
}

/// Time intervals for OHLCV data
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamingInterval {
    OneSecond,
    FiveSeconds,
    FifteenSeconds,
    OneMinute,
    FiveMinutes,
    FifteenMinutes,
    OneHour,
    FourHours,
    OneDay,
}

/// Timeframe windows for aggregation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamingTimeframe {
    OneMinute,
    FiveMinutes,
    FifteenMinutes,
    OneHour,
    FourHours,
    OneDay,
    SevenDays,
}

/// DEX protocols supported
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamingProtocol {
    UniswapV2,
    UniswapV3,
    VirtualsV2,
    #[serde(rename = "CLANKER")]
    ClankerV3,
    RaydiumAmm,
    RaydiumClmm,
    RaydiumCpmm,
    PumpDotFun,
    PumpFunAmm,
    Moonshot,
    RaydiumLaunchLab,
    MeteoraDamm,
    MeteoraDlmm,
    MeteoraDbc,
    PancakeswapV2,
    PancakeswapV3,
    ShadowV2,
    ShadowV3,
    OctoswapV2,
    OctoswapV3,
    QuickswapV2,
    QuickswapV3,
    SushiswapV2,
    ProjectX,
    KumbayaV1,
    JoeV2,
}

/// Contract/Token metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractMetadata {
    pub contract_decimals: u32,
    pub contract_name: String,
    pub contract_ticker_symbol: Option<String>,
    pub contract_address: String,
    #[serde(default)]
    pub supports_erc: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
}

// =============================================================================
// OHLCV Pairs Stream
// =============================================================================

/// Parameters for OHLCV pairs subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OhlcvPairsParams {
    pub chain_name: StreamingChain,
    pub pair_addresses: Vec<String>,
    pub interval: StreamingInterval,
    pub timeframe: StreamingTimeframe,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Response for OHLCV pairs data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OhlcvPairsResponse {
    pub chain_name: StreamingChain,
    pub pair_address: String,
    pub interval: StreamingInterval,
    pub timeframe: StreamingTimeframe,
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub volume_usd: f64,
    pub quote_rate: f64,
    pub quote_rate_usd: f64,
    pub base_token: ContractMetadata,
    pub quote_token: ContractMetadata,
}

// =============================================================================
// OHLCV Tokens Stream
// =============================================================================

/// Parameters for OHLCV tokens subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OhlcvTokensParams {
    pub chain_name: StreamingChain,
    pub token_addresses: Vec<String>,
    pub interval: StreamingInterval,
    pub timeframe: StreamingTimeframe,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Response for OHLCV tokens data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OhlcvTokensResponse {
    pub chain_name: StreamingChain,
    pub pair_address: String,
    pub interval: StreamingInterval,
    pub timeframe: StreamingTimeframe,
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub volume_usd: f64,
    pub quote_rate: f64,
    pub quote_rate_usd: f64,
    pub base_token: ContractMetadata,
    pub quote_token: ContractMetadata,
}

// =============================================================================
// New DEX Pairs Stream
// =============================================================================

/// Parameters for new DEX pairs subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPairsParams {
    pub chain_name: StreamingChain,
    pub protocols: Vec<StreamingProtocol>,
}

/// Price change metrics over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceMetrics {
    pub last_5m: f64,
    pub last_1hr: f64,
    pub last_6hr: f64,
    pub last_24hr: f64,
}

/// Swap count metrics over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapMetrics {
    pub last_5m: u32,
    pub last_1hr: u32,
    pub last_6hr: u32,
    pub last_24hr: u32,
}

/// Response for new DEX pairs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPairsResponse {
    pub chain_name: String,
    pub protocol: String,
    pub protocol_version: String,
    pub pair_address: String,
    pub deployer_address: String,
    pub tx_hash: String,
    pub block_signed_at: String,
    pub liquidity: f64,
    pub supply: f64,
    pub market_cap: f64,
    pub event_name: String,
    pub quote_rate: f64,
    pub quote_rate_usd: f64,
    pub base_token: ContractMetadata,
    pub quote_token: ContractMetadata,
    pub pair: ContractMetadata,
}

// =============================================================================
// Update Pairs Stream
// =============================================================================

/// Parameters for pair updates subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePairsParams {
    pub chain_name: StreamingChain,
    pub pair_addresses: Vec<String>,
}

/// Response for pair updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePairsResponse {
    pub chain_name: String,
    pub pair_address: String,
    pub timestamp: String,
    pub quote_rate: f64,
    pub quote_rate_usd: f64,
    pub volume: f64,
    pub volume_usd: f64,
    pub market_cap: f64,
    pub liquidity: f64,
    pub base_token: ContractMetadata,
    pub quote_token: ContractMetadata,
    pub price_deltas: PriceMetrics,
    pub swap_counts: SwapMetrics,
}

// =============================================================================
// Wallet Activity Stream
// =============================================================================

/// Parameters for wallet activity subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletActivityParams {
    pub chain_name: StreamingChain,
    pub wallet_addresses: Vec<String>,
}

/// Log item in a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletActivityLogItem {
    pub emitter_address: String,
    pub log_offset: u32,
    pub data: String,
    pub topics: Vec<String>,
}

/// Transfer transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferTransaction {
    pub from: String,
    pub to: String,
    pub amount: String,
    pub quote_usd: f64,
    pub quote_rate_usd: f64,
    pub contract_metadata: ContractMetadata,
}

/// Swap transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapTransaction {
    pub token_in: String,
    pub token_out: String,
    pub amount_in: String,
    pub amount_out: String,
}

/// Bridge transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeTransaction {
    #[serde(rename = "type")]
    pub transaction_type: String,
    #[serde(rename = "typeString")]
    pub type_string: String,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub quote_usd: f64,
    pub quote_rate_usd: f64,
    pub contract_metadata: ContractMetadata,
}

/// Deposit transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositTransaction {
    pub from: String,
    pub to: String,
    pub amount: String,
    pub quote_usd: f64,
    pub quote_rate_usd: f64,
    pub contract_metadata: ContractMetadata,
}

/// Withdraw transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawTransaction {
    pub from: String,
    pub to: String,
    pub amount: String,
    pub quote_usd: f64,
    pub quote_rate_usd: f64,
    pub contract_metadata: ContractMetadata,
}

/// Approve transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproveTransaction {
    pub spender: String,
    pub amount: String,
    pub quote_usd: f64,
    pub quote_rate_usd: f64,
    pub contract_metadata: ContractMetadata,
}

/// Error details in transaction decoding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub message: String,
}

/// Decoded transaction details (union type)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DecodedTransactionDetails {
    Transfer(TransferTransaction),
    Swap(SwapTransaction),
    Bridge(BridgeTransaction),
    Deposit(DepositTransaction),
    Withdraw(WithdrawTransaction),
    Approve(ApproveTransaction),
    Error(ErrorDetails),
}

/// Response for wallet activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletActivityResponse {
    pub tx_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub value: f64,
    pub chain_name: String,
    pub block_signed_at: String,
    pub block_height: u64,
    pub block_hash: String,
    pub miner_address: String,
    pub gas_used: u64,
    pub tx_offset: u32,
    pub successful: bool,
    pub decoded_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_details: Option<DecodedTransactionDetails>,
    pub logs: Vec<WalletActivityLogItem>,
}

// =============================================================================
// Token Search Query
// =============================================================================

/// Parameters for token search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSearchParams {
    pub query: String,
}

/// Response for token search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSearchResponse {
    pub pair_address: String,
    pub chain_name: String,
    pub quote_rate: f64,
    pub quote_rate_usd: f64,
    pub volume: f64,
    pub volume_usd: f64,
    pub market_cap: f64,
    pub base_token: ContractMetadata,
    pub quote_token: ContractMetadata,
}

// =============================================================================
// Unrealized PnL for Token Query
// =============================================================================

/// Parameters for unrealized PnL for token query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpnlForTokenParams {
    pub chain_name: StreamingChain,
    pub token_address: String,
}

/// Response for unrealized PnL for token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpnlForTokenResponse {
    pub token_address: String,
    pub wallet_address: String,
    pub volume: String,
    pub transactions_count: u32,
    pub pnl_realized_usd: f64,
    pub balance: String,
    pub balance_pretty: String,
    pub pnl_unrealized_usd: f64,
    pub contract_metadata: ContractMetadata,
}

// =============================================================================
// Unrealized PnL for Wallet Query
// =============================================================================

/// Parameters for unrealized PnL for wallet query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpnlForWalletParams {
    pub chain_name: StreamingChain,
    pub wallet_address: String,
}

/// Response for unrealized PnL for wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpnlForWalletResponse {
    pub token_address: String,
    pub cost_basis: f64,
    pub current_price: f64,
    pub pnl_realized_usd: Option<f64>,
    pub pnl_unrealized_usd: f64,
    pub net_balance_change: String,
    pub marketcap_usd: String,
    pub contract_metadata: ContractMetadata,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_serialization() {
        let chain = StreamingChain::BaseMainnet;
        let json = serde_json::to_string(&chain).unwrap();
        assert_eq!(json, r#""BASE_MAINNET"#);
    }

    #[test]
    fn test_ohlcv_params_serialization() {
        let params = OhlcvPairsParams {
            chain_name: StreamingChain::BaseMainnet,
            pair_addresses: vec!["0xabc".to_string()],
            interval: StreamingInterval::OneMinute,
            timeframe: StreamingTimeframe::OneHour,
            limit: Some(10),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["chain_name"], "BASE_MAINNET");
    }
}
