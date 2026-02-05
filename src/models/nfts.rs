use serde::Deserialize;

/// Represents an NFT item returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct NftItem {
    /// The contract address of the NFT collection.
    pub contract_address: String,
    
    /// The token ID within the collection.
    pub token_id: String,
    
    /// The owner's balance of this NFT (usually 1 for ERC-721, can be >1 for ERC-1155).
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
    /// Token URI pointing to metadata.
    pub token_uri: Option<String>,
    
    /// External metadata loaded from token URI.
    pub external_data: Option<ExternalNftData>,
    
    /// Original owner of the NFT.
    pub original_owner: Option<String>,
    
    /// Current owner of the NFT.
    pub current_owner: Option<String>,
    
    /// Asset details.
    pub asset_original_url: Option<String>,
    pub asset_cached_url: Option<String>,
    pub asset_file_extension: Option<String>,
    pub asset_mime_type: Option<String>,
}

/// External NFT metadata loaded from token URI.
#[derive(Debug, Clone, Deserialize)]
pub struct ExternalNftData {
    /// Name of the NFT.
    pub name: Option<String>,
    
    /// Description of the NFT.
    pub description: Option<String>,
    
    /// Image URL.
    pub image: Option<String>,
    
    /// Animation URL (for video/interactive content).
    pub animation_url: Option<String>,
    
    /// External URL for more information.
    pub external_url: Option<String>,
    
    /// Attributes/traits of the NFT.
    pub attributes: Option<Vec<NftAttribute>>,
}

/// An attribute/trait of an NFT.
#[derive(Debug, Clone, Deserialize)]
pub struct NftAttribute {
    /// The trait type/name.
    pub trait_type: Option<String>,
    
    /// The trait value.
    pub value: Option<serde_json::Value>,
    
    /// Display type for the trait.
    pub display_type: Option<String>,
}

/// Container for NFT items.
#[derive(Debug, Clone, Deserialize)]
pub struct NftsData {
    /// The address these NFTs belong to.
    pub address: Option<String>,
    
    /// The chain ID.
    pub chain_id: Option<u64>,
    
    /// The chain name.
    pub chain_name: Option<String>,
    
    /// List of NFT items.
    pub items: Vec<NftItem>,
}

/// Response structure for NFT queries.
pub type NftsResponse = crate::models::ApiResponse<NftsData>;

/// Represents detailed NFT metadata for a specific token.
#[derive(Debug, Clone, Deserialize)]
pub struct NftMetadataItem {
    /// The contract address.
    pub contract_address: String,
    
    /// The token ID.
    pub token_id: String,
    
    /// The token URI.
    pub token_uri: Option<String>,
    
    /// The loaded metadata.
    pub metadata: Option<serde_json::Value>,
    
    /// External data.
    pub external_data: Option<ExternalNftData>,
    
    /// Asset information.
    pub asset_original_url: Option<String>,
    pub asset_cached_url: Option<String>,
    pub asset_file_extension: Option<String>,
    pub asset_mime_type: Option<String>,
}

/// Response structure for NFT metadata queries.
pub type NftMetadataResponse = crate::models::ApiResponse<Vec<NftMetadataItem>>;