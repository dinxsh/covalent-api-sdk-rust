use crate::Error;
use crate::http::query::QueryParams;
use crate::models::base::AddressActivityResponse;
use crate::models::all_chains::*;
use crate::services::ServiceContext;
use std::sync::Arc;

/// Options for multi-chain transaction queries.
#[derive(Debug, Clone, Default)]
pub struct MultiChainTxOptions {
    pub chains: Option<Vec<String>>,
    pub addresses: Option<Vec<String>>,
    pub limit: Option<u32>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub with_logs: Option<bool>,
    pub quote_currency: Option<String>,
}

impl MultiChainTxOptions {
    pub fn new() -> Self { Self::default() }
    pub fn chains(mut self, v: Vec<String>) -> Self { self.chains = Some(v); self }
    pub fn addresses(mut self, v: Vec<String>) -> Self { self.addresses = Some(v); self }
    pub fn limit(mut self, v: u32) -> Self { self.limit = Some(v); self }
    pub fn before<S: Into<String>>(mut self, v: S) -> Self { self.before = Some(v.into()); self }
    pub fn after<S: Into<String>>(mut self, v: S) -> Self { self.after = Some(v.into()); self }
    pub fn with_logs(mut self, v: bool) -> Self { self.with_logs = Some(v); self }
    pub fn quote_currency<S: Into<String>>(mut self, c: S) -> Self { self.quote_currency = Some(c.into()); self }
}

impl QueryParams for MultiChainTxOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.chains { builder = builder.query(&[("chains", v.join(","))]); }
        if let Some(v) = self.addresses { builder = builder.query(&[("addresses", v.join(","))]); }
        if let Some(v) = self.limit { builder = builder.query(&[("limit", v.to_string())]); }
        if let Some(v) = self.before { builder = builder.query(&[("before", v)]); }
        if let Some(v) = self.after { builder = builder.query(&[("after", v)]); }
        if let Some(v) = self.with_logs { builder = builder.query(&[("with-logs", v.to_string())]); }
        if let Some(v) = self.quote_currency { builder = builder.query(&[("quote-currency", v)]); }
        builder
    }
}

/// Options for multi-chain balance queries.
#[derive(Debug, Clone, Default)]
pub struct MultiChainBalancesOptions {
    pub chains: Option<Vec<String>>,
    pub quote_currency: Option<String>,
    pub limit: Option<u32>,
    pub cutoff_timestamp: Option<String>,
    pub before: Option<String>,
}

impl MultiChainBalancesOptions {
    pub fn new() -> Self { Self::default() }
    pub fn chains(mut self, v: Vec<String>) -> Self { self.chains = Some(v); self }
    pub fn quote_currency<S: Into<String>>(mut self, c: S) -> Self { self.quote_currency = Some(c.into()); self }
    pub fn limit(mut self, v: u32) -> Self { self.limit = Some(v); self }
    pub fn cutoff_timestamp<S: Into<String>>(mut self, v: S) -> Self { self.cutoff_timestamp = Some(v.into()); self }
    pub fn before<S: Into<String>>(mut self, v: S) -> Self { self.before = Some(v.into()); self }
}

impl QueryParams for MultiChainBalancesOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.chains { builder = builder.query(&[("chains", v.join(","))]); }
        if let Some(v) = self.quote_currency { builder = builder.query(&[("quote-currency", v)]); }
        if let Some(v) = self.limit { builder = builder.query(&[("limit", v.to_string())]); }
        if let Some(v) = self.cutoff_timestamp { builder = builder.query(&[("cutoff-timestamp", v)]); }
        if let Some(v) = self.before { builder = builder.query(&[("before", v)]); }
        builder
    }
}

/// Service for cross-chain API endpoints.
pub struct AllChainsService {
    ctx: Arc<ServiceContext>,
}

impl AllChainsService {
    pub(crate) fn new(ctx: Arc<ServiceContext>) -> Self {
        Self { ctx }
    }

    /// Get address activity across all chains.
    pub async fn get_address_activity(
        &self,
        address: &str,
        options: Option<MultiChainBalancesOptions>,
    ) -> Result<AddressActivityResponse, Error> {
        let path = format!("/v1/address/{}/activity/", address);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(o) => o.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get multi-chain transactions.
    pub async fn get_multi_chain_transactions(
        &self,
        options: Option<MultiChainTxOptions>,
    ) -> Result<MultiChainTransactionsResponse, Error> {
        let builder = self.ctx.get("/v1/allchains/transactions/");
        let builder = match options { Some(o) => o.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get multi-chain balances for an address.
    pub async fn get_multi_chain_balances(
        &self,
        address: &str,
        options: Option<MultiChainBalancesOptions>,
    ) -> Result<MultiChainBalancesResponse, Error> {
        let path = format!("/v1/allchains/address/{}/balances/", address);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(o) => o.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Deprecated: alias for get_multi_chain_transactions.
    #[deprecated(note = "Use get_multi_chain_transactions instead")]
    pub async fn get_multi_chain_and_multi_address_transactions(
        &self,
        options: Option<MultiChainTxOptions>,
    ) -> Result<MultiChainTransactionsResponse, Error> {
        self.get_multi_chain_transactions(options).await
    }
}
