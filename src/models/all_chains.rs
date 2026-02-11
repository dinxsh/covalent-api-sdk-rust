use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MultiChainTransactionItem {
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub tx_hash: Option<String>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub value: Option<String>,
    pub value_quote: Option<f64>,
    pub block_signed_at: Option<String>,
    pub block_height: Option<u64>,
    pub successful: Option<bool>,
    pub gas_spent: Option<u64>,
    pub gas_quote: Option<f64>,
    pub fees_paid: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MultiChainTransactionsData {
    pub updated_at: Option<String>,
    pub items: Vec<MultiChainTransactionItem>,
}

pub type MultiChainTransactionsResponse = crate::models::ApiResponse<MultiChainTransactionsData>;

#[derive(Debug, Clone, Deserialize)]
pub struct MultiChainBalanceItem {
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub contract_address: Option<String>,
    pub contract_name: Option<String>,
    pub contract_ticker_symbol: Option<String>,
    pub contract_decimals: Option<u32>,
    pub balance: Option<String>,
    pub quote: Option<f64>,
    pub quote_rate: Option<f64>,
    pub logo_url: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MultiChainBalancesData {
    pub updated_at: Option<String>,
    pub address: Option<String>,
    pub items: Vec<MultiChainBalanceItem>,
}

pub type MultiChainBalancesResponse = crate::models::ApiResponse<MultiChainBalancesData>;
