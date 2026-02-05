use goldrush_sdk::{GoldRushClient, ClientConfig, NftOptions};
use std::env;

/// Example demonstrating how to fetch NFTs for a wallet address.
/// 
/// Usage:
///   GOLDRUSH_API_KEY=your_api_key cargo run --example nfts
///   GOLDRUSH_API_KEY=your_api_key cargo run --example nfts -- 0x742d35Cc6634C0532925a3b8D186dC8b7B3e4fe
/// 
/// If no address is provided, uses a default demo address.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = env::var("GOLDRUSH_API_KEY")
        .expect("GOLDRUSH_API_KEY environment variable is required");

    // Get address from command line args, or use default
    let args: Vec<String> = env::args().collect();
    let address = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de");

    println!("🔗 Creating GoldRush client...");
    let client = GoldRushClient::new(api_key, ClientConfig::default())?;

    // Example 1: Basic NFT query
    println!("\n🖼️  Fetching NFTs for address: {}", address);
    
    let nfts = client
        .get_nfts_for_address("eth-mainnet", address, None)
        .await?;

    if let Some(data) = nfts.data {
        println!("Found {} NFTs", data.items.len());
        
        for (i, nft) in data.items.iter().take(5).enumerate() {
            let collection = nft.contract_name.as_deref()
                .or(nft.contract_ticker_symbol.as_deref())
                .unwrap_or("Unknown Collection");
            let balance = nft.token_balance.as_deref().unwrap_or("1");
            
            println!(
                "  {}: {} #{} ({}x) - {}",
                i + 1,
                collection,
                nft.token_id,
                balance,
                &nft.contract_address[..8]
            );
        }
        
        if data.items.len() > 5 {
            println!("  ... and {} more NFTs", data.items.len() - 5);
        }
    }

    // Example 2: Filtered NFT query with metadata
    println!("\n🎯 Fetching NFTs with metadata (no spam)...");
    
    let options = NftOptions::new()
        .page_size(5)
        .with_metadata(true)
        .no_spam(true);

    let filtered_nfts = client
        .get_nfts_for_address("eth-mainnet", address, Some(options))
        .await?;

    if let Some(data) = filtered_nfts.data {
        println!("Found {} non-spam NFTs with metadata", data.items.len());
        
        for (i, nft) in data.items.iter().enumerate() {
            let collection = nft.contract_name.as_deref().unwrap_or("Unknown");
            println!("  {}: {} #{}", i + 1, collection, nft.token_id);
            
            if let Some(nft_data) = &nft.nft_data {
                if let Some(external_data) = &nft_data.external_data {
                    if let Some(name) = &external_data.name {
                        println!("    Name: {}", name);
                    }
                    if let Some(description) = &external_data.description {
                        let short_desc = if description.len() > 100 {
                            format!("{}...", &description[..100])
                        } else {
                            description.clone()
                        };
                        println!("    Description: {}", short_desc);
                    }
                    
                    if let Some(attributes) = &external_data.attributes {
                        println!("    Attributes: {} traits", attributes.len());
                        for attr in attributes.iter().take(3) {
                            if let (Some(trait_type), Some(value)) = (&attr.trait_type, &attr.value) {
                                println!("      {}: {}", trait_type, value);
                            }
                        }
                    }
                }
            }
            println!();
        }
    }

    // Example 3: Get metadata for a specific NFT
    println!("\n🔍 Getting metadata for a specific NFT (Bored Ape #1)...");
    
    match client
        .get_nft_metadata(
            "eth-mainnet",
            "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d", // BAYC contract
            "1"
        )
        .await
    {
        Ok(response) => {
            if let Some(metadata_items) = response.data {
                for metadata in metadata_items {
                    println!("NFT Metadata:");
                    println!("  Contract: {}", metadata.contract_address);
                    println!("  Token ID: {}", metadata.token_id);
                    
                    if let Some(uri) = &metadata.token_uri {
                        println!("  Token URI: {}", uri);
                    }
                    
                    if let Some(external_data) = &metadata.external_data {
                        if let Some(name) = &external_data.name {
                            println!("  Name: {}", name);
                        }
                        if let Some(image) = &external_data.image {
                            println!("  Image: {}", image);
                        }
                    }
                    
                    if let Some(cached_url) = &metadata.asset_cached_url {
                        println!("  Cached image: {}", cached_url);
                    }
                }
            }
        }
        Err(e) => {
            println!("Could not fetch NFT metadata: {:?}", e);
        }
    }

    // Example 4: Get NFTs for a collection
    println!("\n📚 Getting NFTs for Bored Ape Yacht Club collection...");
    
    let collection_options = NftOptions::new().page_size(5);
    
    match client
        .get_nfts_for_collection(
            "eth-mainnet",
            "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d",
            Some(collection_options)
        )
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Collection has {} NFTs (showing first 5)", data.items.len());
                for nft in data.items.iter() {
                    println!("  Token #{}: {}", nft.token_id, nft.contract_address);
                }
            }
        }
        Err(e) => {
            println!("Could not fetch collection NFTs: {:?}", e);
        }
    }

    // Example 5: Get NFT owners for a collection
    println!("\n👥 Getting owners for Bored Ape Yacht Club...");
    
    match client
        .get_nft_owners_for_collection(
            "eth-mainnet",
            "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d",
            Some(NftOptions::new().page_size(5))
        )
        .await
    {
        Ok(response) => {
            println!("Got NFT owners response (endpoint structure may vary)");
        }
        Err(e) => {
            println!("Could not fetch NFT owners: {:?}", e);
        }
    }

    // Example 6: NFTs on different chains
    println!("\n🌐 Checking NFTs on Polygon...");
    
    let polygon_options = NftOptions::new()
        .page_size(3)
        .no_spam(true);
    
    match client
        .get_nfts_for_address("matic-mainnet", address, Some(polygon_options))
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} NFTs on Polygon", data.items.len());
                for nft in data.items.iter() {
                    let collection = nft.contract_name.as_deref().unwrap_or("Unknown");
                    println!("  - {} #{}", collection, nft.token_id);
                }
            } else {
                println!("No NFTs found on Polygon");
            }
        }
        Err(e) => {
            println!("Polygon NFTs error: {:?}", e);
        }
    }

    // Example 7: Group NFTs by collection
    if let Ok(response) = client
        .get_nfts_for_address("eth-mainnet", address, Some(NftOptions::new().page_size(20)))
        .await
    {
        if let Some(data) = response.data {
            println!("\n📊 NFT Collection Summary:");
            
            let mut collections = std::collections::HashMap::new();
            for nft in &data.items {
                let collection_name = nft.contract_name.as_deref()
                    .unwrap_or_else(|| nft.contract_ticker_symbol.as_deref().unwrap_or("Unknown"));
                *collections.entry(collection_name).or_insert(0) += 1;
            }
            
            let mut collection_counts: Vec<_> = collections.into_iter().collect();
            collection_counts.sort_by(|a, b| b.1.cmp(&a.1));
            
            for (collection, count) in collection_counts.iter().take(10) {
                println!("  {}: {} NFTs", collection, count);
            }
        }
    }

    println!("\n✅ NFT examples completed!");
    Ok(())
}