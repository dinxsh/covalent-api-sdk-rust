use serde::Deserialize;

/// Represents an NFT item returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct NftItem {
    /// The contract address of the NFT collection.
    pub contract_address: String,

    /// The token ID within the collection.
    pub token_id: String,

    /// The owner's balance of this NFT.
    pub token_balance: Option<String>,

    /// URL for the token metadata.
    pub token_url: Option<String>,

    /// Collection name.
    pub contract_name: Option<String>,

    /// Collection symbol/ticker.
    pub contract_ticker_symbol: Option<String>,

    /// Whether this is an ERC-721 or ERC-1155 token.
    pub supports_erc: Option<Vec<String>>,

    /// External metadata for the NFT.
    pub nft_data: Option<NftMetadata>,
}

/// Metadata for an NFT token.
#[derive(Debug, Clone, Deserialize)]
pub struct NftMetadata {
    pub token_uri: Option<String>,
    pub external_data: Option<ExternalNftData>,
    pub original_owner: Option<String>,
    pub current_owner: Option<String>,
    pub asset_original_url: Option<String>,
    pub asset_cached_url: Option<String>,
    pub asset_file_extension: Option<String>,
    pub asset_mime_type: Option<String>,
}

/// External NFT metadata loaded from token URI.
#[derive(Debug, Clone, Deserialize)]
pub struct ExternalNftData {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub animation_url: Option<String>,
    pub external_url: Option<String>,
    pub attributes: Option<Vec<NftAttribute>>,
}

/// An attribute/trait of an NFT.
#[derive(Debug, Clone, Deserialize)]
pub struct NftAttribute {
    pub trait_type: Option<String>,
    pub value: Option<serde_json::Value>,
    pub display_type: Option<String>,
}

/// Container for NFT items.
#[derive(Debug, Clone, Deserialize)]
pub struct NftsData {
    pub address: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<NftItem>,
}

/// Response structure for NFT queries.
pub type NftsResponse = crate::models::ApiResponse<NftsData>;

/// Represents detailed NFT metadata for a specific token.
#[derive(Debug, Clone, Deserialize)]
pub struct NftMetadataItem {
    pub contract_address: String,
    pub token_id: String,
    pub token_uri: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub external_data: Option<ExternalNftData>,
    pub asset_original_url: Option<String>,
    pub asset_cached_url: Option<String>,
    pub asset_file_extension: Option<String>,
    pub asset_mime_type: Option<String>,
}

/// Response structure for NFT metadata queries.
pub type NftMetadataResponse = crate::models::ApiResponse<Vec<NftMetadataItem>>;

// --- Extended models for additional NFT endpoints ---

/// Represents an NFT collection item in chain collection listings.
#[derive(Debug, Clone, Deserialize)]
pub struct ChainCollectionItem {
    pub contract_address: Option<String>,
    pub contract_name: Option<String>,
    pub contract_ticker_symbol: Option<String>,
    pub token_total_supply: Option<String>,
    pub floor_price_quote: Option<f64>,
    pub floor_price_native_quote: Option<f64>,
    pub market_cap_quote: Option<f64>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Container for chain collection items.
#[derive(Debug, Clone, Deserialize)]
pub struct ChainCollectionsData {
    pub updated_at: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<ChainCollectionItem>,
}

/// Response structure for chain collection queries.
pub type ChainCollectionsResponse = crate::models::ApiResponse<ChainCollectionsData>;

/// Represents an NFT transaction item.
#[derive(Debug, Clone, Deserialize)]
pub struct NftTransactionItem {
    pub block_signed_at: Option<String>,
    pub block_height: Option<u64>,
    pub tx_hash: Option<String>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub value: Option<String>,
    pub token_id: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Container for NFT transaction items.
#[derive(Debug, Clone, Deserialize)]
pub struct NftTransactionsData {
    pub updated_at: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<NftTransactionItem>,
}

/// Response structure for NFT transaction queries.
pub type NftTransactionsResponse = crate::models::ApiResponse<NftTransactionsData>;

/// Represents a trait item for a collection.
#[derive(Debug, Clone, Deserialize)]
pub struct TraitItem {
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Container for traits data.
#[derive(Debug, Clone, Deserialize)]
pub struct TraitsData {
    pub items: Vec<TraitItem>,
}

/// Response structure for traits queries.
pub type TraitsResponse = crate::models::ApiResponse<TraitsData>;

/// Represents an attribute item for a trait.
#[derive(Debug, Clone, Deserialize)]
pub struct AttributeItem {
    pub trait_type: Option<String>,
    pub values: Option<Vec<AttributeValue>>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// An attribute value with count information.
#[derive(Debug, Clone, Deserialize)]
pub struct AttributeValue {
    pub value: Option<serde_json::Value>,
    pub count: Option<u64>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Container for attributes data.
#[derive(Debug, Clone, Deserialize)]
pub struct AttributesData {
    pub items: Vec<AttributeItem>,
}

/// Response structure for attribute queries.
pub type AttributesResponse = crate::models::ApiResponse<AttributesData>;

/// Represents a traits summary item.
#[derive(Debug, Clone, Deserialize)]
pub struct TraitsSummaryItem {
    pub name: Option<String>,
    pub value_count: Option<u64>,
    pub unique_values: Option<u64>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Container for traits summary data.
#[derive(Debug, Clone, Deserialize)]
pub struct TraitsSummaryData {
    pub items: Vec<TraitsSummaryItem>,
}

/// Response structure for traits summary queries.
pub type TraitsSummaryResponse = crate::models::ApiResponse<TraitsSummaryData>;

/// Represents a floor price item.
#[derive(Debug, Clone, Deserialize)]
pub struct FloorPriceItem {
    pub date: Option<String>,
    pub floor_price_quote: Option<f64>,
    pub floor_price_native_quote: Option<f64>,
    pub pretty_floor_price_quote: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Container for floor prices data.
#[derive(Debug, Clone, Deserialize)]
pub struct FloorPricesData {
    pub updated_at: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<FloorPriceItem>,
}

/// Response structure for floor price queries.
pub type FloorPricesResponse = crate::models::ApiResponse<FloorPricesData>;

/// Represents a volume item.
#[derive(Debug, Clone, Deserialize)]
pub struct VolumeItem {
    pub date: Option<String>,
    pub volume_quote: Option<f64>,
    pub volume_native_quote: Option<f64>,
    pub pretty_volume_quote: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Container for volume data.
#[derive(Debug, Clone, Deserialize)]
pub struct VolumeData {
    pub updated_at: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<VolumeItem>,
}

/// Response structure for volume queries.
pub type VolumeResponse = crate::models::ApiResponse<VolumeData>;

/// Represents a sales count item.
#[derive(Debug, Clone, Deserialize)]
pub struct SalesCountItem {
    pub date: Option<String>,
    pub sale_count: Option<u64>,
    pub pretty_sale_count: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Container for sales count data.
#[derive(Debug, Clone, Deserialize)]
pub struct SalesCountData {
    pub updated_at: Option<String>,
    pub chain_id: Option<u64>,
    pub chain_name: Option<String>,
    pub items: Vec<SalesCountItem>,
}

/// Response structure for sales count queries.
pub type SalesCountResponse = crate::models::ApiResponse<SalesCountData>;

/// Represents an ownership check item.
#[derive(Debug, Clone, Deserialize)]
pub struct OwnershipCheckItem {
    pub token_id: Option<String>,
    pub token_balance: Option<String>,
    pub contract_address: Option<String>,
    pub contract_name: Option<String>,
    pub contract_ticker_symbol: Option<String>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Container for ownership check data.
#[derive(Debug, Clone, Deserialize)]
pub struct OwnershipCheckData {
    pub address: Option<String>,
    pub is_owner: Option<bool>,
    pub items: Vec<OwnershipCheckItem>,
}

/// Response structure for ownership check queries.
pub type OwnershipCheckResponse = crate::models::ApiResponse<OwnershipCheckData>;
