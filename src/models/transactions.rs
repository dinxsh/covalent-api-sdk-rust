use serde::Deserialize;

/// Represents a transaction item returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct TransactionItem {
    /// The transaction hash.
    pub tx_hash: String,
    
    /// The sender address.
    pub from_address: String,
    
    /// The recipient address.
    pub to_address: Option<String>,
    
    /// The transaction value as a string.
    pub value: String,
    
    /// Whether the transaction was successful.
    pub successful: Option<bool>,
    
    /// Block height where this transaction was included.
    pub block_height: Option<u64>,
    
    /// Block hash where this transaction was included.
    pub block_hash: Option<String>,
    
    /// Timestamp when the transaction was mined.
    pub block_signed_at: Option<String>,
    
    /// Gas price used for the transaction.
    pub gas_price: Option<u64>,
    
    /// Gas limit set for the transaction.
    pub gas_limit: Option<u64>,
    
    /// Gas used by the transaction.
    pub gas_used: Option<u64>,
    
    /// Transaction fee paid.
    pub fees_paid: Option<String>,
    
    /// Quote value of the transaction.
    pub value_quote: Option<f64>,
    
    /// Quote value of the gas fees.
    pub gas_quote: Option<f64>,
    
    /// Quote currency used for calculations.
    pub gas_quote_rate: Option<f64>,
    
    /// Log events associated with this transaction.
    pub log_events: Option<Vec<LogEvent>>,
}

/// Represents a log event in a transaction.
#[derive(Debug, Clone, Deserialize)]
pub struct LogEvent {
    /// The contract address that emitted this log.
    pub sender_contract_address: String,
    
    /// The log topic hash.
    pub sender_contract_ticker_symbol: Option<String>,
    
    /// The raw log data.
    pub raw_log_data: Option<String>,
    
    /// Decoded log parameters.
    pub decoded: Option<serde_json::Value>,
}

/// Container for transaction items.
#[derive(Debug, Clone, Deserialize)]
pub struct TransactionsData {
    /// The address these transactions belong to.
    pub address: Option<String>,
    
    /// The chain ID.
    pub chain_id: Option<u64>,
    
    /// The chain name.
    pub chain_name: Option<String>,
    
    /// List of transaction items.
    pub items: Vec<TransactionItem>,
    
    /// Quote currency used for calculations.
    pub quote_currency: Option<String>,
}

/// Response structure for transaction list queries.
pub type TransactionsResponse = crate::models::ApiResponse<TransactionsData>;

/// Response structure for single transaction queries.
pub type TransactionResponse = crate::models::ApiResponse<TransactionItem>;