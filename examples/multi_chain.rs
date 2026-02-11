use goldrush_sdk::{GoldRushClient, ClientConfig, Chain, MultiChainBalancesOptions};
use std::env;

/// Example demonstrating cross-chain endpoints.
///
/// Usage:
///   GOLDRUSH_API_KEY=your_api_key cargo run --example multi_chain

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GOLDRUSH_API_KEY")
        .expect("GOLDRUSH_API_KEY environment variable is required");

    let client = GoldRushClient::new(api_key, ClientConfig::default())?;
    let address = "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de";

    // Example 1: Get address activity across all chains
    println!("Fetching address activity across all chains...");
    match client.all_chains_service().get_address_activity(address, None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Address is active on {} chains", data.items.len());
                for item in data.items.iter().take(10) {
                    println!("  - Chain: {:?} (ID: {:?})",
                        item.chain_name, item.chain_id);
                }
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    // Example 2: Multi-chain balances
    println!("\nFetching multi-chain balances...");
    let opts = MultiChainBalancesOptions::new()
        .quote_currency("USD");

    match client.all_chains_service().get_multi_chain_balances(address, Some(opts)).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} balance items across chains", data.items.len());
                for item in data.items.iter().take(10) {
                    println!("  Chain: {:?}, Token: {:?} ({:?})",
                        item.chain_name, item.contract_ticker_symbol, item.quote);
                }
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    // Example 3: Compare balances across chains using Chain enum
    println!("\nComparing balances on Ethereum vs Polygon...");
    let chains = vec![
        (Chain::EthereumMainnet, "Ethereum"),
        (Chain::PolygonMainnet, "Polygon"),
        (Chain::ArbitrumMainnet, "Arbitrum"),
    ];

    for (chain, name) in chains {
        match client.balance_service()
            .get_token_balances_for_wallet_address(chain, address, None)
            .await
        {
            Ok(response) => {
                if let Some(data) = response.data {
                    let total: f64 = data.items.iter()
                        .filter_map(|item| item.quote)
                        .sum();
                    println!("  {}: {} tokens, total value: ${:.2}", name, data.items.len(), total);
                }
            }
            Err(e) => println!("  {}: Error - {:?}", name, e),
        }
    }

    println!("\nMulti-chain examples completed!");
    Ok(())
}
