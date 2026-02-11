use crate::Error;
use crate::http::query::QueryParams;
use crate::models::balances::{BalancesResponse, Erc20TransfersResponse, TokenHoldersResponse, HistoricalBalancesResponse, NativeTokenBalanceResponse};
use crate::services::ServiceContext;
use std::sync::Arc;

/// Options for balance queries.
#[derive(Debug, Clone, Default)]
pub struct BalancesOptions {
    pub quote_currency: Option<String>,
    pub nft: Option<bool>,
    pub no_spam: Option<bool>,
    pub no_nft_fetch: Option<bool>,
    pub no_nft_asset_metadata: Option<bool>,
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
}

impl BalancesOptions {
    pub fn new() -> Self { Self::default() }
    pub fn quote_currency<S: Into<String>>(mut self, c: S) -> Self { self.quote_currency = Some(c.into()); self }
    pub fn nft(mut self, v: bool) -> Self { self.nft = Some(v); self }
    pub fn no_spam(mut self, v: bool) -> Self { self.no_spam = Some(v); self }
    pub fn no_nft_fetch(mut self, v: bool) -> Self { self.no_nft_fetch = Some(v); self }
    pub fn no_nft_asset_metadata(mut self, v: bool) -> Self { self.no_nft_asset_metadata = Some(v); self }
    pub fn page_number(mut self, v: u32) -> Self { self.page_number = Some(v); self }
    pub fn page_size(mut self, v: u32) -> Self { self.page_size = Some(v); self }
}

impl QueryParams for BalancesOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.quote_currency { builder = builder.query(&[("quote-currency", v)]); }
        if let Some(v) = self.nft { builder = builder.query(&[("nft", v.to_string())]); }
        if let Some(v) = self.no_spam { builder = builder.query(&[("no-spam", v.to_string())]); }
        if let Some(v) = self.no_nft_fetch { builder = builder.query(&[("no-nft-fetch", v.to_string())]); }
        if let Some(v) = self.no_nft_asset_metadata { builder = builder.query(&[("no-nft-asset-metadata", v.to_string())]); }
        if let Some(v) = self.page_number { builder = builder.query(&[("page-number", v.to_string())]); }
        if let Some(v) = self.page_size { builder = builder.query(&[("page-size", v.to_string())]); }
        builder
    }
}

/// Options for portfolio queries.
#[derive(Debug, Clone, Default)]
pub struct PortfolioOptions {
    pub quote_currency: Option<String>,
    pub days: Option<u32>,
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
}

impl PortfolioOptions {
    pub fn new() -> Self { Self::default() }
    pub fn quote_currency<S: Into<String>>(mut self, c: S) -> Self { self.quote_currency = Some(c.into()); self }
    pub fn days(mut self, v: u32) -> Self { self.days = Some(v); self }
    pub fn page_number(mut self, v: u32) -> Self { self.page_number = Some(v); self }
    pub fn page_size(mut self, v: u32) -> Self { self.page_size = Some(v); self }
}

impl QueryParams for PortfolioOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.quote_currency { builder = builder.query(&[("quote-currency", v)]); }
        if let Some(v) = self.days { builder = builder.query(&[("days", v.to_string())]); }
        if let Some(v) = self.page_number { builder = builder.query(&[("page-number", v.to_string())]); }
        if let Some(v) = self.page_size { builder = builder.query(&[("page-size", v.to_string())]); }
        builder
    }
}

/// Options for ERC20 transfer queries.
#[derive(Debug, Clone, Default)]
pub struct Erc20TransfersOptions {
    pub quote_currency: Option<String>,
    pub contract_address: Option<String>,
    pub starting_block: Option<u64>,
    pub ending_block: Option<u64>,
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
}

impl Erc20TransfersOptions {
    pub fn new() -> Self { Self::default() }
    pub fn quote_currency<S: Into<String>>(mut self, c: S) -> Self { self.quote_currency = Some(c.into()); self }
    pub fn contract_address<S: Into<String>>(mut self, c: S) -> Self { self.contract_address = Some(c.into()); self }
    pub fn starting_block(mut self, v: u64) -> Self { self.starting_block = Some(v); self }
    pub fn ending_block(mut self, v: u64) -> Self { self.ending_block = Some(v); self }
    pub fn page_number(mut self, v: u32) -> Self { self.page_number = Some(v); self }
    pub fn page_size(mut self, v: u32) -> Self { self.page_size = Some(v); self }
}

impl QueryParams for Erc20TransfersOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.quote_currency { builder = builder.query(&[("quote-currency", v)]); }
        if let Some(v) = self.contract_address { builder = builder.query(&[("contract-address", v)]); }
        if let Some(v) = self.starting_block { builder = builder.query(&[("starting-block", v.to_string())]); }
        if let Some(v) = self.ending_block { builder = builder.query(&[("ending-block", v.to_string())]); }
        if let Some(v) = self.page_number { builder = builder.query(&[("page-number", v.to_string())]); }
        if let Some(v) = self.page_size { builder = builder.query(&[("page-size", v.to_string())]); }
        builder
    }
}

/// Options for token holder queries.
#[derive(Debug, Clone, Default)]
pub struct TokenHoldersOptions {
    pub quote_currency: Option<String>,
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
    pub block_height: Option<u64>,
}

impl TokenHoldersOptions {
    pub fn new() -> Self { Self::default() }
    pub fn quote_currency<S: Into<String>>(mut self, c: S) -> Self { self.quote_currency = Some(c.into()); self }
    pub fn page_number(mut self, v: u32) -> Self { self.page_number = Some(v); self }
    pub fn page_size(mut self, v: u32) -> Self { self.page_size = Some(v); self }
    pub fn block_height(mut self, v: u64) -> Self { self.block_height = Some(v); self }
}

impl QueryParams for TokenHoldersOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.quote_currency { builder = builder.query(&[("quote-currency", v)]); }
        if let Some(v) = self.page_number { builder = builder.query(&[("page-number", v.to_string())]); }
        if let Some(v) = self.page_size { builder = builder.query(&[("page-size", v.to_string())]); }
        if let Some(v) = self.block_height { builder = builder.query(&[("block-height", v.to_string())]); }
        builder
    }
}

/// Options for historical balance queries.
#[derive(Debug, Clone, Default)]
pub struct HistoricalBalancesOptions {
    pub quote_currency: Option<String>,
    pub date: Option<String>,
    pub block_height: Option<u64>,
}

impl HistoricalBalancesOptions {
    pub fn new() -> Self { Self::default() }
    pub fn quote_currency<S: Into<String>>(mut self, c: S) -> Self { self.quote_currency = Some(c.into()); self }
    pub fn date<S: Into<String>>(mut self, d: S) -> Self { self.date = Some(d.into()); self }
    pub fn block_height(mut self, v: u64) -> Self { self.block_height = Some(v); self }
}

impl QueryParams for HistoricalBalancesOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.quote_currency { builder = builder.query(&[("quote-currency", v)]); }
        if let Some(v) = self.date { builder = builder.query(&[("date", v)]); }
        if let Some(v) = self.block_height { builder = builder.query(&[("block-height", v.to_string())]); }
        builder
    }
}

/// Options for native token balance queries.
#[derive(Debug, Clone, Default)]
pub struct NativeBalanceOptions {
    pub quote_currency: Option<String>,
    pub block_height: Option<u64>,
}

impl NativeBalanceOptions {
    pub fn new() -> Self { Self::default() }
    pub fn quote_currency<S: Into<String>>(mut self, c: S) -> Self { self.quote_currency = Some(c.into()); self }
    pub fn block_height(mut self, v: u64) -> Self { self.block_height = Some(v); self }
}

impl QueryParams for NativeBalanceOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.quote_currency { builder = builder.query(&[("quote-currency", v)]); }
        if let Some(v) = self.block_height { builder = builder.query(&[("block-height", v.to_string())]); }
        builder
    }
}

/// Service for balance-related API endpoints.
pub struct BalanceService {
    ctx: Arc<ServiceContext>,
}

impl BalanceService {
    pub(crate) fn new(ctx: Arc<ServiceContext>) -> Self {
        Self { ctx }
    }

    /// Get token balances for a wallet address.
    pub async fn get_token_balances_for_wallet_address(
        &self,
        chain_name: impl AsRef<str>,
        address: &str,
        options: Option<BalancesOptions>,
    ) -> Result<BalancesResponse, Error> {
        let path = format!("/v1/{}/address/{}/balances_v2/", chain_name.as_ref(), address);
        let builder = self.ctx.get(&path);
        let builder = match options {
            Some(opts) => opts.apply_to(builder),
            None => builder,
        };
        self.ctx.send_with_retry(builder).await
    }

    /// Get historical portfolio balances for an address.
    pub async fn get_historical_portfolio_for_wallet_address(
        &self,
        chain_name: impl AsRef<str>,
        address: &str,
        options: Option<PortfolioOptions>,
    ) -> Result<BalancesResponse, Error> {
        let path = format!("/v1/{}/address/{}/portfolio_v2/", chain_name.as_ref(), address);
        let builder = self.ctx.get(&path);
        let builder = match options {
            Some(opts) => opts.apply_to(builder),
            None => builder,
        };
        self.ctx.send_with_retry(builder).await
    }

    /// Get ERC20 token transfers for a wallet address.
    pub async fn get_erc20_transfers_for_wallet_address(
        &self,
        chain_name: impl AsRef<str>,
        address: &str,
        options: Option<Erc20TransfersOptions>,
    ) -> Result<Erc20TransfersResponse, Error> {
        let path = format!("/v1/{}/address/{}/transfers_v2/", chain_name.as_ref(), address);
        let builder = self.ctx.get(&path);
        let builder = match options {
            Some(opts) => opts.apply_to(builder),
            None => builder,
        };
        self.ctx.send_with_retry(builder).await
    }

    /// Get token holders for a token address (v2).
    pub async fn get_token_holders_v2_for_token_address(
        &self,
        chain_name: impl AsRef<str>,
        token_address: &str,
        options: Option<TokenHoldersOptions>,
    ) -> Result<TokenHoldersResponse, Error> {
        let path = format!("/v1/{}/tokens/{}/token_holders_v2/", chain_name.as_ref(), token_address);
        let builder = self.ctx.get(&path);
        let builder = match options {
            Some(opts) => opts.apply_to(builder),
            None => builder,
        };
        self.ctx.send_with_retry(builder).await
    }

    /// Get historical token balances for an address.
    pub async fn get_historical_token_balances(
        &self,
        chain_name: impl AsRef<str>,
        address: &str,
        options: Option<HistoricalBalancesOptions>,
    ) -> Result<HistoricalBalancesResponse, Error> {
        let path = format!("/v1/{}/address/{}/historical_balances/", chain_name.as_ref(), address);
        let builder = self.ctx.get(&path);
        let builder = match options {
            Some(opts) => opts.apply_to(builder),
            None => builder,
        };
        self.ctx.send_with_retry(builder).await
    }

    /// Get native token balance for an address.
    pub async fn get_native_token_balance(
        &self,
        chain_name: impl AsRef<str>,
        address: &str,
        options: Option<NativeBalanceOptions>,
    ) -> Result<NativeTokenBalanceResponse, Error> {
        let path = format!("/v1/{}/address/{}/balances_native/", chain_name.as_ref(), address);
        let builder = self.ctx.get(&path);
        let builder = match options {
            Some(opts) => opts.apply_to(builder),
            None => builder,
        };
        self.ctx.send_with_retry(builder).await
    }
}
