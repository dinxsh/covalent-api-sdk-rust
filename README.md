# GoldRush SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/goldrush-sdk)](https://crates.io/crates/goldrush-sdk)
[![Documentation](https://docs.rs/goldrush-sdk/badge.svg)](https://docs.rs/goldrush-sdk)
[![License](https://img.shields.io/crates/l/goldrush-sdk)](#license)

A Rust client library for the [GoldRush blockchain data APIs](https://goldrush.dev/docs/overview) by Covalent. Fetch token balances, transaction history, NFT data, and more across 100+ blockchain networks.

## Features

- 🔄 **Async/await support** with Tokio
- 🛡️ **Type-safe** API responses with serde
- 🔀 **Automatic retries** with exponential backoff
- 📖 **Comprehensive error handling**
- 🔗 **Multi-chain support** (Ethereum, Polygon, BSC, Avalanche, and more)
- 📄 **Built-in pagination helpers**
- 🧪 **Extensive test coverage**
- 📡 **Real-time streaming** via WebSocket GraphQL subscriptions (optional)

## Supported Endpoints

### REST APIs (36 endpoints)
- **Token Balances** - Get ERC-20 token balances for any address
- **Transactions** - Fetch transaction history with detailed information
- **NFTs** - Query NFT holdings and metadata
- **Historical Data** - Access portfolio valuations over time
- **Pricing** - Token prices and historical data
- **Security** - Token approvals and security analysis

### Real-Time Streaming APIs (8 endpoints) - Optional Feature
- **OHLCV Subscriptions** - Real-time candlestick data for trading pairs and tokens
- **New DEX Pairs** - Live notifications for newly created liquidity pairs
- **Pair Updates** - Real-time price, volume, and liquidity updates
- **Wallet Activity** - Monitor transactions and transfers in real-time
- **Token Search** - Search for tokens across chains
- **P&L Queries** - Unrealized profit/loss for tokens and wallets

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
goldrush-sdk = "0.2.0"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }

# Optional: Enable streaming support
# goldrush-sdk = { version = "0.2.0", features = ["streaming"] }
```

### Basic Usage

```rust
use goldrush_sdk::{GoldRushClient, ClientConfig, BalancesOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with your API key
    let client = GoldRushClient::new("your-api-key", ClientConfig::default())?;

    // Get token balances for an address
    let balances = client
        .get_token_balances_for_wallet_address(
            "eth-mainnet",
            "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
            None
        )
        .await?;

    // Process the results
    if let Some(data) = balances.data {
        for token in &data.items {
            println!(
                "{}: {} (${:.2})",
                token.contract_ticker_symbol.as_deref().unwrap_or("Unknown"),
                token.balance,
                token.quote.unwrap_or(0.0)
            );
        }
    }

    Ok(())
}
```

## Examples

### Token Balances

```rust
use goldrush_sdk::{GoldRushClient, BalancesOptions};

let options = BalancesOptions::new()
    .quote_currency("USD")
    .no_spam(true)
    .page_size(50);

let balances = client
    .get_token_balances_for_wallet_address(
        "eth-mainnet",
        "0x742d35Cc6634C0532925a3b8D186dC8b7B3e4fe",
        Some(options)
    )
    .await?;
```

### Transaction History

```rust
use goldrush_sdk::{TxOptions};

let options = TxOptions::new()
    .page_size(10)
    .quote_currency("USD")
    .with_log_events(true);

let transactions = client
    .get_all_transactions_for_address(
        "eth-mainnet",
        "0x742d35Cc6634C0532925a3b8D186dC8b7B3e4fe",
        Some(options)
    )
    .await?;
```

### NFT Holdings

```rust
use goldrush_sdk::{NftOptions};

let options = NftOptions::new()
    .with_metadata(true)
    .no_spam(true)
    .page_size(20);

let nfts = client
    .get_nfts_for_address(
        "eth-mainnet",
        "0x742d35Cc6634C0532925a3b8D186dC8b7B3e4fe",
        Some(options)
    )
    .await?;
```

### Specific Transaction

```rust
let transaction = client
    .get_transaction(
        "eth-mainnet",
        "0x123abc..."
    )
    .await?;
```

### NFT Metadata

```rust
let metadata = client
    .get_nft_metadata(
        "eth-mainnet",
        "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d", // BAYC contract
        "1" // Token ID
    )
    .await?;
```

## Supported Chains

The SDK supports all chains available in the GoldRush API. Some popular ones include:

- **Ethereum** - `eth-mainnet`, `eth-goerli`, `eth-sepolia`
- **Polygon** - `matic-mainnet`, `matic-mumbai`
- **BSC** - `bsc-mainnet`, `bsc-testnet`
- **Avalanche** - `avalanche-mainnet`, `avalanche-fuji`
- **Arbitrum** - `arbitrum-mainnet`, `arbitrum-goerli`
- **Optimism** - `optimism-mainnet`, `optimism-goerli`
- **Base** - `base-mainnet`, `base-goerli`

For the full list, see the [GoldRush documentation](https://goldrush.dev/docs/overview).

## Configuration

### Client Configuration

```rust
use goldrush_sdk::{GoldRushClient, ClientConfig};
use std::time::Duration;

let config = ClientConfig::default()
    .with_timeout(Duration::from_secs(60))
    .with_max_retries(5)
    .with_user_agent("my-app/1.0");

let client = GoldRushClient::new("your-api-key", config)?;
```

### Custom Base URL

```rust
let config = ClientConfig::new("https://api.custom-goldrush-instance.com")
    .with_timeout(Duration::from_secs(30));

let client = GoldRushClient::new("your-api-key", config)?;
```

## Error Handling

The SDK provides comprehensive error types:

```rust
use goldrush_sdk::Error;

match client.get_token_balances_for_wallet_address("eth-mainnet", "0x123", None).await {
    Ok(response) => {
        // Handle success
    }
    Err(Error::Api { status, message, .. }) => {
        eprintln!("API Error {}: {}", status, message);
    }
    Err(Error::Http(e)) => {
        eprintln!("Network error: {}", e);
    }
    Err(Error::Serialization(e)) => {
        eprintln!("JSON parsing error: {}", e);
    }
    Err(Error::MissingApiKey) => {
        eprintln!("API key is required");
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Pagination

### Manual Pagination

```rust
let mut page = 0;
loop {
    let options = TxOptions::new()
        .page_size(100)
        .page_number(page);
    
    let response = client
        .get_all_transactions_for_address("eth-mainnet", address, Some(options))
        .await?;
    
    if let Some(data) = response.data {
        process_transactions(&data.items);
        
        // Check if there are more pages
        if let Some(pagination) = response.pagination {
            if !pagination.has_more.unwrap_or(false) {
                break;
            }
        }
        page += 1;
    } else {
        break;
    }
}
```

### Using Iterator Helper

```rust
use goldrush_sdk::{TransactionsPageIter, TxOptions};

let options = TxOptions::new().page_size(100);
let mut iter = TransactionsPageIter::new(&client, "eth-mainnet", address, options);

while let Some(transactions) = iter.next().await? {
    for tx in transactions {
        println!("Transaction: {}", tx.tx_hash);
    }
}
```

## Real-Time Streaming (Optional)

Enable the `streaming` feature to access real-time WebSocket subscriptions:

```toml
[dependencies]
goldrush-sdk = { version = "0.2.0", features = ["streaming"] }
futures-util = "0.3"
```

### Wallet Activity Stream

Monitor wallet transactions in real-time:

```rust
use goldrush_sdk::models::streaming::*;
use futures_util::StreamExt;

let service = client.streaming_service();

let params = WalletActivityParams {
    chain_name: StreamingChain::BaseMainnet,
    wallet_addresses: vec!["0x...".to_string()],
};

let (mut stream, handle) = service.subscribe_to_wallet_activity(params).await?;

while let Some(result) = stream.next().await {
    match result {
        Ok(transactions) => {
            for tx in transactions {
                println!("New TX: {} - {} -> {}",
                    tx.tx_hash, tx.from_address, tx.to_address);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

handle.unsubscribe().await?;
```

### OHLCV Price Data

Stream real-time candlestick data:

```rust
use goldrush_sdk::models::streaming::*;

let params = OhlcvPairsParams {
    chain_name: StreamingChain::BaseMainnet,
    pair_addresses: vec!["0x9c087Eb773291e50CF6c6a90ef0F4500e349B903".to_string()],
    interval: StreamingInterval::OneMinute,
    timeframe: StreamingTimeframe::OneHour,
    limit: Some(10),
};

let (mut stream, handle) = service.subscribe_to_ohlcv_pairs(params).await?;

while let Some(result) = stream.next().await {
    match result {
        Ok(candles) => {
            for candle in candles {
                println!("OHLC: O:{:.4} H:{:.4} L:{:.4} C:{:.4}",
                    candle.open, candle.high, candle.low, candle.close);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Token Search

Search for tokens across chains:

```rust
let params = TokenSearchParams {
    query: "USDC".to_string(),
};

let results = service.search_token(params).await?;

for token in results {
    println!("{} - ${:.2} market cap",
        token.base_token.contract_name,
        token.market_cap);
}
```

### Streaming Configuration

Customize WebSocket behavior:

```rust
use goldrush_sdk::streaming::StreamingConfig;

let config = StreamingConfig::builder()
    .max_reconnect_attempts(10)
    .on_connected(|| println!("Connected!"))
    .on_error(|e| eprintln!("Error: {}", e))
    .auto_resubscribe(true)
    .build();

let service = client.streaming_service_with_config(config);
```

## Running Examples

Clone this repository and run the examples:

```bash
# Set your API key
export GOLDRUSH_API_KEY="your-api-key-here"

# Run balance example
cargo run --example balances

# Run with specific address
cargo run --example balances -- 0x742d35Cc6634C0532925a3b8D186dC8b7B3e4fe

# Run transaction example
cargo run --example transactions

# Run NFT example
cargo run --example nfts
```

## Testing

Run the unit tests:

```bash
cargo test
```

Run integration tests (requires API key):

```bash
export GOLDRUSH_API_KEY="your-api-key"
cargo test --test integration
```

## API Key

Get your free API key from the [Covalent Dashboard](https://www.covalenthq.com/platform/#/auth/register/).

- Free tier: 100,000 API credits per month
- Generous rate limits for development and small applications
- Production plans available for higher volumes

## Related Projects

- **TypeScript SDK**: [@covalenthq/client-sdk](https://www.npmjs.com/package/@covalenthq/client-sdk)
- **Go SDK**: [covalent-api-sdk-go](https://github.com/covalenthq/covalent-api-sdk-go)
- **GoldRush Kit**: React components for blockchain UIs
- **Block Explorer Template**: [Built with GoldRush](https://www.covalenthq.com/blog/introducing-the-goldrush-toolkit-reimagining-a-modular-block-explorer/)

## Requirements

- Rust 1.70.0 or later
- Tokio runtime for async support

## Roadmap

- [ ] WebSocket support for real-time data
- [ ] Additional analytics endpoints
- [ ] More comprehensive pagination helpers
- [ ] CLI tools
- [ ] Framework integrations (Axum, Actix)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Links

- **GoldRush API Documentation**: https://goldrush.dev/docs/overview
- **Covalent Website**: https://www.covalenthq.com/
- **TypeScript SDK**: https://www.npmjs.com/package/@covalenthq/client-sdk
- **Go SDK**: https://github.com/covalenthq/covalent-api-sdk-go