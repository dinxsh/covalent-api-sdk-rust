use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BtcHdWalletBalance {
    pub total_balance: Option<String>,
    pub total_receive: Option<String>,
    pub total_spend: Option<String>,
    pub hd_wallet_address: Option<String>,
    pub address: Option<String>,
    pub offset: Option<u64>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BtcHdWalletData {
    pub updated_at: Option<String>,
    pub address: Option<String>,
    pub items: Vec<BtcHdWalletBalance>,
}

pub type BtcHdWalletResponse = crate::models::ApiResponse<BtcHdWalletData>;

#[derive(Debug, Clone, Deserialize)]
pub struct BtcTransactionItem {
    pub block_signed_at: Option<String>,
    pub block_height: Option<u64>,
    pub tx_hash: Option<String>,
    pub successful: Option<bool>,
    pub fees_paid: Option<String>,
    pub value: Option<String>,
    pub value_quote: Option<f64>,
    pub gas_quote: Option<f64>,
    pub inputs: Option<Vec<BtcTxInput>>,
    pub outputs: Option<Vec<BtcTxOutput>>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BtcTxInput {
    pub prev_hash: Option<String>,
    pub output_index: Option<u64>,
    pub script: Option<String>,
    pub output_value: Option<u64>,
    pub sequence: Option<u64>,
    pub addresses: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BtcTxOutput {
    pub value: Option<u64>,
    pub script: Option<String>,
    pub addresses: Option<Vec<String>>,
    pub script_type: Option<String>,
    pub spent_by: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BtcTransactionsData {
    pub updated_at: Option<String>,
    pub address: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<BtcTransactionItem>,
    pub quote_currency: Option<String>,
}

pub type BtcTransactionsResponse = crate::models::ApiResponse<BtcTransactionsData>;
