use goldrush_sdk::{GoldRushClient, ClientConfig, Chain, BalancesOptions};
use std::env;

/// Example demonstrating how to fetch token balances for a wallet address.
///
/// Usage:
///   GOLDRUSH_API_KEY=your_api_key cargo run --example balances
///   GOLDRUSH_API_KEY=your_api_key cargo run --example balances -- 0x742d35Cc6634C0532925a3b8D186dC8b7B3e4fe
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

    println!("Creating GoldRush client...");
    let client = GoldRushClient::new(api_key, ClientConfig::default())?;

    // Example 1: Basic balance query using Chain enum
    println!("\nFetching token balances for address: {}", address);

    let basic_balances = client
        .balance_service()
        .get_token_balances_for_wallet_address(Chain::EthereumMainnet, address, None)
        .await?;

    if let Some(data) = basic_balances.data {
        println!("Found {} token balances", data.items.len());

        for (i, item) in data.items.iter().take(5).enumerate() {
            let symbol = item.contract_ticker_symbol.as_deref().unwrap_or("Unknown");
            let balance = &item.balance;
            let quote = item.quote.map(|q| format!("${:.2}", q)).unwrap_or_default();

            println!(
                "  {}: {} {} {}",
                i + 1,
                symbol,
                balance,
                quote
            );
        }

        if data.items.len() > 5 {
            println!("  ... and {} more tokens", data.items.len() - 5);
        }
    }

    // Example 2: Filtered balance query
    println!("\nFetching filtered balances (USD quotes, no spam, limited)...");

    let options = BalancesOptions::new()
        .quote_currency("USD")
        .no_spam(true)
        .page_size(10);

    let filtered_balances = client
        .balance_service()
        .get_token_balances_for_wallet_address(Chain::EthereumMainnet, address, Some(options))
        .await?;

    if let Some(data) = filtered_balances.data {
        println!("Found {} non-spam tokens with USD quotes", data.items.len());

        let mut total_value = 0.0;
        for item in &data.items {
            if let Some(quote) = item.quote {
                total_value += quote;
            }
        }

        println!("Total portfolio value: ${:.2}", total_value);

        // Show top tokens by value
        let mut tokens_with_value: Vec<_> = data.items.iter()
            .filter(|item| item.quote.unwrap_or(0.0) > 0.0)
            .collect();
        tokens_with_value.sort_by(|a, b|
            b.quote.partial_cmp(&a.quote).unwrap_or(std::cmp::Ordering::Equal)
        );

        println!("\nTop tokens by value:");
        for (i, token) in tokens_with_value.iter().take(5).enumerate() {
            let symbol = token.contract_ticker_symbol.as_deref().unwrap_or("Unknown");
            let quote = token.quote.unwrap();
            let percentage = (quote / total_value) * 100.0;

            println!(
                "  {}: {} - ${:.2} ({:.1}%)",
                i + 1,
                symbol,
                quote,
                percentage
            );
        }
    }

    // Example 3: Historical portfolio (if available)
    println!("\nTrying to fetch historical portfolio...");

    match client
        .balance_service()
        .get_historical_portfolio_for_wallet_address(Chain::EthereumMainnet, address, None)
        .await
    {
        Ok(portfolio) => {
            println!("Historical portfolio data received");
            if let Some(data) = portfolio.data {
                println!("Portfolio items: {}", data.items.len());
            }
        }
        Err(e) => {
            println!("Historical portfolio not available: {:?}", e);
        }
    }

    // Example 4: Different chain using Chain enum
    println!("\nChecking Polygon balances...");

    let polygon_options = BalancesOptions::new()
        .quote_currency("USD")
        .page_size(5);

    match client
        .balance_service()
        .get_token_balances_for_wallet_address(Chain::PolygonMainnet, address, Some(polygon_options))
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} tokens on Polygon", data.items.len());
                for item in data.items.iter().take(3) {
                    let symbol = item.contract_ticker_symbol.as_deref().unwrap_or("Unknown");
                    println!("  - {}", symbol);
                }
            }
        }
        Err(e) => {
            println!("Polygon balances error: {:?}", e);
        }
    }

    println!("\nBalance examples completed!");
    Ok(())
}
