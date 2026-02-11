use crate::Error;
use crate::http::query::QueryParams;
use crate::models::base::*;
use crate::services::ServiceContext;
use std::sync::Arc;

/// Options for block height queries.
#[derive(Debug, Clone, Default)]
pub struct BlockHeightsOptions {
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
}

impl BlockHeightsOptions {
    pub fn new() -> Self { Self::default() }
    pub fn page_number(mut self, v: u32) -> Self { self.page_number = Some(v); self }
    pub fn page_size(mut self, v: u32) -> Self { self.page_size = Some(v); self }
}

impl QueryParams for BlockHeightsOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.page_number { builder = builder.query(&[("page-number", v.to_string())]); }
        if let Some(v) = self.page_size { builder = builder.query(&[("page-size", v.to_string())]); }
        builder
    }
}

/// Options for log event queries by address.
#[derive(Debug, Clone, Default)]
pub struct LogEventsByAddressOptions {
    pub starting_block: Option<u64>,
    pub ending_block: Option<u64>,
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
}

impl LogEventsByAddressOptions {
    pub fn new() -> Self { Self::default() }
    pub fn starting_block(mut self, v: u64) -> Self { self.starting_block = Some(v); self }
    pub fn ending_block(mut self, v: u64) -> Self { self.ending_block = Some(v); self }
    pub fn page_number(mut self, v: u32) -> Self { self.page_number = Some(v); self }
    pub fn page_size(mut self, v: u32) -> Self { self.page_size = Some(v); self }
}

impl QueryParams for LogEventsByAddressOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.starting_block { builder = builder.query(&[("starting-block", v.to_string())]); }
        if let Some(v) = self.ending_block { builder = builder.query(&[("ending-block", v.to_string())]); }
        if let Some(v) = self.page_number { builder = builder.query(&[("page-number", v.to_string())]); }
        if let Some(v) = self.page_size { builder = builder.query(&[("page-size", v.to_string())]); }
        builder
    }
}

/// Options for log event queries by topic hash.
#[derive(Debug, Clone, Default)]
pub struct LogEventsByTopicOptions {
    pub starting_block: Option<u64>,
    pub ending_block: Option<u64>,
    pub secondary_topics: Option<String>,
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
}

impl LogEventsByTopicOptions {
    pub fn new() -> Self { Self::default() }
    pub fn starting_block(mut self, v: u64) -> Self { self.starting_block = Some(v); self }
    pub fn ending_block(mut self, v: u64) -> Self { self.ending_block = Some(v); self }
    pub fn secondary_topics<S: Into<String>>(mut self, v: S) -> Self { self.secondary_topics = Some(v.into()); self }
    pub fn page_number(mut self, v: u32) -> Self { self.page_number = Some(v); self }
    pub fn page_size(mut self, v: u32) -> Self { self.page_size = Some(v); self }
}

impl QueryParams for LogEventsByTopicOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.starting_block { builder = builder.query(&[("starting-block", v.to_string())]); }
        if let Some(v) = self.ending_block { builder = builder.query(&[("ending-block", v.to_string())]); }
        if let Some(v) = self.secondary_topics { builder = builder.query(&[("secondary-topics", v)]); }
        if let Some(v) = self.page_number { builder = builder.query(&[("page-number", v.to_string())]); }
        if let Some(v) = self.page_size { builder = builder.query(&[("page-size", v.to_string())]); }
        builder
    }
}

/// Service for base/utility API endpoints.
pub struct BaseService {
    ctx: Arc<ServiceContext>,
}

impl BaseService {
    pub(crate) fn new(ctx: Arc<ServiceContext>) -> Self {
        Self { ctx }
    }

    /// Get block data by block height.
    pub async fn get_block(
        &self, chain_name: impl AsRef<str>, block_height: &str,
    ) -> Result<BlockResponse, Error> {
        let path = format!("/v1/{}/block_v2/{}/", chain_name.as_ref(), block_height);
        self.ctx.send_with_retry(self.ctx.get(&path)).await
    }

    /// Resolve an address to an ENS or other domain name.
    pub async fn get_resolved_address(
        &self, chain_name: impl AsRef<str>, address: &str,
    ) -> Result<ResolvedAddressResponse, Error> {
        let path = format!("/v1/{}/address/{}/resolve_address/", chain_name.as_ref(), address);
        self.ctx.send_with_retry(self.ctx.get(&path)).await
    }

    /// Get block heights between two dates.
    pub async fn get_block_heights(
        &self, chain_name: impl AsRef<str>, start_date: &str, end_date: &str, options: Option<BlockHeightsOptions>,
    ) -> Result<BlockHeightsResponse, Error> {
        let path = format!("/v1/{}/block_v2/{}/{}/", chain_name.as_ref(), start_date, end_date);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(o) => o.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get log events by contract address.
    pub async fn get_log_events_by_address(
        &self, chain_name: impl AsRef<str>, contract_address: &str, options: Option<LogEventsByAddressOptions>,
    ) -> Result<LogsResponse, Error> {
        let path = format!("/v1/{}/events/address/{}/", chain_name.as_ref(), contract_address);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(o) => o.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get log events by topic hash.
    pub async fn get_log_events_by_topic_hash(
        &self, chain_name: impl AsRef<str>, topic: &str, options: Option<LogEventsByTopicOptions>,
    ) -> Result<LogsResponse, Error> {
        let path = format!("/v1/{}/events/topics/{}/", chain_name.as_ref(), topic);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(o) => o.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get all supported chains.
    pub async fn get_all_chains(&self) -> Result<AllChainsResponse, Error> {
        self.ctx.send_with_retry(self.ctx.get("/v1/chains/")).await
    }

    /// Get status of all supported chains.
    pub async fn get_all_chain_status(&self) -> Result<AllChainStatusResponse, Error> {
        self.ctx.send_with_retry(self.ctx.get("/v1/chains/status/")).await
    }

    /// Get gas prices for a specific event type.
    pub async fn get_gas_prices(
        &self, chain_name: impl AsRef<str>, event_type: impl AsRef<str>,
    ) -> Result<GasPricesResponse, Error> {
        let path = format!("/v1/{}/event/{}/gas_prices/", chain_name.as_ref(), event_type.as_ref());
        self.ctx.send_with_retry(self.ctx.get(&path)).await
    }

    /// Get address activity across chains.
    #[deprecated(note = "Use AllChainsService::get_address_activity instead")]
    pub async fn get_address_activity(
        &self, address: &str,
    ) -> Result<AddressActivityResponse, Error> {
        let path = format!("/v1/address/{}/activity/", address);
        self.ctx.send_with_retry(self.ctx.get(&path)).await
    }
}
