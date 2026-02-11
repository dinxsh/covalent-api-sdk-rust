use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ApprovalItem {
    pub token_address: Option<String>,
    pub token_address_label: Option<String>,
    pub ticker_symbol: Option<String>,
    pub contract_decimals: Option<u32>,
    pub logo_url: Option<String>,
    pub quote_rate: Option<f64>,
    pub balance: Option<String>,
    pub balance_quote: Option<f64>,
    pub pretty_balance_quote: Option<String>,
    pub value_at_risk: Option<String>,
    pub value_at_risk_quote: Option<f64>,
    pub pretty_value_at_risk_quote: Option<String>,
    pub spenders: Option<Vec<SpenderItem>>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpenderItem {
    pub block_height: Option<u64>,
    pub tx_hash: Option<String>,
    pub tx_offset: Option<u64>,
    pub spender_address: Option<String>,
    pub spender_address_label: Option<String>,
    pub allowance: Option<String>,
    pub pretty_allowance: Option<String>,
    pub value_at_risk: Option<String>,
    pub risk_factor: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApprovalsData {
    pub updated_at: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub address: Option<String>,
    pub items: Vec<ApprovalItem>,
}

pub type ApprovalsResponse = crate::models::ApiResponse<ApprovalsData>;

#[derive(Debug, Clone, Deserialize)]
pub struct NftApprovalItem {
    pub contract_address: Option<String>,
    pub contract_address_label: Option<String>,
    pub ticker_symbol: Option<String>,
    pub contract_name: Option<String>,
    pub logo_url: Option<String>,
    pub token_id: Option<String>,
    pub token_balance: Option<String>,
    pub spenders: Option<Vec<NftSpenderItem>>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NftSpenderItem {
    pub block_height: Option<u64>,
    pub tx_hash: Option<String>,
    pub spender_address: Option<String>,
    pub spender_address_label: Option<String>,
    pub allowance: Option<String>,
    pub token_id: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NftApprovalsData {
    pub updated_at: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub address: Option<String>,
    pub items: Vec<NftApprovalItem>,
}

pub type NftApprovalsResponse = crate::models::ApiResponse<NftApprovalsData>;
