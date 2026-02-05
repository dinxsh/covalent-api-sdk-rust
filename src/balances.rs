use crate::{Error, GoldRushClient};
use crate::models::balances::BalancesResponse;
use reqwest::Method;

/// Options for customizing balance queries.
#[derive(Debug, Clone, Default)]
pub struct BalancesOptions {
    /// Quote currency for pricing (e.g., "USD", "ETH").
    pub quote_currency: Option<String>,
    
    /// Whether to include NFT balances.
    pub nft: Option<bool>,
    
    /// Whether to include spam tokens.
    pub no_spam: Option<bool>,
    
    /// Page number for pagination (0-indexed).
    pub page_number: Option<u32>,
    
    /// Number of items per page.
    pub page_size: Option<u32>,
}

impl BalancesOptions {
    /// Create new default options.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the quote currency.
    pub fn quote_currency<S: Into<String>>(mut self, currency: S) -> Self {
        self.quote_currency = Some(currency.into());
        self
    }
    
    /// Include or exclude NFT balances.
    pub fn nft(mut self, include_nft: bool) -> Self {
        self.nft = Some(include_nft);
        self
    }
    
    /// Exclude spam tokens.
    pub fn no_spam(mut self, exclude_spam: bool) -> Self {
        self.no_spam = Some(exclude_spam);
        self
    }
    
    /// Set page number for pagination.
    pub fn page_number(mut self, page: u32) -> Self {
        self.page_number = Some(page);
        self
    }
    
    /// Set page size.
    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size);
        self
    }
}

impl GoldRushClient {
    /// Get token balances for a wallet address.
    ///
    /// # Arguments
    ///
    /// * `chain_name` - The blockchain name (e.g., "eth-mainnet", "matic-mainnet")
    /// * `address` - The wallet address to query
    /// * `options` - Optional query parameters
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use goldrush_sdk::{GoldRushClient, BalancesOptions};
    ///
    /// # async fn example(client: GoldRushClient) -> Result<(), goldrush_sdk::Error> {
    /// let options = BalancesOptions::new()
    ///     .quote_currency("USD")
    ///     .no_spam(true);
    ///     
    /// let balances = client
    ///     .get_token_balances_for_wallet_address(
    ///         "eth-mainnet",
    ///         "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
    ///         Some(options)
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_token_balances_for_wallet_address(
        &self,
        chain_name: &str,
        address: &str,
        options: Option<BalancesOptions>,
    ) -> Result<BalancesResponse, Error> {
        // Verified endpoint path from GoldRush documentation
        let path = format!("/v1/{}/address/{}/balances_v2/", chain_name, address);
        
        let mut builder = self.build_request(Method::GET, &path);
        
        // Add query parameters if options are provided
        if let Some(opts) = options {
            if let Some(currency) = opts.quote_currency {
                builder = builder.query(&[("quote-currency", currency)]);
            }
            if let Some(include_nft) = opts.nft {
                builder = builder.query(&[("nft", include_nft.to_string())]);
            }
            if let Some(no_spam) = opts.no_spam {
                builder = builder.query(&[("no-spam", no_spam.to_string())]);
            }
            if let Some(page_num) = opts.page_number {
                builder = builder.query(&[("page-number", page_num.to_string())]);
            }
            if let Some(page_sz) = opts.page_size {
                builder = builder.query(&[("page-size", page_sz.to_string())]);
            }
        }
        
        self.send_with_retry(builder).await
    }
    
    /// Get historical portfolio balances for an address.
    ///
    /// # Arguments
    ///
    /// * `chain_name` - The blockchain name
    /// * `address` - The wallet address to query
    /// * `options` - Optional query parameters
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use goldrush_sdk::GoldRushClient;
    ///
    /// # async fn example(client: GoldRushClient) -> Result<(), goldrush_sdk::Error> {
    /// let portfolio = client
    ///     .get_historical_portfolio_for_wallet_address(
    ///         "eth-mainnet",
    ///         "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
    ///         None
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_historical_portfolio_for_wallet_address(
        &self,
        chain_name: &str,
        address: &str,
        options: Option<BalancesOptions>,
    ) -> Result<BalancesResponse, Error> {
        // Verified endpoint path from GoldRush documentation
        let path = format!("/v1/{}/address/{}/portfolio_v2/", chain_name, address);
        
        let mut builder = self.build_request(Method::GET, &path);
        
        if let Some(opts) = options {
            if let Some(currency) = opts.quote_currency {
                builder = builder.query(&[("quote-currency", currency)]);
            }
            if let Some(page_num) = opts.page_number {
                builder = builder.query(&[("page-number", page_num.to_string())]);
            }
            if let Some(page_sz) = opts.page_size {
                builder = builder.query(&[("page-size", page_sz.to_string())]);
            }
        }
        
        self.send_with_retry(builder).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_balances_options_builder() {
        let options = BalancesOptions::new()
            .quote_currency("USD")
            .nft(true)
            .no_spam(true)
            .page_size(50);
            
        assert_eq!(options.quote_currency, Some("USD".to_string()));
        assert_eq!(options.nft, Some(true));
        assert_eq!(options.no_spam, Some(true));
        assert_eq!(options.page_size, Some(50));
    }

    #[test]
    fn test_deserialize_balances_response() {
        let json_data = json!({
            "data": {
                "address": "0x123",
                "chain_id": 1,
                "items": [{
                    "contract_address": "0xA0b86a33E6441e6b32f6aDaa51a3FC6F1b6a3B9a",
                    "contract_ticker_symbol": "COVALENT",
                    "contract_name": "Covalent Query Token",
                    "balance": "1000000000000000000",
                    "quote_rate": 1.23,
                    "quote": 1.23,
                    "token_type": "cryptocurrency",
                    "is_spam": false,
                    "contract_decimals": 18
                }]
            },
            "error": null,
            "pagination": {
                "has_more": false,
                "page_number": 0,
                "page_size": 100,
                "total_count": 1
            }
        });

        let response: BalancesResponse = serde_json::from_value(json_data).unwrap();
        assert!(response.data.is_some());
        
        let data = response.data.unwrap();
        assert_eq!(data.items.len(), 1);
        assert_eq!(data.items[0].contract_ticker_symbol, Some("COVALENT".to_string()));
        assert_eq!(data.items[0].balance, "1000000000000000000");
    }
}