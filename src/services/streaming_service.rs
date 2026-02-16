//! Streaming Service
//!
//! Provides real-time data subscriptions via WebSocket GraphQL.

use std::sync::Arc;

use async_stream::stream;
use futures_util::Stream;
use serde_json::Value;
use tokio::sync::Mutex;
use tracing::instrument;

use crate::error::Result;
use crate::models::streaming::*;
use crate::streaming::{StreamingConfig, SubscriptionHandle, WebSocketClient};

/// Service for streaming real-time blockchain data
pub struct StreamingService {
    api_key: String,
    config: StreamingConfig,
    client: Arc<Mutex<Option<WebSocketClient>>>,
}

impl StreamingService {
    /// Creates a new streaming service
    pub fn new(api_key: String, config: StreamingConfig) -> Self {
        Self {
            api_key,
            config,
            client: Arc::new(Mutex::new(None)),
        }
    }

    /// Gets or creates the WebSocket client
    async fn get_client(&self) -> Result<WebSocketClient> {
        let mut client_guard = self.client.lock().await;

        if let Some(ref client) = *client_guard {
            return Ok(client.clone());
        }

        // Create new client
        let client = WebSocketClient::new(self.api_key.clone(), self.config.clone());
        client.connect().await?;
        *client_guard = Some(client.clone());

        Ok(client)
    }

    /// Subscribes to OHLCV data for specific trading pairs
    ///
    /// # Example
    /// ```no_run
    /// use goldrush_sdk::*;
    /// use goldrush_sdk::models::streaming::*;
    /// use goldrush_sdk::streaming::StreamingConfig;
    /// use futures_util::StreamExt;
    ///
    /// # async fn example() -> Result<()> {
    /// let client = GoldRushClient::new("YOUR_API_KEY", Default::default())?;
    /// let service = client.streaming_service();
    ///
    /// let params = OhlcvPairsParams {
    ///     chain_name: StreamingChain::BaseMainnet,
    ///     pair_addresses: vec!["0x9c087Eb773291e50CF6c6a90ef0F4500e349B903".to_string()],
    ///     interval: StreamingInterval::OneMinute,
    ///     timeframe: StreamingTimeframe::OneHour,
    ///     limit: Some(10),
    /// };
    ///
    /// let (mut stream, handle) = service.subscribe_to_ohlcv_pairs(params).await?;
    ///
    /// while let Some(result) = stream.next().await {
    ///     match result {
    ///         Ok(candles) => println!("Received {} candles", candles.len()),
    ///         Err(e) => eprintln!("Error: {}", e),
    ///     }
    /// }
    ///
    /// handle.unsubscribe().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, params))]
    pub async fn subscribe_to_ohlcv_pairs(
        &self,
        params: OhlcvPairsParams,
    ) -> Result<(impl Stream<Item = Result<Vec<OhlcvPairsResponse>>>, SubscriptionHandle)> {
        let query = build_ohlcv_pairs_query();
        let variables = serde_json::to_value(&params)?;

        let client = self.get_client().await?;
        let (id, mut rx) = client.subscribe(query, Some(variables)).await?;

        let handle = SubscriptionHandle::new(id, self.client.clone());

        let stream = stream! {
            while let Some(result) = rx.recv().await {
                match result {
                    Ok(value) => {
                        match parse_subscription_response::<Vec<OhlcvPairsResponse>>(&value, "subscribeToOHLCVPairs") {
                            Ok(data) => yield Ok(data),
                            Err(e) => yield Err(e),
                        }
                    }
                    Err(e) => yield Err(e),
                }
            }
        };

        Ok((stream, handle))
    }

    /// Subscribes to OHLCV data for specific tokens
    #[instrument(skip(self, params))]
    pub async fn subscribe_to_ohlcv_tokens(
        &self,
        params: OhlcvTokensParams,
    ) -> Result<(impl Stream<Item = Result<Vec<OhlcvTokensResponse>>>, SubscriptionHandle)> {
        let query = build_ohlcv_tokens_query();
        let variables = serde_json::to_value(&params)?;

        let client = self.get_client().await?;
        let (id, mut rx) = client.subscribe(query, Some(variables)).await?;

        let handle = SubscriptionHandle::new(id, self.client.clone());

        let stream = stream! {
            while let Some(result) = rx.recv().await {
                match result {
                    Ok(value) => {
                        match parse_subscription_response::<Vec<OhlcvTokensResponse>>(&value, "subscribeToOHLCVTokens") {
                            Ok(data) => yield Ok(data),
                            Err(e) => yield Err(e),
                        }
                    }
                    Err(e) => yield Err(e),
                }
            }
        };

        Ok((stream, handle))
    }

    /// Subscribes to new DEX pair creation events
    #[instrument(skip(self, params))]
    pub async fn subscribe_to_new_pairs(
        &self,
        params: NewPairsParams,
    ) -> Result<(impl Stream<Item = Result<Vec<NewPairsResponse>>>, SubscriptionHandle)> {
        let query = build_new_pairs_query();
        let variables = serde_json::to_value(&params)?;

        let client = self.get_client().await?;
        let (id, mut rx) = client.subscribe(query, Some(variables)).await?;

        let handle = SubscriptionHandle::new(id, self.client.clone());

        let stream = stream! {
            while let Some(result) = rx.recv().await {
                match result {
                    Ok(value) => {
                        match parse_subscription_response::<Vec<NewPairsResponse>>(&value, "subscribeToNewDexPairs") {
                            Ok(data) => yield Ok(data),
                            Err(e) => yield Err(e),
                        }
                    }
                    Err(e) => yield Err(e),
                }
            }
        };

        Ok((stream, handle))
    }

    /// Subscribes to real-time updates for existing DEX pairs
    #[instrument(skip(self, params))]
    pub async fn subscribe_to_update_pairs(
        &self,
        params: UpdatePairsParams,
    ) -> Result<(impl Stream<Item = Result<UpdatePairsResponse>>, SubscriptionHandle)> {
        let query = build_update_pairs_query();
        let variables = serde_json::to_value(&params)?;

        let client = self.get_client().await?;
        let (id, mut rx) = client.subscribe(query, Some(variables)).await?;

        let handle = SubscriptionHandle::new(id, self.client.clone());

        let stream = stream! {
            while let Some(result) = rx.recv().await {
                match result {
                    Ok(value) => {
                        match parse_subscription_response::<UpdatePairsResponse>(&value, "subscribeToUpdateDexPairs") {
                            Ok(data) => yield Ok(data),
                            Err(e) => yield Err(e),
                        }
                    }
                    Err(e) => yield Err(e),
                }
            }
        };

        Ok((stream, handle))
    }

    /// Subscribes to wallet activity (transactions and transfers)
    ///
    /// # Example
    /// ```no_run
    /// use goldrush_sdk::*;
    /// use goldrush_sdk::models::streaming::*;
    /// use futures_util::StreamExt;
    ///
    /// # async fn example() -> Result<()> {
    /// let client = GoldRushClient::new("YOUR_API_KEY", Default::default())?;
    /// let service = client.streaming_service();
    ///
    /// let params = WalletActivityParams {
    ///     chain_name: StreamingChain::BaseMainnet,
    ///     wallet_addresses: vec!["0x4200000000000000000000000000000000000006".to_string()],
    /// };
    ///
    /// let (mut stream, handle) = service.subscribe_to_wallet_activity(params).await?;
    ///
    /// while let Some(result) = stream.next().await {
    ///     match result {
    ///         Ok(txs) => {
    ///             for tx in txs {
    ///                 println!("TX: {} - {} -> {}", tx.tx_hash, tx.from_address, tx.to_address);
    ///             }
    ///         }
    ///         Err(e) => eprintln!("Error: {}", e),
    ///     }
    /// }
    ///
    /// handle.unsubscribe().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, params))]
    pub async fn subscribe_to_wallet_activity(
        &self,
        params: WalletActivityParams,
    ) -> Result<(impl Stream<Item = Result<Vec<WalletActivityResponse>>>, SubscriptionHandle)> {
        let query = build_wallet_activity_query();
        let variables = serde_json::to_value(&params)?;

        let client = self.get_client().await?;
        let (id, mut rx) = client.subscribe(query, Some(variables)).await?;

        let handle = SubscriptionHandle::new(id, self.client.clone());

        let stream = stream! {
            while let Some(result) = rx.recv().await {
                match result {
                    Ok(value) => {
                        match parse_subscription_response::<Vec<WalletActivityResponse>>(&value, "subscribeToWalletActivity") {
                            Ok(data) => yield Ok(data),
                            Err(e) => yield Err(e),
                        }
                    }
                    Err(e) => yield Err(e),
                }
            }
        };

        Ok((stream, handle))
    }

    /// Searches for tokens by name or symbol
    ///
    /// # Example
    /// ```no_run
    /// use goldrush_sdk::*;
    /// use goldrush_sdk::models::streaming::*;
    ///
    /// # async fn example() -> Result<()> {
    /// let client = GoldRushClient::new("YOUR_API_KEY", Default::default())?;
    /// let service = client.streaming_service();
    ///
    /// let params = TokenSearchParams { query: "USDC".to_string() };
    /// let results = service.search_token(params).await?;
    ///
    /// for token in results {
    ///     println!("{} - Market Cap: ${}",
    ///         token.base_token.contract_ticker_symbol.unwrap_or_default(),
    ///         token.market_cap);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, params))]
    pub async fn search_token(&self, params: TokenSearchParams) -> Result<Vec<TokenSearchResponse>> {
        let query = build_search_token_query();
        let variables = serde_json::to_value(&params)?;

        let client = self.get_client().await?;
        let (id, mut rx) = client.subscribe(query, Some(variables)).await?;

        // For queries, we expect a single response then complete
        if let Some(result) = rx.recv().await {
            let value = result?;
            let data = parse_query_response::<Vec<TokenSearchResponse>>(&value, "searchToken")?;

            // Unsubscribe after getting the response
            let _ = client.unsubscribe(&id).await;

            Ok(data)
        } else {
            Err(crate::error::Error::Streaming("No response received".to_string()))
        }
    }

    /// Gets unrealized P&L for top traders of a token
    #[instrument(skip(self, params))]
    pub async fn get_upnl_for_token(
        &self,
        params: UpnlForTokenParams,
    ) -> Result<Vec<UpnlForTokenResponse>> {
        let query = build_upnl_for_token_query();
        let variables = serde_json::to_value(&params)?;

        let client = self.get_client().await?;
        let (id, mut rx) = client.subscribe(query, Some(variables)).await?;

        if let Some(result) = rx.recv().await {
            let value = result?;
            let data = parse_query_response::<Vec<UpnlForTokenResponse>>(&value, "getUPnLForToken")?;

            let _ = client.unsubscribe(&id).await;

            Ok(data)
        } else {
            Err(crate::error::Error::Streaming("No response received".to_string()))
        }
    }

    /// Gets unrealized P&L for all tokens held by a wallet
    #[instrument(skip(self, params))]
    pub async fn get_upnl_for_wallet(
        &self,
        params: UpnlForWalletParams,
    ) -> Result<Vec<UpnlForWalletResponse>> {
        let query = build_upnl_for_wallet_query();
        let variables = serde_json::to_value(&params)?;

        let client = self.get_client().await?;
        let (id, mut rx) = client.subscribe(query, Some(variables)).await?;

        if let Some(result) = rx.recv().await {
            let value = result?;
            let data = parse_query_response::<Vec<UpnlForWalletResponse>>(&value, "getUPnLForWallet")?;

            let _ = client.unsubscribe(&id).await;

            Ok(data)
        } else {
            Err(crate::error::Error::Streaming("No response received".to_string()))
        }
    }
}

// =============================================================================
// GraphQL Query Builders
// =============================================================================

fn build_ohlcv_pairs_query() -> String {
    r#"
    subscription SubscribeToOHLCVPairs($chain_name: StreamingChain!, $pair_addresses: [String!]!, $interval: StreamingInterval!, $timeframe: StreamingTimeframe!, $limit: Int) {
      subscribeToOHLCVPairs(chain_name: $chain_name, pair_addresses: $pair_addresses, interval: $interval, timeframe: $timeframe, limit: $limit) {
        chain_name
        pair_address
        interval
        timeframe
        timestamp
        open
        high
        low
        close
        volume
        volume_usd
        quote_rate
        quote_rate_usd
        base_token {
          contract_decimals
          contract_name
          contract_ticker_symbol
          contract_address
          supports_erc
          logo_url
        }
        quote_token {
          contract_decimals
          contract_name
          contract_ticker_symbol
          contract_address
          supports_erc
          logo_url
        }
      }
    }
    "#.to_string()
}

fn build_ohlcv_tokens_query() -> String {
    r#"
    subscription SubscribeToOHLCVTokens($chain_name: StreamingChain!, $token_addresses: [String!]!, $interval: StreamingInterval!, $timeframe: StreamingTimeframe!, $limit: Int) {
      subscribeToOHLCVTokens(chain_name: $chain_name, token_addresses: $token_addresses, interval: $interval, timeframe: $timeframe, limit: $limit) {
        chain_name
        pair_address
        interval
        timeframe
        timestamp
        open
        high
        low
        close
        volume
        volume_usd
        quote_rate
        quote_rate_usd
        base_token {
          contract_decimals
          contract_name
          contract_ticker_symbol
          contract_address
          supports_erc
          logo_url
        }
        quote_token {
          contract_decimals
          contract_name
          contract_ticker_symbol
          contract_address
          supports_erc
          logo_url
        }
      }
    }
    "#.to_string()
}

fn build_new_pairs_query() -> String {
    r#"
    subscription SubscribeToNewDexPairs($chain_name: StreamingChain!, $protocols: [StreamingProtocol!]!) {
      subscribeToNewDexPairs(chain_name: $chain_name, protocols: $protocols) {
        chain_name
        protocol
        protocol_version
        pair_address
        deployer_address
        tx_hash
        block_signed_at
        liquidity
        supply
        market_cap
        event_name
        quote_rate
        quote_rate_usd
        base_token {
          contract_decimals
          contract_name
          contract_ticker_symbol
          contract_address
          supports_erc
          logo_url
        }
        quote_token {
          contract_decimals
          contract_name
          contract_ticker_symbol
          contract_address
          supports_erc
          logo_url
        }
        pair {
          contract_decimals
          contract_name
          contract_ticker_symbol
          contract_address
          supports_erc
          logo_url
        }
      }
    }
    "#.to_string()
}

fn build_update_pairs_query() -> String {
    r#"
    subscription SubscribeToUpdateDexPairs($chain_name: StreamingChain!, $pair_addresses: [String!]!) {
      subscribeToUpdateDexPairs(chain_name: $chain_name, pair_addresses: $pair_addresses) {
        chain_name
        pair_address
        timestamp
        quote_rate
        quote_rate_usd
        volume
        volume_usd
        market_cap
        liquidity
        base_token {
          contract_decimals
          contract_name
          contract_ticker_symbol
          contract_address
          supports_erc
          logo_url
        }
        quote_token {
          contract_decimals
          contract_name
          contract_ticker_symbol
          contract_address
          supports_erc
          logo_url
        }
        price_deltas {
          last_5m
          last_1hr
          last_6hr
          last_24hr
        }
        swap_counts {
          last_5m
          last_1hr
          last_6hr
          last_24hr
        }
      }
    }
    "#.to_string()
}

fn build_wallet_activity_query() -> String {
    r#"
    subscription SubscribeToWalletActivity($chain_name: StreamingChain!, $wallet_addresses: [String!]!) {
      subscribeToWalletActivity(chain_name: $chain_name, wallet_addresses: $wallet_addresses) {
        tx_hash
        from_address
        to_address
        value
        chain_name
        block_signed_at
        block_height
        block_hash
        miner_address
        gas_used
        tx_offset
        successful
        decoded_type
        decoded_details
        logs {
          emitter_address
          log_offset
          data
          topics
        }
      }
    }
    "#.to_string()
}

fn build_search_token_query() -> String {
    r#"
    query SearchToken($query: String!) {
      searchToken(query: $query) {
        pair_address
        chain_name
        quote_rate
        quote_rate_usd
        volume
        volume_usd
        market_cap
        base_token {
          contract_decimals
          contract_name
          contract_ticker_symbol
          contract_address
          supports_erc
          logo_url
        }
        quote_token {
          contract_decimals
          contract_name
          contract_ticker_symbol
          contract_address
          supports_erc
          logo_url
        }
      }
    }
    "#.to_string()
}

fn build_upnl_for_token_query() -> String {
    r#"
    query GetUPnLForToken($chain_name: StreamingChain!, $token_address: String!) {
      getUPnLForToken(chain_name: $chain_name, token_address: $token_address) {
        token_address
        wallet_address
        volume
        transactions_count
        pnl_realized_usd
        balance
        balance_pretty
        pnl_unrealized_usd
        contract_metadata {
          contract_decimals
          contract_name
          contract_ticker_symbol
          contract_address
          supports_erc
          logo_url
        }
      }
    }
    "#.to_string()
}

fn build_upnl_for_wallet_query() -> String {
    r#"
    query GetUPnLForWallet($chain_name: StreamingChain!, $wallet_address: String!) {
      getUPnLForWallet(chain_name: $chain_name, wallet_address: $wallet_address) {
        token_address
        cost_basis
        current_price
        pnl_realized_usd
        pnl_unrealized_usd
        net_balance_change
        marketcap_usd
        contract_metadata {
          contract_decimals
          contract_name
          contract_ticker_symbol
          contract_address
          supports_erc
          logo_url
        }
      }
    }
    "#.to_string()
}

// =============================================================================
// Response Parsers
// =============================================================================

fn parse_subscription_response<T>(value: &Value, field_name: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let data = value
        .get("data")
        .and_then(|d| d.get(field_name))
        .ok_or_else(|| crate::error::Error::Streaming(format!("Missing field: {}", field_name)))?;

    serde_json::from_value(data.clone())
        .map_err(|e| crate::error::Error::Streaming(format!("Deserialization error: {}", e)))
}

fn parse_query_response<T>(value: &Value, field_name: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let data = value
        .get("data")
        .and_then(|d| d.get(field_name))
        .ok_or_else(|| crate::error::Error::Streaming(format!("Missing field: {}", field_name)))?;

    serde_json::from_value(data.clone())
        .map_err(|e| crate::error::Error::Streaming(format!("Deserialization error: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_builders() {
        let query = build_ohlcv_pairs_query();
        assert!(query.contains("subscribeToOHLCVPairs"));

        let query = build_search_token_query();
        assert!(query.contains("searchToken"));
    }
}
