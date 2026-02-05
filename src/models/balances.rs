use serde::Deserialize;

/// Represents a token balance item returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct BalanceItem {
    /// The contract address of the token.
    pub contract_address: String,
    
    /// The token symbol/ticker.
    #[serde(rename = "contract_ticker_symbol")]
    pub contract_ticker_symbol: Option<String>,
    
    /// The token name.
    #[serde(rename = "contract_name")]
    pub contract_name: Option<String>,
    
    /// The raw balance amount as a string (to handle large numbers).
    pub balance: String,
    
    /// The number of decimal places for this token.
    #[serde(rename = "contract_decimals")]
    pub contract_decimals: Option<u32>,
    
    /// The current quote rate for the token.
    pub quote_rate: Option<f64>,
    
    /// The quote value (balance * quote_rate).
    pub quote: Option<f64>,
    
    /// The type of token (e.g., "cryptocurrency", "stablecoin", etc.).
    #[serde(rename = "type")]
    pub token_type: Option<String>,
    
    /// Whether this token is spam.
    pub is_spam: Option<bool>,
    
    /// The logo URL for the token.
    pub logo_url: Option<String>,
    
    /// Last transferred timestamp.
    pub last_transferred_at: Option<String>,
    
    /// Whether this token is native to the chain.
    pub native_token: Option<bool>,
    
    /// Additional metadata.
    #[serde(flatten)]
    pub metadata: Option<serde_json::Value>,
}

/// Container for balance items.
#[derive(Debug, Clone, Deserialize)]
pub struct BalancesData {
    /// The address these balances belong to.
    pub address: Option<String>,
    
    /// The chain ID.
    pub chain_id: Option<u64>,
    
    /// The chain name.
    pub chain_name: Option<String>,
    
    /// List of token balance items.
    pub items: Vec<BalanceItem>,
    
    /// Quote currency used for calculations.
    pub quote_currency: Option<String>,
    
    /// Total quote value across all tokens.
    pub total_quote: Option<f64>,
}

/// Response structure for balance queries.
pub type BalancesResponse = crate::models::ApiResponse<BalancesData>;