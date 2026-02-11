use crate::Error;
use crate::http::query::QueryParams;
use crate::models::nfts::*;
use crate::services::ServiceContext;
use std::sync::Arc;

/// Options for NFT queries.
#[derive(Debug, Clone, Default)]
pub struct NftOptions {
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
    pub quote_currency: Option<String>,
    pub with_metadata: Option<bool>,
    pub no_spam: Option<bool>,
    pub with_uncached: Option<bool>,
    pub traits_filter: Option<String>,
    pub values_filter: Option<String>,
    pub no_nft_asset_metadata: Option<bool>,
}

impl NftOptions {
    pub fn new() -> Self { Self::default() }
    pub fn page_number(mut self, v: u32) -> Self { self.page_number = Some(v); self }
    pub fn page_size(mut self, v: u32) -> Self { self.page_size = Some(v); self }
    pub fn quote_currency<S: Into<String>>(mut self, c: S) -> Self { self.quote_currency = Some(c.into()); self }
    pub fn with_metadata(mut self, v: bool) -> Self { self.with_metadata = Some(v); self }
    pub fn no_spam(mut self, v: bool) -> Self { self.no_spam = Some(v); self }
    pub fn with_uncached(mut self, v: bool) -> Self { self.with_uncached = Some(v); self }
    pub fn traits_filter<S: Into<String>>(mut self, v: S) -> Self { self.traits_filter = Some(v.into()); self }
    pub fn values_filter<S: Into<String>>(mut self, v: S) -> Self { self.values_filter = Some(v.into()); self }
    pub fn no_nft_asset_metadata(mut self, v: bool) -> Self { self.no_nft_asset_metadata = Some(v); self }
}

impl QueryParams for NftOptions {
    fn apply_to(self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(v) = self.page_number { builder = builder.query(&[("page-number", v.to_string())]); }
        if let Some(v) = self.page_size { builder = builder.query(&[("page-size", v.to_string())]); }
        if let Some(v) = self.quote_currency { builder = builder.query(&[("quote-currency", v)]); }
        if let Some(v) = self.with_metadata { builder = builder.query(&[("with-metadata", v.to_string())]); }
        if let Some(v) = self.no_spam { builder = builder.query(&[("no-spam", v.to_string())]); }
        if let Some(v) = self.with_uncached { builder = builder.query(&[("with-uncached", v.to_string())]); }
        if let Some(v) = self.traits_filter { builder = builder.query(&[("traits-filter", v)]); }
        if let Some(v) = self.values_filter { builder = builder.query(&[("values-filter", v)]); }
        if let Some(v) = self.no_nft_asset_metadata { builder = builder.query(&[("no-nft-asset-metadata", v.to_string())]); }
        builder
    }
}

/// Service for NFT-related API endpoints.
pub struct NftService {
    ctx: Arc<ServiceContext>,
}

impl NftService {
    pub(crate) fn new(ctx: Arc<ServiceContext>) -> Self {
        Self { ctx }
    }

    /// Get NFTs owned by an address.
    pub async fn get_nfts_for_address(
        &self, chain_name: impl AsRef<str>, address: &str, options: Option<NftOptions>,
    ) -> Result<NftsResponse, Error> {
        let path = format!("/v1/{}/address/{}/balances_nft/", chain_name.as_ref(), address);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(o) => o.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get metadata for a specific NFT.
    pub async fn get_nft_metadata(
        &self, chain_name: impl AsRef<str>, contract_address: &str, token_id: &str,
    ) -> Result<NftMetadataResponse, Error> {
        let path = format!("/v1/{}/tokens/{}/nft_metadata/{}/", chain_name.as_ref(), contract_address, token_id);
        let builder = self.ctx.get(&path);
        self.ctx.send_with_retry(builder).await
    }

    /// Get all NFTs from a specific collection (token IDs).
    pub async fn get_nfts_for_collection(
        &self, chain_name: impl AsRef<str>, contract_address: &str, options: Option<NftOptions>,
    ) -> Result<NftsResponse, Error> {
        let path = format!("/v1/{}/tokens/{}/nft_token_ids/", chain_name.as_ref(), contract_address);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(o) => o.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get NFT owners for a collection.
    pub async fn get_nft_owners_for_collection(
        &self, chain_name: impl AsRef<str>, contract_address: &str, options: Option<NftOptions>,
    ) -> Result<NftsResponse, Error> {
        let path = format!("/v1/{}/tokens/{}/nft_token_owners/", chain_name.as_ref(), contract_address);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(o) => o.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get chain collections for NFTs.
    pub async fn get_chain_collections(
        &self, chain_name: impl AsRef<str>, options: Option<NftOptions>,
    ) -> Result<ChainCollectionsResponse, Error> {
        let path = format!("/v1/{}/nft/collections/", chain_name.as_ref());
        let builder = self.ctx.get(&path);
        let builder = match options { Some(o) => o.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get token IDs for a contract with metadata.
    pub async fn get_token_ids_for_contract_with_metadata(
        &self, chain_name: impl AsRef<str>, contract_address: &str, options: Option<NftOptions>,
    ) -> Result<NftsResponse, Error> {
        let path = format!("/v1/{}/nft/{}/metadata/", chain_name.as_ref(), contract_address);
        let builder = self.ctx.get(&path);
        let builder = match options { Some(o) => o.apply_to(builder), None => builder };
        self.ctx.send_with_retry(builder).await
    }

    /// Get NFT transactions for a contract and token ID.
    pub async fn get_nft_transactions_for_contract_token_id(
        &self, chain_name: impl AsRef<str>, contract_address: &str, token_id: &str,
    ) -> Result<NftTransactionsResponse, Error> {
        let path = format!("/v1/{}/tokens/{}/nft_transactions/{}/", chain_name.as_ref(), contract_address, token_id);
        let builder = self.ctx.get(&path);
        self.ctx.send_with_retry(builder).await
    }

    /// Get traits for a collection.
    pub async fn get_traits_for_collection(
        &self, chain_name: impl AsRef<str>, contract_address: &str,
    ) -> Result<TraitsResponse, Error> {
        let path = format!("/v1/{}/nft/{}/traits/", chain_name.as_ref(), contract_address);
        let builder = self.ctx.get(&path);
        self.ctx.send_with_retry(builder).await
    }

    /// Get attributes for a trait in a collection.
    pub async fn get_attributes_for_trait_in_collection(
        &self, chain_name: impl AsRef<str>, contract_address: &str, trait_name: &str,
    ) -> Result<AttributesResponse, Error> {
        let path = format!("/v1/{}/nft/{}/traits/{}/attributes/", chain_name.as_ref(), contract_address, trait_name);
        let builder = self.ctx.get(&path);
        self.ctx.send_with_retry(builder).await
    }

    /// Get collection traits summary.
    pub async fn get_collection_traits_summary(
        &self, chain_name: impl AsRef<str>, contract_address: &str,
    ) -> Result<TraitsSummaryResponse, Error> {
        let path = format!("/v1/{}/nft/{}/traits_summary/", chain_name.as_ref(), contract_address);
        let builder = self.ctx.get(&path);
        self.ctx.send_with_retry(builder).await
    }

    /// Get historical floor prices for an NFT collection.
    pub async fn get_historical_floor_prices(
        &self, chain_name: impl AsRef<str>, contract_address: &str,
    ) -> Result<FloorPricesResponse, Error> {
        let path = format!("/v1/{}/nft_market/{}/floor_price/", chain_name.as_ref(), contract_address);
        let builder = self.ctx.get(&path);
        self.ctx.send_with_retry(builder).await
    }

    /// Get historical volume for an NFT collection.
    pub async fn get_historical_volume(
        &self, chain_name: impl AsRef<str>, contract_address: &str,
    ) -> Result<VolumeResponse, Error> {
        let path = format!("/v1/{}/nft_market/{}/volume/", chain_name.as_ref(), contract_address);
        let builder = self.ctx.get(&path);
        self.ctx.send_with_retry(builder).await
    }

    /// Get historical sales count for an NFT collection.
    pub async fn get_historical_sales_count(
        &self, chain_name: impl AsRef<str>, contract_address: &str,
    ) -> Result<SalesCountResponse, Error> {
        let path = format!("/v1/{}/nft_market/{}/sale_count/", chain_name.as_ref(), contract_address);
        let builder = self.ctx.get(&path);
        self.ctx.send_with_retry(builder).await
    }

    /// Check ownership in an NFT collection.
    pub async fn check_ownership_in_nft(
        &self, chain_name: impl AsRef<str>, address: &str, contract_address: &str,
    ) -> Result<OwnershipCheckResponse, Error> {
        let path = format!("/v1/{}/address/{}/collection/{}/", chain_name.as_ref(), address, contract_address);
        let builder = self.ctx.get(&path);
        self.ctx.send_with_retry(builder).await
    }

    /// Check ownership in an NFT for a specific token ID.
    pub async fn check_ownership_in_nft_for_token_id(
        &self, chain_name: impl AsRef<str>, address: &str, contract_address: &str, token_id: &str,
    ) -> Result<OwnershipCheckResponse, Error> {
        let path = format!("/v1/{}/address/{}/collection/{}/token/{}/", chain_name.as_ref(), address, contract_address, token_id);
        let builder = self.ctx.get(&path);
        self.ctx.send_with_retry(builder).await
    }
}
