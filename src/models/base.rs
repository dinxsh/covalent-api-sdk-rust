use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BlockData {
    pub updated_at: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<BlockItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlockItem {
    pub signed_at: Option<String>,
    pub height: Option<u64>,
    pub block_hash: Option<String>,
    pub miner_address: Option<String>,
    pub gas_used: Option<u64>,
    pub gas_limit: Option<u64>,
    pub extra_data: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

pub type BlockResponse = crate::models::ApiResponse<BlockData>;

#[derive(Debug, Clone, Deserialize)]
pub struct ResolvedAddressData {
    pub address: Option<String>,
    pub name: Option<String>,
}

pub type ResolvedAddressResponse = crate::models::ApiResponse<ResolvedAddressData>;

#[derive(Debug, Clone, Deserialize)]
pub struct BlockHeightsData {
    pub updated_at: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<BlockHeightItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlockHeightItem {
    pub signed_at: Option<String>,
    pub height: Option<u64>,
}

pub type BlockHeightsResponse = crate::models::ApiResponse<BlockHeightsData>;

#[derive(Debug, Clone, Deserialize)]
pub struct LogsData {
    pub updated_at: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<LogEventItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogEventItem {
    pub block_signed_at: Option<String>,
    pub block_height: Option<u64>,
    pub tx_offset: Option<u64>,
    pub log_offset: Option<u64>,
    pub tx_hash: Option<String>,
    pub raw_log_topics: Option<Vec<String>>,
    pub sender_contract_decimals: Option<u32>,
    pub sender_name: Option<String>,
    pub sender_contract_ticker_symbol: Option<String>,
    pub sender_address: Option<String>,
    pub sender_address_label: Option<String>,
    pub sender_factory_address: Option<String>,
    pub raw_log_data: Option<String>,
    pub decoded: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

pub type LogsResponse = crate::models::ApiResponse<LogsData>;

#[derive(Debug, Clone, Deserialize)]
pub struct ChainItem {
    pub name: Option<String>,
    pub chain_id: Option<String>,
    pub is_testnet: Option<bool>,
    pub db_schema_name: Option<String>,
    pub label: Option<String>,
    pub category_label: Option<String>,
    pub logo_url: Option<String>,
    pub black_logo_url: Option<String>,
    pub white_logo_url: Option<String>,
    pub color_theme: Option<serde_json::Value>,
    pub is_appchain: Option<bool>,
    pub appchain_of: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

pub type AllChainsResponse = crate::models::ApiResponse<AllChainsData>;

#[derive(Debug, Clone, Deserialize)]
pub struct AllChainsData {
    pub updated_at: Option<String>,
    pub items: Vec<ChainItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChainStatusItem {
    pub name: Option<String>,
    pub chain_id: Option<String>,
    pub is_testnet: Option<bool>,
    pub logo_url: Option<String>,
    pub synced_block_height: Option<u64>,
    pub synced_blocked_signed_at: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AllChainStatusData {
    pub updated_at: Option<String>,
    pub items: Vec<ChainStatusItem>,
}

pub type AllChainStatusResponse = crate::models::ApiResponse<AllChainStatusData>;

#[derive(Debug, Clone, Deserialize)]
pub struct AddressActivityItem {
    pub chain_id: Option<String>,
    pub chain_name: Option<String>,
    pub first_seen_at: Option<String>,
    pub last_seen_at: Option<String>,
    pub is_testnet: Option<bool>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddressActivityData {
    pub updated_at: Option<String>,
    pub address: Option<String>,
    pub items: Vec<AddressActivityItem>,
}

pub type AddressActivityResponse = crate::models::ApiResponse<AddressActivityData>;

#[derive(Debug, Clone, Deserialize)]
pub struct GasPriceItem {
    pub event_type: Option<String>,
    pub gas_quote_rate: Option<f64>,
    pub gas_price_gwei: Option<f64>,
    pub gas_price_wei: Option<String>,
    pub interval: Option<String>,
    pub pretty_total_gas_quote: Option<String>,
    pub total_gas_quote: Option<f64>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GasPricesData {
    pub updated_at: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<GasPriceItem>,
}

pub type GasPricesResponse = crate::models::ApiResponse<GasPricesData>;
