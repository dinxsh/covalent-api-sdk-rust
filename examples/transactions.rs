use goldrush_sdk::{GoldRushClient, ClientConfig, Chain, TxOptions};
use std::env;

/// Example demonstrating how to fetch transactions for a wallet address.
///
/// Usage:
///   GOLDRUSH_API_KEY=your_api_key cargo run --example transactions
///   GOLDRUSH_API_KEY=your_api_key cargo run --example transactions -- 0x742d35Cc6634C0532925a3b8D186dC8b7B3e4fe
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

    // Example 1: Basic transaction query
    println!("\nFetching recent transactions for address: {}", address);

    let basic_options = TxOptions::new().page_size(10);

    let transactions = client
        .transaction_service()
        .get_all_transactions_for_address(Chain::EthereumMainnet, address, Some(basic_options))
        .await?;

    if let Some(data) = transactions.data {
        println!("Found {} transactions", data.items.len());

        for (i, tx) in data.items.iter().take(5).enumerate() {
            let success_icon = if tx.successful.unwrap_or(false) { "OK" } else { "FAIL" };
            let value_eth = parse_wei_to_eth(&tx.value);
            let to_addr = tx.to_address.as_deref().unwrap_or("N/A");

            println!(
                "  {} {}: {} -> {} ({:.6} ETH) {}",
                i + 1,
                &tx.tx_hash[..10],
                &tx.from_address[..8],
                &to_addr[..std::cmp::min(8, to_addr.len())],
                value_eth,
                success_icon
            );
        }

        if let Some(pagination) = &transactions.pagination {
            println!("Pagination: page {} of ~{} total items",
                pagination.page_number.unwrap_or(0),
                pagination.total_count.unwrap_or(0)
            );
        }
    }

    // Example 2: Transactions with details and filtering
    println!("\nFetching transactions with USD quotes...");

    let detailed_options = TxOptions::new()
        .page_size(5)
        .quote_currency("USD")
        .no_logs(false);

    let detailed_txs = client
        .transaction_service()
        .get_all_transactions_for_address(Chain::EthereumMainnet, address, Some(detailed_options))
        .await?;

    if let Some(data) = detailed_txs.data {
        for (i, tx) in data.items.iter().enumerate() {
            let value_usd = tx.value_quote.map(|v| format!("${:.2}", v)).unwrap_or_default();
            let gas_usd = tx.gas_quote.map(|v| format!("${:.2}", v)).unwrap_or_default();

            println!(
                "  {}: {} (Value: {}, Gas: {})",
                i + 1,
                &tx.tx_hash[..16],
                value_usd,
                gas_usd
            );

            if let Some(block_time) = &tx.block_signed_at {
                println!("    Time: {}", block_time);
            }

            if let Some(log_events) = &tx.log_events {
                println!("    Log events: {}", log_events.len());
            }
        }
    }

    // Example 3: Get a specific transaction
    println!("\nLooking up a specific transaction...");

    let known_tx = "0x88df016429689c079f3b2f6ad39fa052532c56795b733da78a91ebe6a713944b";

    match client
        .transaction_service()
        .get_transaction(Chain::EthereumMainnet, known_tx, None)
        .await
    {
        Ok(response) => {
            if let Some(tx) = response.data {
                println!("Transaction details:");
                println!("  Hash: {}", tx.tx_hash);
                println!("  From: {}", tx.from_address);
                println!("  To: {}", tx.to_address.unwrap_or_else(|| "Contract Creation".to_string()));
                println!("  Value: {} ETH", parse_wei_to_eth(&tx.value));
                println!("  Success: {}", tx.successful.unwrap_or(false));

                if let Some(block_height) = tx.block_height {
                    println!("  Block: {}", block_height);
                }

                if let Some(gas_used) = tx.gas_used {
                    println!("  Gas used: {}", gas_used);
                }
            }
        }
        Err(e) => {
            println!("Could not fetch specific transaction: {:?}", e);
        }
    }

    // Example 4: Pagination through transactions
    println!("\nDemonstrating pagination...");

    let mut page = 0u32;
    let mut total_found = 0;

    loop {
        let page_options = TxOptions::new()
            .page_size(5)
            .page_number(page);

        match client
            .transaction_service()
            .get_all_transactions_for_address(Chain::EthereumMainnet, address, Some(page_options))
            .await
        {
            Ok(response) => {
                if let Some(data) = response.data {
                    if data.items.is_empty() {
                        break;
                    }

                    total_found += data.items.len();
                    println!("  Page {}: {} transactions", page, data.items.len());

                    // Only fetch a few pages for demo
                    page += 1;
                    if page >= 3 {
                        break;
                    }

                    // Check if there are more pages
                    if let Some(pagination) = response.pagination {
                        if !pagination.has_more.unwrap_or(false) {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
            Err(e) => {
                println!("  Pagination error: {:?}", e);
                break;
            }
        }
    }

    println!("Total transactions found across {} pages: {}", page, total_found);

    // Example 5: Transactions on different chain
    println!("\nChecking Polygon transactions...");

    let polygon_options = TxOptions::new().page_size(3);

    match client
        .transaction_service()
        .get_all_transactions_for_address(Chain::PolygonMainnet, address, Some(polygon_options))
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} transactions on Polygon", data.items.len());
                for tx in data.items.iter().take(2) {
                    println!("  - {}", &tx.tx_hash[..16]);
                }
            }
        }
        Err(e) => {
            println!("Polygon transactions error: {:?}", e);
        }
    }

    println!("\nTransaction examples completed!");
    Ok(())
}

/// Helper function to convert Wei string to ETH as f64
fn parse_wei_to_eth(wei_str: &str) -> f64 {
    wei_str
        .parse::<f64>()
        .map(|wei| wei / 1e18)
        .unwrap_or(0.0)
}
