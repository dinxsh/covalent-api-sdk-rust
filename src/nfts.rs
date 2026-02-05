use crate::{Error, GoldRushClient};
use crate::models::nfts::{NftsResponse, NftMetadataResponse};
use reqwest::Method;

/// Options for customizing NFT queries.
#[derive(Debug, Clone, Default)]
pub struct NftOptions {
    /// Page number for pagination (0-indexed).
    pub page_number: Option<u32>,
    
    /// Number of items per page.
    pub page_size: Option<u32>,
    
    /// Quote currency for pricing (e.g., "USD", "ETH").
    pub quote_currency: Option<String>,
    
    /// Whether to include metadata.
    pub with_metadata: Option<bool>,
    
    /// Whether to exclude spam NFTs.
    pub no_spam: Option<bool>,
}

impl NftOptions {
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
    
    /// Include or exclude metadata in the response.
    pub fn with_metadata(mut self, include_metadata: bool) -> Self {
        self.with_metadata = Some(include_metadata);
        self
    }
    
    /// Exclude spam NFTs.
    pub fn no_spam(mut self, exclude_spam: bool) -> Self {
        self.no_spam = Some(exclude_spam);
        self
    }
}

impl GoldRushClient {
    /// Get NFTs owned by an address.
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
    /// use goldrush_sdk::{GoldRushClient, NftOptions};
    ///
    /// # async fn example(client: GoldRushClient) -> Result<(), goldrush_sdk::Error> {
    /// let options = NftOptions::new()
    ///     .page_size(10)
    ///     .with_metadata(true)
    ///     .no_spam(true);
    ///     
    /// let nfts = client
    ///     .get_nfts_for_address(
    ///         "eth-mainnet",
    ///         "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
    ///         Some(options)
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_nfts_for_address(
        &self,
        chain_name: &str,
        address: &str,
        options: Option<NftOptions>,
    ) -> Result<NftsResponse, Error> {
        // TODO: Confirm exact endpoint path with maintainers
        let path = format!("/v1/{}/address/{}/balances_nft/", chain_name, address);
        
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
            if let Some(with_meta) = opts.with_metadata {
                builder = builder.query(&[("with-metadata", with_meta.to_string())]);
            }
            if let Some(no_spam) = opts.no_spam {
                builder = builder.query(&[("no-spam", no_spam.to_string())]);
            }
        }
        
        self.send_with_retry(builder).await
    }

    /// Get metadata for a specific NFT.
    ///
    /// # Arguments
    ///
    /// * `chain_name` - The blockchain name
    /// * `contract_address` - The NFT collection contract address
    /// * `token_id` - The specific token ID
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use goldrush_sdk::GoldRushClient;
    ///
    /// # async fn example(client: GoldRushClient) -> Result<(), goldrush_sdk::Error> {
    /// let metadata = client
    ///     .get_nft_metadata(
    ///         "eth-mainnet",
    ///         "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d",
    ///         "1"
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_nft_metadata(
        &self,
        chain_name: &str,
        contract_address: &str,
        token_id: &str,
    ) -> Result<NftMetadataResponse, Error> {
        // TODO: Confirm exact endpoint path with maintainers
        let path = format!(
            "/v1/{}/tokens/{}/nft_metadata/{}/",
            chain_name,
            contract_address,
            token_id
        );
        
        let builder = self.build_request(Method::GET, &path);
        self.send_with_retry(builder).await
    }
    
    /// Get all NFTs from a specific collection.
    ///
    /// # Arguments
    ///
    /// * `chain_name` - The blockchain name
    /// * `contract_address` - The NFT collection contract address
    /// * `options` - Optional query parameters
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use goldrush_sdk::GoldRushClient;
    ///
    /// # async fn example(client: GoldRushClient) -> Result<(), goldrush_sdk::Error> {
    /// let collection_nfts = client
    ///     .get_nfts_for_collection(
    ///         "eth-mainnet",
    ///         "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d",
    ///         None
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_nfts_for_collection(
        &self,
        chain_name: &str,
        contract_address: &str,
        options: Option<NftOptions>,
    ) -> Result<NftsResponse, Error> {
        // TODO: Confirm exact endpoint path with maintainers
        let path = format!("/v1/{}/tokens/{}/nft_token_ids/", chain_name, contract_address);
        
        let mut builder = self.build_request(Method::GET, &path);
        
        if let Some(opts) = options {
            if let Some(page_num) = opts.page_number {
                builder = builder.query(&[("page-number", page_num.to_string())]);
            }
            if let Some(page_sz) = opts.page_size {
                builder = builder.query(&[("page-size", page_sz.to_string())]);
            }
            if let Some(with_meta) = opts.with_metadata {
                builder = builder.query(&[("with-metadata", with_meta.to_string())]);
            }
        }
        
        self.send_with_retry(builder).await
    }
    
    /// Get NFT owners for a specific collection.
    ///
    /// # Arguments
    ///
    /// * `chain_name` - The blockchain name
    /// * `contract_address` - The NFT collection contract address
    /// * `options` - Optional query parameters
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use goldrush_sdk::GoldRushClient;
    ///
    /// # async fn example(client: GoldRushClient) -> Result<(), goldrush_sdk::Error> {
    /// let owners = client
    ///     .get_nft_owners_for_collection(
    ///         "eth-mainnet",
    ///         "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d",
    ///         None
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_nft_owners_for_collection(
        &self,
        chain_name: &str,
        contract_address: &str,
        options: Option<NftOptions>,
    ) -> Result<NftsResponse, Error> {
        // TODO: Confirm exact endpoint path with maintainers
        let path = format!("/v1/{}/tokens/{}/nft_token_owners/", chain_name, contract_address);
        
        let mut builder = self.build_request(Method::GET, &path);
        
        if let Some(opts) = options {
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

/// Iterator for paginating through NFTs.
pub struct NftsPageIter<'a> {
    client: &'a GoldRushClient,
    chain_name: String,
    address: String,
    options: NftOptions,
    finished: bool,
}

impl<'a> NftsPageIter<'a> {
    /// Create a new NFTs page iterator.
    pub fn new<C: Into<String>, A: Into<String>>(
        client: &'a GoldRushClient,
        chain_name: C,
        address: A,
        options: NftOptions,
    ) -> Self {
        Self {
            client,
            chain_name: chain_name.into(),
            address: address.into(),
            options,
            finished: false,
        }
    }

    /// Get the next page of NFTs.
    pub async fn next(
        &mut self,
    ) -> Result<Option<Vec<crate::models::nfts::NftItem>>, Error> {
        if self.finished {
            return Ok(None);
        }

        let resp = self
            .client
            .get_nfts_for_address(&self.chain_name, &self.address, Some(self.options.clone()))
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
    fn test_nft_options_builder() {
        let options = NftOptions::new()
            .page_size(20)
            .with_metadata(true)
            .no_spam(true)
            .quote_currency("USD");
            
        assert_eq!(options.page_size, Some(20));
        assert_eq!(options.with_metadata, Some(true));
        assert_eq!(options.no_spam, Some(true));
        assert_eq!(options.quote_currency, Some("USD".to_string()));
    }

    #[test]
    fn test_deserialize_nfts_response() {
        let json_data = json!({
            "data": {
                "address": "0x123",
                "chain_id": 1,
                "items": [{
                    "contract_address": "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d",
                    "token_id": "1",
                    "token_balance": "1",
                    "contract_name": "Bored Ape Yacht Club",
                    "contract_ticker_symbol": "BAYC",
                    "supports_erc": ["erc721"]
                }]
            },
            "pagination": {
                "has_more": false,
                "page_number": 0,
                "page_size": 100,
                "total_count": 1
            }
        });

        let response: NftsResponse = serde_json::from_value(json_data).unwrap();
        assert!(response.data.is_some());
        
        let data = response.data.unwrap();
        assert_eq!(data.items.len(), 1);
        assert_eq!(data.items[0].contract_ticker_symbol, Some("BAYC".to_string()));
        assert_eq!(data.items[0].token_id, "1");
    }

    #[test] 
    fn test_deserialize_nft_metadata_response() {
        let json_data = json!({
            "data": [{
                "contract_address": "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d",
                "token_id": "1",
                "token_uri": "https://ipfs.io/ipfs/...",
                "metadata": {
                    "name": "Bored Ape #1",
                    "description": "A bored ape",
                    "image": "https://ipfs.io/ipfs/..."
                }
            }]
        });

        let response: NftMetadataResponse = serde_json::from_value(json_data).unwrap();
        assert!(response.data.is_some());
        
        let data = response.data.unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].contract_address, "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d");
        assert_eq!(data[0].token_id, "1");
    }
}