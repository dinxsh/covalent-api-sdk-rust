use goldrush_sdk::{GoldRushClient, ClientConfig, Chain};
use std::env;

/// Example demonstrating NFT market data endpoints (floor prices, volume, traits).
///
/// Usage:
///   GOLDRUSH_API_KEY=your_api_key cargo run --example nft_market

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GOLDRUSH_API_KEY")
        .expect("GOLDRUSH_API_KEY environment variable is required");

    let client = GoldRushClient::new(api_key, ClientConfig::default())?;
    let bayc = "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d";

    // Example 1: Historical floor prices
    println!("Fetching BAYC historical floor prices...");
    match client.nft_service().get_historical_floor_prices(Chain::EthereumMainnet, bayc).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} floor price data points", data.items.len());
                for item in data.items.iter().take(5) {
                    println!("  Date: {:?}, Price: {:?}",
                        item.date, item.floor_price_quote);
                }
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    // Example 2: Historical volume
    println!("\nFetching BAYC historical volume...");
    match client.nft_service().get_historical_volume(Chain::EthereumMainnet, bayc).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} volume data points", data.items.len());
                for item in data.items.iter().take(5) {
                    println!("  Date: {:?}, Volume: {:?}",
                        item.date, item.volume_quote);
                }
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    // Example 3: Traits summary
    println!("\nFetching BAYC collection traits summary...");
    match client.nft_service().get_collection_traits_summary(Chain::EthereumMainnet, bayc).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} trait categories", data.items.len());
                for item in data.items.iter().take(5) {
                    println!("  Trait: {:?}, Unique values: {:?}",
                        item.name, item.unique_values);
                }
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    // Example 4: Sales count
    println!("\nFetching BAYC historical sales count...");
    match client.nft_service().get_historical_sales_count(Chain::EthereumMainnet, bayc).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} sales count data points", data.items.len());
                for item in data.items.iter().take(5) {
                    println!("  Date: {:?}, Sales: {:?}",
                        item.date, item.sale_count);
                }
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    println!("\nNFT market examples completed!");
    Ok(())
}
