use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TokenPriceItem {
    pub contract_decimals: Option<u32>,
    pub contract_name: Option<String>,
    pub contract_ticker_symbol: Option<String>,
    pub contract_address: Option<String>,
    pub supports_erc: Option<Vec<String>>,
    pub logo_url: Option<String>,
    pub quote_currency: Option<String>,
    pub prices: Option<Vec<PricePoint>>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PricePoint {
    pub date: Option<String>,
    pub price: Option<f64>,
    pub pretty_price: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenPricesData {
    pub updated_at: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<TokenPriceItem>,
}

pub type TokenPricesResponse = crate::models::ApiResponse<Vec<TokenPriceItem>>;

#[derive(Debug, Clone, Deserialize)]
pub struct PoolSpotPriceItem {
    pub exchange: Option<String>,
    pub swap_count_24h: Option<u64>,
    pub total_liquidity_quote: Option<f64>,
    pub volume_24h_quote: Option<f64>,
    pub fee_24h_quote: Option<f64>,
    pub token_0: Option<PoolToken>,
    pub token_1: Option<PoolToken>,
    pub quote_currency: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PoolToken {
    pub contract_address: Option<String>,
    pub contract_name: Option<String>,
    pub contract_ticker_symbol: Option<String>,
    pub contract_decimals: Option<u32>,
    pub logo_url: Option<String>,
    pub quote_rate: Option<f64>,
    pub reserve: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PoolSpotPricesData {
    pub updated_at: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<PoolSpotPriceItem>,
}

pub type PoolSpotPricesResponse = crate::models::ApiResponse<PoolSpotPricesData>;
