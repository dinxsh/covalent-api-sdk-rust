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

impl BalanceItem {
    /// Get the display symbol for this token.
    pub fn symbol(&self) -> &str {
        self.contract_ticker_symbol.as_deref().unwrap_or("Unknown")
    }

    /// Get the display name for this token.
    pub fn name(&self) -> &str {
        self.contract_name.as_deref()
            .or(self.contract_ticker_symbol.as_deref())
            .unwrap_or("Unknown")
    }

    /// Parse the balance as a floating point number, accounting for decimals.
    pub fn balance_as_float(&self) -> Option<f64> {
        let balance = self.balance.parse::<f64>().ok()?;
        let decimals = self.contract_decimals.unwrap_or(18);
        Some(balance / 10f64.powi(decimals as i32))
    }

    /// Check if this token has a non-zero balance.
    pub fn has_balance(&self) -> bool {
        self.balance.parse::<f64>().unwrap_or(0.0) > 0.0
    }

    /// Check if this token has quote value information.
    pub fn has_quote_value(&self) -> bool {
        self.quote.unwrap_or(0.0) > 0.0
    }

    /// Check if this token is marked as spam.
    pub fn is_spam(&self) -> bool {
        self.is_spam.unwrap_or(false)
    }
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

impl BalancesData {
    /// Calculate the total portfolio value from all balance items.
    pub fn total_value(&self) -> f64 {
        self.items.iter()
            .filter_map(|item| item.quote)
            .sum()
    }

    /// Get tokens with non-zero quote value, sorted by value (highest first).
    pub fn tokens_by_value(&self) -> Vec<&BalanceItem> {
        let mut tokens: Vec<_> = self.items.iter()
            .filter(|item| item.quote.unwrap_or(0.0) > 0.0)
            .collect();
        tokens.sort_by(|a, b|
            b.quote.partial_cmp(&a.quote).unwrap_or(std::cmp::Ordering::Equal)
        );
        tokens
    }

    /// Filter tokens by minimum quote value.
    pub fn tokens_above_value(&self, min_value: f64) -> Vec<&BalanceItem> {
        self.items.iter()
            .filter(|item| item.quote.unwrap_or(0.0) >= min_value)
            .collect()
    }

    /// Get count of non-spam tokens.
    pub fn non_spam_count(&self) -> usize {
        self.items.iter()
            .filter(|item| !item.is_spam.unwrap_or(false))
            .count()
    }
}

/// Response structure for balance queries.
pub type BalancesResponse = crate::models::ApiResponse<BalancesData>;

// --- Extended models for additional balance endpoints ---

/// Represents an ERC20 token transfer item.
#[derive(Debug, Clone, Deserialize)]
pub struct Erc20TransferItem {
    pub block_signed_at: Option<String>,
    pub block_height: Option<u64>,
    pub tx_hash: Option<String>,
    pub from_address: Option<String>,
    pub from_address_label: Option<String>,
    pub to_address: Option<String>,
    pub to_address_label: Option<String>,
    pub contract_decimals: Option<u32>,
    pub contract_name: Option<String>,
    pub contract_ticker_symbol: Option<String>,
    pub contract_address: Option<String>,
    pub logo_url: Option<String>,
    pub transfer_type: Option<String>,
    pub delta: Option<String>,
    pub balance: Option<String>,
    pub quote_rate: Option<f64>,
    pub delta_quote: Option<f64>,
    pub balance_quote: Option<f64>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Container for ERC20 transfer items.
#[derive(Debug, Clone, Deserialize)]
pub struct Erc20TransfersData {
    pub address: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<Erc20TransferItem>,
    pub quote_currency: Option<String>,
}

/// Response structure for ERC20 transfer queries.
pub type Erc20TransfersResponse = crate::models::ApiResponse<Erc20TransfersData>;

/// Represents a token holder item.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenHolderItem {
    pub address: Option<String>,
    pub balance: Option<String>,
    pub total_supply: Option<String>,
    pub block_height: Option<u64>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Container for token holder items.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenHoldersData {
    pub updated_at: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<TokenHolderItem>,
}

/// Response structure for token holder queries.
pub type TokenHoldersResponse = crate::models::ApiResponse<TokenHoldersData>;

/// Represents a historical balance item.
#[derive(Debug, Clone, Deserialize)]
pub struct HistoricalBalanceItem {
    pub contract_address: Option<String>,
    pub contract_name: Option<String>,
    pub contract_ticker_symbol: Option<String>,
    pub contract_decimals: Option<u32>,
    pub logo_url: Option<String>,
    pub balance: Option<String>,
    pub quote: Option<f64>,
    pub quote_rate: Option<f64>,
    pub block_height: Option<u64>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Container for historical balance items.
#[derive(Debug, Clone, Deserialize)]
pub struct HistoricalBalancesData {
    pub address: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<HistoricalBalanceItem>,
    pub quote_currency: Option<String>,
}

/// Response structure for historical balance queries.
pub type HistoricalBalancesResponse = crate::models::ApiResponse<HistoricalBalancesData>;

/// Container for native token balance data.
#[derive(Debug, Clone, Deserialize)]
pub struct NativeTokenBalanceData {
    pub address: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<BalanceItem>,
    pub quote_currency: Option<String>,
}

/// Response structure for native token balance queries.
pub type NativeTokenBalanceResponse = crate::models::ApiResponse<NativeTokenBalanceData>;
