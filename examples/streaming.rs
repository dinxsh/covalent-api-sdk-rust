//! Streaming API Examples
//!
//! This example demonstrates all streaming endpoints:
//! 1. OHLCV pairs subscription
//! 2. OHLCV tokens subscription
//! 3. New DEX pairs subscription
//! 4. Update DEX pairs subscription
//! 5. Wallet activity subscription
//! 6. Token search query
//! 7. Unrealized P&L for token query
//! 8. Unrealized P&L for wallet query
//!
//! Run with:
//! ```bash
//! GOLDRUSH_API_KEY=your_key cargo run --example streaming --features streaming
//! ```

use goldrush_sdk::models::streaming::*;
use goldrush_sdk::streaming::StreamingConfig;
use goldrush_sdk::{ClientConfig, GoldRushClient};
use futures_util::{pin_mut, StreamExt};
use std::env;
use std::iter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get API key from environment
    let api_key = env::var("GOLDRUSH_API_KEY")
        .expect("GOLDRUSH_API_KEY environment variable not set");

    // Create client
    let client = GoldRushClient::new(&api_key, ClientConfig::default())?;

    // Configure streaming with custom callbacks
    let streaming_config = StreamingConfig::builder()
        .on_connected(|| println!("✅ WebSocket connected!"))
        .on_connecting(|| println!("🔄 Connecting to WebSocket..."))
        .on_closed(|| println!("❌ WebSocket closed"))
        .on_error(|e| eprintln!("⚠️  WebSocket error: {}", e))
        .max_reconnect_attempts(5)
        .build();

    let service = client.streaming_service_with_config(streaming_config);

    println!("\n=== GoldRush Streaming API Examples ===\n");

    // Example menu
    println!("Choose an example to run:");
    println!("1. OHLCV Pairs Subscription");
    println!("2. OHLCV Tokens Subscription");
    println!("3. New DEX Pairs Subscription");
    println!("4. Update DEX Pairs Subscription");
    println!("5. Wallet Activity Subscription");
    println!("6. Token Search Query");
    println!("7. Unrealized P&L for Token");
    println!("8. Unrealized P&L for Wallet");
    println!("0. Run all examples (limited duration)");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let choice: u8 = input.trim().parse().unwrap_or(0);

    match choice {
        1 => ohlcv_pairs_example(&service).await?,
        2 => ohlcv_tokens_example(&service).await?,
        3 => new_pairs_example(&service).await?,
        4 => update_pairs_example(&service).await?,
        5 => wallet_activity_example(&service).await?,
        6 => token_search_example(&service).await?,
        7 => upnl_token_example(&service).await?,
        8 => upnl_wallet_example(&service).await?,
        _ => run_all_examples(&service).await?,
    }

    Ok(())
}

/// Example 1: Subscribe to OHLCV pairs data
async fn ohlcv_pairs_example(
    service: &goldrush_sdk::services::StreamingService,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Example 1: OHLCV Pairs Subscription\n");

    let params = OhlcvPairsParams {
        chain_name: StreamingChain::BaseMainnet,
        pair_addresses: vec!["0x9c087Eb773291e50CF6c6a90ef0F4500e349B903".to_string()],
        interval: StreamingInterval::OneMinute,
        timeframe: StreamingTimeframe::OneHour,
        limit: Some(10),
    };

    let (stream, handle) = service.subscribe_to_ohlcv_pairs(params).await?;

    pin_mut!(stream);

    println!("Subscribed! Waiting for OHLCV data...");
    println!("(Press Ctrl+C to stop)\n");

    let mut count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(candles) => {
                println!("📈 Received {} candles:", candles.len());
                for candle in candles.iter().take(3) {
                    println!(
                        "  {} | O: {:.6} H: {:.6} L: {:.6} C: {:.6} | Vol: ${:.2}",
                        candle.timestamp,
                        candle.open,
                        candle.high,
                        candle.low,
                        candle.close,
                        candle.volume_usd
                    );
                }
                count += 1;
                if count >= 5 {
                    break;
                }
            }
            Err(e) => eprintln!("❌ Error: {}", e),
        }
    }

    handle.unsubscribe().await?;
    println!("\n✅ Unsubscribed from OHLCV pairs");

    Ok(())
}

/// Example 2: Subscribe to OHLCV tokens data
async fn ohlcv_tokens_example(
    service: &goldrush_sdk::services::StreamingService,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Example 2: OHLCV Tokens Subscription\n");

    let params = OhlcvTokensParams {
        chain_name: StreamingChain::BaseMainnet,
        token_addresses: vec!["0x4200000000000000000000000000000000000006".to_string()],
        interval: StreamingInterval::FiveMinutes,
        timeframe: StreamingTimeframe::OneHour,
        limit: Some(5),
    };

    let (stream, handle) = service.subscribe_to_ohlcv_tokens(params).await?;

    pin_mut!(stream);

    println!("Subscribed! Waiting for token OHLCV data...\n");

    let mut count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(candles) => {
                for candle in candles {
                    println!(
                        "📊 {} - ${:.4} (Vol: ${:.2})",
                        candle.base_token.contract_ticker_symbol.unwrap_or_default(),
                        candle.close,
                        candle.volume_usd
                    );
                }
                count += 1;
                if count >= 3 {
                    break;
                }
            }
            Err(e) => eprintln!("❌ Error: {}", e),
        }
    }

    handle.unsubscribe().await?;
    println!("\n✅ Unsubscribed from OHLCV tokens");

    Ok(())
}

/// Example 3: Subscribe to new DEX pairs
async fn new_pairs_example(
    service: &goldrush_sdk::services::StreamingService,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🆕 Example 3: New DEX Pairs Subscription\n");

    let params = NewPairsParams {
        chain_name: StreamingChain::BaseMainnet,
        protocols: vec![StreamingProtocol::UniswapV2, StreamingProtocol::UniswapV3],
    };

    let (stream, handle) = service.subscribe_to_new_pairs(params).await?;

    pin_mut!(stream);

    println!("Subscribed! Waiting for new pairs...");
    println!("(This may take a while if no new pairs are being created)\n");

    let mut count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(pairs) => {
                for pair in pairs {
                    println!("🆕 New Pair Detected!");
                    println!("  Protocol: {}", pair.protocol);
                    println!("  Pair: {}", pair.pair_address);
                    println!(
                        "  Token: {}",
                        pair.base_token.contract_ticker_symbol.unwrap_or_default()
                    );
                    println!("  Liquidity: ${:.2}", pair.liquidity);
                    println!("  Market Cap: ${:.2}", pair.market_cap);
                    println!("  Deployer: {}", pair.deployer_address);
                    println!();
                }
                count += 1;
                if count >= 5 {
                    break;
                }
            }
            Err(e) => eprintln!("❌ Error: {}", e),
        }
    }

    handle.unsubscribe().await?;
    println!("✅ Unsubscribed from new pairs");

    Ok(())
}

/// Example 4: Subscribe to pair updates
async fn update_pairs_example(
    service: &goldrush_sdk::services::StreamingService,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 Example 4: Update DEX Pairs Subscription\n");

    let params = UpdatePairsParams {
        chain_name: StreamingChain::BaseMainnet,
        pair_addresses: vec!["0x9c087Eb773291e50CF6c6a90ef0F4500e349B903".to_string()],
    };

    let (stream, handle) = service.subscribe_to_update_pairs(params).await?;

    pin_mut!(stream);

    println!("Subscribed! Waiting for pair updates...\n");

    let mut count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(update) => {
                println!("🔄 Pair Update:");
                println!("  Price: ${:.6}", update.quote_rate_usd);
                println!("  Volume: ${:.2}", update.volume_usd);
                println!("  Market Cap: ${:.2}", update.market_cap);
                println!("  Liquidity: ${:.2}", update.liquidity);
                println!("  Price Δ 5m: {:.2}%", update.price_deltas.last_5m * 100.0);
                println!("  Price Δ 1h: {:.2}%", update.price_deltas.last_1hr * 100.0);
                println!("  Swaps (5m): {}", update.swap_counts.last_5m);
                println!();
                count += 1;
                if count >= 5 {
                    break;
                }
            }
            Err(e) => eprintln!("❌ Error: {}", e),
        }
    }

    handle.unsubscribe().await?;
    println!("✅ Unsubscribed from pair updates");

    Ok(())
}

/// Example 5: Subscribe to wallet activity
async fn wallet_activity_example(
    service: &goldrush_sdk::services::StreamingService,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n👛 Example 5: Wallet Activity Subscription\n");

    let params = WalletActivityParams {
        chain_name: StreamingChain::BaseMainnet,
        wallet_addresses: vec!["0x4200000000000000000000000000000000000006".to_string()],
    };

    let (stream, handle) = service.subscribe_to_wallet_activity(params).await?;

    pin_mut!(stream);

    println!("Subscribed! Waiting for wallet transactions...");
    println!("(Press Ctrl+C to stop)\n");

    let mut count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(transactions) => {
                for tx in transactions {
                    println!("💸 Transaction:");
                    println!("  Hash: {}", tx.tx_hash);
                    println!("  From: {}", tx.from_address);
                    println!("  To: {}", tx.to_address);
                    println!("  Type: {}", tx.decoded_type);
                    println!("  Success: {}", tx.successful);
                    println!("  Gas Used: {}", tx.gas_used);
                    println!();
                }
                count += 1;
                if count >= 5 {
                    break;
                }
            }
            Err(e) => eprintln!("❌ Error: {}", e),
        }
    }

    handle.unsubscribe().await?;
    println!("✅ Unsubscribed from wallet activity");

    Ok(())
}

/// Example 6: Search for tokens
async fn token_search_example(
    service: &goldrush_sdk::services::StreamingService,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Example 6: Token Search\n");

    let params = TokenSearchParams {
        query: "USDC".to_string(),
    };

    println!("Searching for tokens matching 'USDC'...\n");

    let results = service.search_token(params).await?;

    println!("Found {} results:\n", results.len());

    for (i, token) in results.iter().take(10).enumerate() {
        println!("{}. {}", i + 1, token.base_token.contract_name);
        println!(
            "   Symbol: {}",
            token.base_token.contract_ticker_symbol.as_deref().unwrap_or("N/A")
        );
        println!("   Price: ${:.4}", token.quote_rate_usd);
        println!("   Market Cap: ${:.2}", token.market_cap);
        println!("   Volume: ${:.2}", token.volume_usd);
        println!("   Chain: {}", token.chain_name);
        println!();
    }

    Ok(())
}

/// Example 7: Get unrealized P&L for token
async fn upnl_token_example(
    service: &goldrush_sdk::services::StreamingService,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💰 Example 7: Unrealized P&L for Token\n");

    let params = UpnlForTokenParams {
        chain_name: StreamingChain::BaseMainnet,
        token_address: "0x4200000000000000000000000000000000000006".to_string(),
    };

    println!("Fetching top traders P&L...\n");

    let results = service.get_upnl_for_token(params).await?;

    println!("Top {} traders:\n", results.len());

    for (i, trader) in results.iter().take(10).enumerate() {
        println!("{}. Wallet: {}", i + 1, trader.wallet_address);
        println!("   Balance: {} ({})", trader.balance_pretty, trader.balance);
        println!("   Volume: {}", trader.volume);
        println!("   Transactions: {}", trader.transactions_count);
        println!("   Realized P&L: ${:.2}", trader.pnl_realized_usd);
        println!("   Unrealized P&L: ${:.2}", trader.pnl_unrealized_usd);
        println!();
    }

    Ok(())
}

/// Example 8: Get unrealized P&L for wallet
async fn upnl_wallet_example(
    service: &goldrush_sdk::services::StreamingService,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💼 Example 8: Unrealized P&L for Wallet\n");

    let params = UpnlForWalletParams {
        chain_name: StreamingChain::BaseMainnet,
        wallet_address: "0x4200000000000000000000000000000000000006".to_string(),
    };

    println!("Fetching wallet P&L...\n");

    let results = service.get_upnl_for_wallet(params).await?;

    println!("Holdings ({} tokens):\n", results.len());

    for (i, holding) in results.iter().take(10).enumerate() {
        println!(
            "{}. {}",
            i + 1,
            holding.contract_metadata.contract_ticker_symbol.as_deref().unwrap_or("N/A")
        );
        println!("   Token: {}", holding.token_address);
        println!("   Cost Basis: ${:.4}", holding.cost_basis);
        println!("   Current Price: ${:.4}", holding.current_price);
        if let Some(realized) = holding.pnl_realized_usd {
            println!("   Realized P&L: ${:.2}", realized);
        }
        println!("   Unrealized P&L: ${:.2}", holding.pnl_unrealized_usd);
        println!("   Market Cap: ${}", holding.marketcap_usd);
        println!();
    }

    Ok(())
}

/// Run all examples briefly
async fn run_all_examples(
    service: &goldrush_sdk::services::StreamingService,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🚀 Running all examples (brief demonstration)...\n");

    // Run query examples (these are quick)
    println!("{}", iter::repeat("─").take(50).collect::<String>());
    token_search_example(service).await?;

    println!("{}", iter::repeat("─").take(50).collect::<String>());
    upnl_token_example(service).await?;

    println!("{}", iter::repeat("─").take(50).collect::<String>());
    upnl_wallet_example(service).await?;

    println!("\n✅ All examples completed!");
    println!("\nNote: Subscription examples were skipped in 'all' mode.");
    println!("Run individual examples (1-5) to see streaming in action.");

    Ok(())
}
