use crate::Error;
use crate::http::query::QueryParams;
use crate::models::transactions::{TransactionsResponse, TransactionResponse, TransactionSummaryResponse, TimeBucketResponse};
use crate::services::ServiceContext;
use std::sync::Arc;

/// Options for transaction queries.
#[derive(Debug, Clone, Default)]
pub struct TxOptions {
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
    pub quote_currency: Option<String>,
    pub no_logs: Option<bool>,
    pub block_signed_at_asc: Option<bool>,
    pub with_internal: Option<bool>,
    pub with_state: Option<bool>,
    pub with_input_data: Option<bool>,
    pub starting_block: Option<u64>,
    pub ending_block: Option<u64>,
}

impl TxOptions {
    pub fn new() -> Self { Self::default() }
    pub fn page_number(mut self, v: u32) -> Self { self.page_number = Some(v); self }
    pub fn page_size(mut self, v: u32) -> Self { self.page_size = Some(v); self }
    pub fn quote_currency<S: Into<String>>(mut self, c: S) -> Self { self.quote_currency = Some(c.into()); self }
    pub fn no_logs(mut self, v: bool) -> Self { self.no_logs = Some(v); self }
    pub fn block_signed_at_asc(mut self, v: bool) -> Self { self.block_signed_at_asc = Some(v); self }
    pub fn with_internal(mut self, v: bool) -> Self { self.with_internal = Some(v); self }
    pub fn with_state(mut self, v: bool) -> Self { self.with_state = Some(v); self }
    pub fn with_input_data(mut self, v: bool) -> Self { self.with_input_data = Some(v); self }
    pub fn starting_block(mut self, v: u64) -> Self { self.starting_block = Some(v); self }
    pub fn ending_block(mut self, v: u64) -> Self { self.ending_block = Some(v); self }
}

impl QueryParams for TxOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.page_number { builder = builder.query(&[("page-number", v.to_string())]); }
        if let Some(v) = self.page_size { builder = builder.query(&[("page-size", v.to_string())]); }
        if let Some(v) = self.quote_currency { builder = builder.query(&[("quote-currency", v)]); }
        if let Some(v) = self.no_logs { builder = builder.query(&[("no-logs", v.to_string())]); }
        if let Some(v) = self.block_signed_at_asc { builder = builder.query(&[("block-signed-at-asc", v.to_string())]); }
        if let Some(v) = self.with_internal { builder = builder.query(&[("with-internal", v.to_string())]); }
        if let Some(v) = self.with_state { builder = builder.query(&[("with-state", v.to_string())]); }
        if let Some(v) = self.with_input_data { builder = builder.query(&[("with-input-data", v.to_string())]); }
        if let Some(v) = self.starting_block { builder = builder.query(&[("starting-block", v.to_string())]); }
        if let Some(v) = self.ending_block { builder = builder.query(&[("ending-block", v.to_string())]); }
        builder
    }
}

/// Options for single transaction queries.
#[derive(Debug, Clone, Default)]
pub struct SingleTxOptions {
    pub quote_currency: Option<String>,
    pub no_logs: Option<bool>,
    pub with_internal: Option<bool>,
    pub with_state: Option<bool>,
    pub with_input_data: Option<bool>,
}

impl SingleTxOptions {
    pub fn new() -> Self { Self::default() }
    pub fn quote_currency<S: Into<String>>(mut self, c: S) -> Self { self.quote_currency = Some(c.into()); self }
    pub fn no_logs(mut self, v: bool) -> Self { self.no_logs = Some(v); self }
    pub fn with_internal(mut self, v: bool) -> Self { self.with_internal = Some(v); self }
    pub fn with_state(mut self, v: bool) -> Self { self.with_state = Some(v); self }
    pub fn with_input_data(mut self, v: bool) -> Self { self.with_input_data = Some(v); self }
}

impl QueryParams for SingleTxOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.quote_currency { builder = builder.query(&[("quote-currency", v)]); }
        if let Some(v) = self.no_logs { builder = builder.query(&[("no-logs", v.to_string())]); }
        if let Some(v) = self.with_internal { builder = builder.query(&[("with-internal", v.to_string())]); }
        if let Some(v) = self.with_state { builder = builder.query(&[("with-state", v.to_string())]); }
        if let Some(v) = self.with_input_data { builder = builder.query(&[("with-input-data", v.to_string())]); }
        builder
    }
}

/// Options for transaction summary queries.
#[derive(Debug, Clone, Default)]
pub struct TransactionSummaryOptions {
    pub quote_currency: Option<String>,
}

impl TransactionSummaryOptions {
    pub fn new() -> Self { Self::default() }
    pub fn quote_currency<S: Into<String>>(mut self, c: S) -> Self { self.quote_currency = Some(c.into()); self }
}

impl QueryParams for TransactionSummaryOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.quote_currency { builder = builder.query(&[("quote-currency", v)]); }
        builder
    }
}

/// Options for time bucket transaction queries.
#[derive(Debug, Clone, Default)]
pub struct TimeBucketOptions {
    pub quote_currency: Option<String>,
    pub no_logs: Option<bool>,
}

impl TimeBucketOptions {
    pub fn new() -> Self { Self::default() }
    pub fn quote_currency<S: Into<String>>(mut self, c: S) -> Self { self.quote_currency = Some(c.into()); self }
    pub fn no_logs(mut self, v: bool) -> Self { self.no_logs = Some(v); self }
}

impl QueryParams for TimeBucketOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.quote_currency { builder = builder.query(&[("quote-currency", v)]); }
        if let Some(v) = self.no_logs { builder = builder.query(&[("no-logs", v.to_string())]); }
        builder
    }
}

/// Service for transaction-related API endpoints.
pub struct TransactionService {
    ctx: Arc<ServiceContext>,
}

impl TransactionService {
    pub(crate) fn new(ctx: Arc<ServiceContext>) -> Self {
        Self { ctx }
    }

    /// Get all transactions for an address (v3).
    pub async fn get_all_transactions_for_address(
        &self,
        chain_name: impl AsRef<str>,
        address: &str,
        options: Option<TxOptions>,
    ) -> Result<TransactionsResponse, Error> {
        let path = format!("/v1/{}/address/{}/transactions_v3/", chain_name.as_ref(), address);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(opts) => opts.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get a specific transaction by hash.
    pub async fn get_transaction(
        &self,
        chain_name: impl AsRef<str>,
        tx_hash: &str,
        options: Option<SingleTxOptions>,
    ) -> Result<TransactionResponse, Error> {
        let path = format!("/v1/{}/transaction_v2/{}/", chain_name.as_ref(), tx_hash);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(opts) => opts.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get transaction summary for an address.
    pub async fn get_transaction_summary(
        &self,
        chain_name: impl AsRef<str>,
        address: &str,
        options: Option<TransactionSummaryOptions>,
    ) -> Result<TransactionSummaryResponse, Error> {
        let path = format!("/v1/{}/address/{}/transactions_summary/", chain_name.as_ref(), address);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(opts) => opts.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get earliest transactions for an address.
    pub async fn get_earliest_transactions(
        &self,
        chain_name: impl AsRef<str>,
        address: &str,
        options: Option<TxOptions>,
    ) -> Result<TransactionsResponse, Error> {
        let path = format!("/v1/{}/bulk/transactions/{}/", chain_name.as_ref(), address);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(opts) => opts.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get paginated transactions for an address (v3, specific page).
    pub async fn get_paginated_transactions(
        &self,
        chain_name: impl AsRef<str>,
        address: &str,
        page: u32,
        options: Option<TxOptions>,
    ) -> Result<TransactionsResponse, Error> {
        let path = format!("/v1/{}/address/{}/transactions_v3/page/{}/", chain_name.as_ref(), address, page);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(opts) => opts.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get transactions in a time bucket.
    pub async fn get_time_bucket_transactions(
        &self,
        chain_name: impl AsRef<str>,
        address: &str,
        time_bucket: u32,
        options: Option<TimeBucketOptions>,
    ) -> Result<TimeBucketResponse, Error> {
        let path = format!("/v1/{}/bulk/transactions/{}/{}/", chain_name.as_ref(), address, time_bucket);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(opts) => opts.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get transactions for a block by page number.
    pub async fn get_transactions_for_block_by_page(
        &self,
        chain_name: impl AsRef<str>,
        block_height: u64,
        page: u32,
        options: Option<TxOptions>,
    ) -> Result<TransactionsResponse, Error> {
        let path = format!("/v1/{}/block/{}/transactions_v3/page/{}/", chain_name.as_ref(), block_height, page);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(opts) => opts.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get transactions for a block by block hash (v3).
    pub async fn get_transactions_for_block(
        &self,
        chain_name: impl AsRef<str>,
        block_hash: &str,
        options: Option<TxOptions>,
    ) -> Result<TransactionsResponse, Error> {
        let path = format!("/v1/{}/block_hash/{}/transactions_v3/", chain_name.as_ref(), block_hash);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(opts) => opts.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Deprecated: alias for get_paginated_transactions.
    #[deprecated(note = "Use get_paginated_transactions instead")]
    pub async fn get_transactions_for_address_v3(
        &self,
        chain_name: impl AsRef<str>,
        address: &str,
        page: u32,
        options: Option<TxOptions>,
    ) -> Result<TransactionsResponse, Error> {
        self.get_paginated_transactions(chain_name, address, page, options).await
    }
}
