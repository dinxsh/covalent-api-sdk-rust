use goldrush_sdk::{GoldRushClient, ClientConfig, Chain, QuoteCurrency, PricingOptions};
use std::env;

/// Example demonstrating pricing service endpoints.
///
/// Usage:
///   GOLDRUSH_API_KEY=your_api_key cargo run --example pricing

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GOLDRUSH_API_KEY")
        .expect("GOLDRUSH_API_KEY environment variable is required");

    let client = GoldRushClient::new(api_key, ClientConfig::default())?;

    // Example 1: Get historical token prices
    println!("Fetching historical USDC prices...");
    let opts = PricingOptions::new()
        .from("2024-01-01")
        .to("2024-01-31");

    match client.pricing_service().get_token_prices(
        Chain::EthereumMainnet,
        QuoteCurrency::USD,
        "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", // USDC
        Some(opts)
    ).await {
        Ok(response) => {
            if let Some(items) = response.data {
                println!("Found {} price items", items.len());
                for item in items.iter().take(3) {
                    println!("  Contract: {:?}", item.contract_address);
                    if let Some(prices) = &item.prices {
                        println!("  Price points: {}", prices.len());
                    }
                }
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    // Example 2: Get pool spot prices
    println!("\nFetching pool spot prices...");
    match client.pricing_service().get_pool_spot_prices(
        Chain::EthereumMainnet,
        "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640", // USDC/WETH Uniswap V3
    ).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} pool price items", data.items.len());
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    println!("\nPricing examples completed!");
    Ok(())
}
