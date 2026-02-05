use crate::{Error, GoldRushClient};
use crate::models::transactions::{TransactionsResponse, TransactionResponse};
use reqwest::Method;

/// Options for customizing transaction queries.
#[derive(Debug, Clone, Default)]
pub struct TxOptions {
    /// Page number for pagination (0-indexed).
    pub page_number: Option<u32>,
    
    /// Number of items per page.
    pub page_size: Option<u32>,
    
    /// Quote currency for pricing (e.g., "USD", "ETH").
    pub quote_currency: Option<String>,
    
    /// Include or exclude log events.
    pub with_log_events: Option<bool>,
    
    /// Start block height for filtering.
    pub starting_block: Option<u64>,
    
    /// End block height for filtering.
    pub ending_block: Option<u64>,
}

impl TxOptions {
    /// Create new default options.
    pub fn new() -> Self {
        Self::default()
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
    
    /// Set the quote currency.
    pub fn quote_currency<S: Into<String>>(mut self, currency: S) -> Self {
        self.quote_currency = Some(currency.into());
        self
    }
    
    /// Include or exclude log events in the response.
    pub fn with_log_events(mut self, include_logs: bool) -> Self {
        self.with_log_events = Some(include_logs);
        self
    }
    
    /// Set starting block for filtering.
    pub fn starting_block(mut self, block: u64) -> Self {
        self.starting_block = Some(block);
        self
    }
    
    /// Set ending block for filtering.
    pub fn ending_block(mut self, block: u64) -> Self {
        self.ending_block = Some(block);
        self
    }
}

impl GoldRushClient {
    /// Get all transactions for an address.
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
    /// use goldrush_sdk::{GoldRushClient, TxOptions};
    ///
    /// # async fn example(client: GoldRushClient) -> Result<(), goldrush_sdk::Error> {
    /// let options = TxOptions::new()
    ///     .page_size(10)
    ///     .quote_currency("USD");
    ///     
    /// let transactions = client
    ///     .get_all_transactions_for_address(
    ///         "eth-mainnet",
    ///         "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
    ///         Some(options)
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_all_transactions_for_address(
        &self,
        chain_name: &str,
        address: &str,
        options: Option<TxOptions>,
    ) -> Result<TransactionsResponse, Error> {
        // Verified endpoint path from GoldRush documentation
        let path = format!("/v1/{}/address/{}/transactions_v2/", chain_name, address);
        
        let mut builder = self.build_request(Method::GET, &path);
        
        // Add query parameters if options are provided
        if let Some(opts) = options {
            if let Some(page_num) = opts.page_number {
                builder = builder.query(&[("page-number", page_num.to_string())]);
            }
            if let Some(page_sz) = opts.page_size {
                builder = builder.query(&[("page-size", page_sz.to_string())]);
            }
            if let Some(currency) = opts.quote_currency {
                builder = builder.query(&[("quote-currency", currency)]);
            }
            if let Some(with_logs) = opts.with_log_events {
                builder = builder.query(&[("with-log-events", with_logs.to_string())]);
            }
            if let Some(start_block) = opts.starting_block {
                builder = builder.query(&[("starting-block", start_block.to_string())]);
            }
            if let Some(end_block) = opts.ending_block {
                builder = builder.query(&[("ending-block", end_block.to_string())]);
            }
        }
        
        self.send_with_retry(builder).await
    }

    /// Get a specific transaction by hash.
    ///
    /// # Arguments
    ///
    /// * `chain_name` - The blockchain name
    /// * `tx_hash` - The transaction hash
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use goldrush_sdk::GoldRushClient;
    ///
    /// # async fn example(client: GoldRushClient) -> Result<(), goldrush_sdk::Error> {
    /// let transaction = client
    ///     .get_transaction(
    ///         "eth-mainnet",
    ///         "0x1234567890abcdef..."
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_transaction(
        &self,
        chain_name: &str,
        tx_hash: &str,
    ) -> Result<TransactionResponse, Error> {
        // Verified endpoint path from GoldRush documentation
        let path = format!("/v1/{}/transaction_v2/{}/", chain_name, tx_hash);
        
        let builder = self.build_request(Method::GET, &path);
        self.send_with_retry(builder).await
    }
    
    /// Get transactions between two addresses.
    ///
    /// # Arguments
    ///
    /// * `chain_name` - The blockchain name
    /// * `from_address` - The sender address
    /// * `to_address` - The recipient address
    /// * `options` - Optional query parameters
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use goldrush_sdk::GoldRushClient;
    ///
    /// # async fn example(client: GoldRushClient) -> Result<(), goldrush_sdk::Error> {
    /// let transactions = client
    ///     .get_transactions_between_addresses(
    ///         "eth-mainnet",
    ///         "0xfrom...",
    ///         "0xto...",
    ///         None
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_transactions_between_addresses(
        &self,
        chain_name: &str,
        from_address: &str,
        to_address: &str,
        options: Option<TxOptions>,
    ) -> Result<TransactionsResponse, Error> {
        // Verified endpoint path from GoldRush documentation
        let path = format!(
            "/v1/{}/bulk/transactions/{}/{}/", 
            chain_name, 
            from_address, 
            to_address
        );
        
        let mut builder = self.build_request(Method::GET, &path);
        
        if let Some(opts) = options {
            if let Some(page_num) = opts.page_number {
                builder = builder.query(&[("page-number", page_num.to_string())]);
            }
            if let Some(page_sz) = opts.page_size {
                builder = builder.query(&[("page-size", page_sz.to_string())]);
            }
            if let Some(currency) = opts.quote_currency {
                builder = builder.query(&[("quote-currency", currency)]);
            }
        }
        
        self.send_with_retry(builder).await
    }
}

/// Iterator for paginating through transactions.
pub struct TransactionsPageIter<'a> {
    client: &'a GoldRushClient,
    chain_name: String,
    address: String,
    options: TxOptions,
    finished: bool,
}

impl<'a> TransactionsPageIter<'a> {
    /// Create a new transaction page iterator.
    pub fn new<C: Into<String>, A: Into<String>>(
        client: &'a GoldRushClient,
        chain_name: C,
        address: A,
        options: TxOptions,
    ) -> Self {
        Self {
            client,
            chain_name: chain_name.into(),
            address: address.into(),
            options,
            finished: false,
        }
    }

    /// Get the next page of transactions.
    pub async fn next(
        &mut self,
    ) -> Result<Option<Vec<crate::models::transactions::TransactionItem>>, Error> {
        if self.finished {
            return Ok(None);
        }

        let resp = self
            .client
            .get_all_transactions_for_address(&self.chain_name, &self.address, Some(self.options.clone()))
            .await?;

        if let Some(data) = resp.data {
            let items = data.items;
            if items.is_empty() || !resp.pagination.as_ref().and_then(|p| p.has_more).unwrap_or(false) {
                self.finished = true;
            } else if let Some(pagination) = resp.pagination {
                if let Some(next_page) = pagination.page_number.map(|n| n + 1) {
                    self.options.page_number = Some(next_page);
                } else {
                    self.finished = true;
                }
            }
            Ok(Some(items))
        } else {
            self.finished = true;
            Ok(None)
        }
    }
    
    /// Check if there are more pages available.
    pub fn has_more(&self) -> bool {
        !self.finished
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tx_options_builder() {
        let options = TxOptions::new()
            .page_size(25)
            .quote_currency("USD")
            .with_log_events(true)
            .starting_block(18000000);
            
        assert_eq!(options.page_size, Some(25));
        assert_eq!(options.quote_currency, Some("USD".to_string()));
        assert_eq!(options.with_log_events, Some(true));
        assert_eq!(options.starting_block, Some(18000000));
    }

    #[test]
    fn test_deserialize_transaction_response() {
        let json_data = json!({
            "data": {
                "tx_hash": "0x123abc...",
                "from_address": "0xfrom...",
                "to_address": "0xto...",
                "value": "1000000000000000000",
                "successful": true,
                "block_height": 18500000,
                "gas_used": 21000,
                "fees_paid": "420000000000000"
            },
            "error": null
        });

        let response: TransactionResponse = serde_json::from_value(json_data).unwrap();
        assert!(response.data.is_some());
        
        let tx = response.data.unwrap();
        assert_eq!(tx.tx_hash, "0x123abc...");
        assert_eq!(tx.successful, Some(true));
        assert_eq!(tx.block_height, Some(18500000));
    }

    #[test]
    fn test_deserialize_transactions_response() {
        let json_data = json!({
            "data": {
                "address": "0x123",
                "chain_id": 1,
                "items": [{
                    "tx_hash": "0x123abc...",
                    "from_address": "0xfrom...",
                    "to_address": "0xto...",
                    "value": "1000000000000000000",
                    "successful": true
                }]
            },
            "pagination": {
                "has_more": true,
                "page_number": 0,
                "page_size": 100
            }
        });

        let response: TransactionsResponse = serde_json::from_value(json_data).unwrap();
        assert!(response.data.is_some());
        
        let data = response.data.unwrap();
        assert_eq!(data.items.len(), 1);
        assert_eq!(data.items[0].tx_hash, "0x123abc...");
    }
}