use crate::Error;
use crate::http::query::QueryParams;
use crate::models::pricing::*;
use crate::services::ServiceContext;
use std::sync::Arc;

/// Options for token pricing queries.
#[derive(Debug, Clone, Default)]
pub struct PricingOptions {
    pub from: Option<String>,
    pub to: Option<String>,
    pub prices_at_asc: Option<bool>,
    pub quote_currency: Option<String>,
}

impl PricingOptions {
    pub fn new() -> Self { Self::default() }
    pub fn from<S: Into<String>>(mut self, v: S) -> Self { self.from = Some(v.into()); self }
    pub fn to<S: Into<String>>(mut self, v: S) -> Self { self.to = Some(v.into()); self }
    pub fn prices_at_asc(mut self, v: bool) -> Self { self.prices_at_asc = Some(v); self }
    pub fn quote_currency<S: Into<String>>(mut self, c: S) -> Self { self.quote_currency = Some(c.into()); self }
}

impl QueryParams for PricingOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.from { builder = builder.query(&[("from", v)]); }
        if let Some(v) = self.to { builder = builder.query(&[("to", v)]); }
        if let Some(v) = self.prices_at_asc { builder = builder.query(&[("prices-at-asc", v.to_string())]); }
        if let Some(v) = self.quote_currency { builder = builder.query(&[("quote-currency", v)]); }
        builder
    }
}

/// Service for pricing API endpoints.
pub struct PricingService {
    ctx: Arc<ServiceContext>,
}

impl PricingService {
    pub(crate) fn new(ctx: Arc<ServiceContext>) -> Self {
        Self { ctx }
    }

    /// Get historical token prices.
    pub async fn get_token_prices(
        &self,
        chain_name: impl AsRef<str>,
        quote_currency: impl AsRef<str>,
        contract_address: &str,
        options: Option<PricingOptions>,
    ) -> Result<TokenPricesResponse, Error> {
        let path = format!(
            "/v1/pricing/historical_by_addresses_v2/{}/{}/{}/",
            chain_name.as_ref(), quote_currency.as_ref(), contract_address
        );
        let builder = self.ctx.get(&path);
        let builder = match options { Some(o) => o.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get pool spot prices.
    pub async fn get_pool_spot_prices(
        &self,
        chain_name: impl AsRef<str>,
        contract_address: &str,
    ) -> Result<PoolSpotPricesResponse, Error> {
        let path = format!(
            "/v1/pricing/spot_prices/{}/pools/{}/",
            chain_name.as_ref(), contract_address
        );
        self.ctx.send_with_retry(self.ctx.get(&path)).await
    }
}
