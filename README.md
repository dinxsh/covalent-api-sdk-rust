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

## Supported Endpoints

- **Token Balances** - Get ERC-20 token balances for any address
- **Transactions** - Fetch transaction history with detailed information
- **NFTs** - Query NFT holdings and metadata
- **Historical Data** - Access portfolio valuations over time

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
goldrush-sdk = "0.1.0"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
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
use goldrush_sdk::transactions::TransactionsPageIter;

let options = TxOptions::new().page_size(100);
let mut iter = TransactionsPageIter::new(&client, "eth-mainnet", address, options);

while let Some(transactions) = iter.next().await? {
    for tx in transactions {
        println!("Transaction: {}", tx.tx_hash);
    }
}
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

---

## TODOs for Maintainers

The following items need confirmation from the Covalent team:

- **Base URL**: Confirm exact production URL for GoldRush API (currently using `https://api.goldrush.dev`)
- **Endpoint Paths**: Verify exact paths for all endpoints (e.g., `/v1/{chain}/address/{address}/balances_v2/`)
- **Response Formats**: Align models with actual API response structure
- **Chain Names**: Confirm supported chain identifiers format
- **Error Codes**: Map specific API error codes to appropriate error variants
- **Crate Name**: Finalize crate name and repository URL
- **Authentication**: Confirm Bearer token format vs other auth methods

## Links

- **GoldRush API Documentation**: https://goldrush.dev/docs/overview
- **Covalent Website**: https://www.covalenthq.com/
- **TypeScript SDK**: https://www.npmjs.com/package/@covalenthq/client-sdk
- **Go SDK**: https://github.com/covalenthq/covalent-api-sdk-go