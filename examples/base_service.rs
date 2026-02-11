use goldrush_sdk::{GoldRushClient, ClientConfig, Chain};
use goldrush_sdk::services::base_service::{BlockHeightsOptions, LogEventsByAddressOptions};
use std::env;

/// Example demonstrating base service endpoints (blocks, logs, chains).
///
/// Usage:
///   GOLDRUSH_API_KEY=your_api_key cargo run --example base_service

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GOLDRUSH_API_KEY")
        .expect("GOLDRUSH_API_KEY environment variable is required");

    let client = GoldRushClient::new(api_key, ClientConfig::default())?;

    // Example 1: Get all supported chains
    println!("Fetching all supported chains...");
    match client.base_service().get_all_chains().await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} supported chains", data.items.len());
                for chain in data.items.iter().take(10) {
                    println!("  - {} (ID: {:?})", chain.name.as_deref().unwrap_or("?"), chain.chain_id);
                }
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    // Example 2: Get a specific block
    println!("\nFetching block 18000000 on Ethereum...");
    match client.base_service().get_block(Chain::EthereumMainnet, "18000000").await {
        Ok(response) => {
            if let Some(data) = response.data {
                for block in data.items.iter().take(1) {
                    println!("Block height: {:?}", block.height);
                    println!("Block signed at: {:?}", block.signed_at);
                }
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    // Example 3: Get block heights by date range
    println!("\nFetching block heights for date range...");
    let opts = BlockHeightsOptions::new().page_size(5);
    match client.base_service().get_block_heights(Chain::EthereumMainnet, "2024-01-01", "2024-01-02", Some(opts)).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} blocks in range", data.items.len());
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    // Example 4: Get log events by contract address
    println!("\nFetching log events for USDC contract...");
    let log_opts = LogEventsByAddressOptions::new()
        .starting_block(18000000)
        .ending_block(18000010)
        .page_size(5);
    match client.base_service().get_log_events_by_address(
        Chain::EthereumMainnet,
        "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", // USDC
        Some(log_opts)
    ).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} log events", data.items.len());
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    println!("\nBase service examples completed!");
    Ok(())
}
