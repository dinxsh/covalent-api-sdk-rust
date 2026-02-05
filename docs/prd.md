# PRD: Rust SDK for GoldRush Unified API

## 0. Context & References

- GoldRush is the unified blockchain data API product by Covalent, providing balances, transactions, NFT data, and more across 100+ chains. [covalenthq](https://www.covalenthq.com/blog/goldrush-blockchain-data-apis-now-available-on-google-cloud-marketplace/)
- Existing SDKs:
  - TypeScript: `@covalenthq/client-sdk` (GoldRush client). [npmjs](https://www.npmjs.com/package/@covalenthq/client-sdk)
  - Go: `covalent-api-sdk-go` (Covalent API client). [github](https://github.com/covalenthq/covalent-api-sdk-go)

**This project:** Implement a **Rust SDK** that is idiomatic, async, and modeled conceptually on the TS and Go SDKs while targeting GoldRush’s modern API.

Claude should:

- Create a **new Rust crate** (library) with production‑ready structure.  
- Implement core services (balances, transactions, NFTs).  
- Include tests, examples, and documentation.  
- Make design choices explicit in code comments where ambiguity exists.

***

## 1. Goals & Non‑Goals

### 1.1 Goals

1. Provide a **Rust crate** (tentative name: `goldrush-sdk`) that exposes an async `GoldRushClient` for interacting with the GoldRush Unified API. [goldrush](https://goldrush.dev/docs/overview)
2. Abstract **HTTP, JSON, auth, pagination, and errors** into an ergonomic Rust interface.  
3. Mirror the key endpoint coverage and high‑level patterns of:
   - TS `@covalenthq/client-sdk`. [npmjs](https://www.npmjs.com/package/@covalenthq/client-sdk)
   - Go `covalent-api-sdk-go`. [github](https://github.com/covalenthq/covalent-api-sdk-go)
4. Ship with:
   - Strongly typed models.  
   - Unit tests and basic integration tests.  
   - Examples under `examples/`.  
   - A clear README.

### 1.2 Non‑Goals (for v1)

- No WebSocket/SSE streaming.  
- No CLIs.  
- No deep framework integration (Axum/Actix) beyond simple examples.  
- No advanced pricing/analytics helpers; just a thin, typed HTTP client for core data.

***

## 2. Target Users & Use Cases

### 2.1 Users

- Rust backend engineers building APIs or microservices that read GoldRush data.  
- Rust infra/indexer teams powering explorers or dashboards. [covalenthq](https://www.covalenthq.com/blog/introducing-the-goldrush-toolkit-reimagining-a-modular-block-explorer/)
- Protocol/dapp teams using Rust for services but GoldRush for data.  
- DevRel/education creating Rust samples to complement TS examples. [covalenthq](https://www.covalenthq.com/blog/the-covalent-sdk-your-new-dev-language/)

### 2.2 Use Cases

1. **Wallet/portfolio APIs**  
   - For a given chain + address, fetch token/NFT balances and return to frontends.

2. **Transaction history endpoints**  
   - For a given chain + address, fetch paginated transactions and show them in dashboards.

3. **NFT gallery backends**  
   - Get NFTs for an address and metadata for each NFT.

4. **Explorer backends**  
   - Use GoldRush as the data source for custom explorers / GoldRush Kit frontends. [covalenthq](https://www.covalenthq.com/blog/introducing-the-goldrush-toolkit-reimagining-a-modular-block-explorer/)

***

## 3. High‑Level Architecture

### 3.1 Crate

- Name (to be confirmed by maintainers later):
  - `goldrush-sdk` or `goldrush-rs`.  
- Type:
  - `lib` crate, async‑only, with `reqwest` + `serde`.

### 3.2 Core Types

- `GoldRushClient`: public entrypoint to the SDK.  
- `ClientConfig`: configuration options (base URL, timeout, retries).  
- `Error`: enum representing HTTP, serialization, and API errors.  
- Service modules:
  - `balances` (functions on client).  
  - `transactions`.  
  - `nfts`.  

- Model modules:
  - `models::balances`.  
  - `models::transactions`.  
  - `models::nfts`.  

- HTTP internals:
  - `http::request` for constructing requests and headers.  
  - `http::retry` for retry logic.

### 3.3 Async & Runtime Assumptions

- Use `reqwest` with `tokio` feature enabled by default.  
- Do NOT create a runtime internally; expect user to run in a runtime.  
- All public methods that make HTTP calls should be `async fn`.

***

## 4. Project Layout (File/Module Structure)

Claude should create the following structure:

```text
goldrush-sdk/
  Cargo.toml
  README.md
  src/
    lib.rs
    client.rs
    error.rs
    balances.rs
    transactions.rs
    nfts.rs
    models/
      mod.rs
      balances.rs
      transactions.rs
      nfts.rs
    http/
      mod.rs
      request.rs
      retry.rs
  examples/
    balances.rs
    transactions.rs
    nfts.rs
  tests/
    integration_balances.rs
    integration_transactions.rs
    integration_nfts.rs
```

### 4.1 `Cargo.toml` Requirements

- Library crate with:

```toml
[package]
name = "goldrush-sdk"
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"
description = "Rust SDK for GoldRush blockchain data APIs"
repository = "https://github.com/covalenthq/goldrush-sdk-rs" # placeholder

[dependencies]
reqwest = { version = "0.11", features = ["json", "gzip", "brotli", "deflate", "rustls-tls"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"], optional = true }

[features]
default = ["tokio-runtime"]
tokio-runtime = ["tokio"]
```

- MSRV: mention in README (e.g., 1.70+).

***

## 5. Client & Config Design

### 5.1 `ClientConfig`

Create a struct:

```rust
// src/client.rs
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub base_url: String,
    pub timeout: std::time::Duration,
    pub max_retries: u8,
    pub user_agent: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            // official GoldRush API base URL from docs
            base_url: "https://api.goldrush.dev".to_string(), // update if docs say otherwise[web:41]
            timeout: std::time::Duration::from_secs(30),
            max_retries: 3,
            user_agent: format!("goldrush-sdk-rs/{}", env!("CARGO_PKG_VERSION")),
        }
    }
}
```

### 5.2 `GoldRushClient`

```rust
// src/client.rs
use crate::{ClientConfig, Error};
use reqwest::Client as HttpClient;

pub struct GoldRushClient {
    http: HttpClient,
    api_key: String,
    config: ClientConfig,
}

impl GoldRushClient {
    pub fn new<S: Into<String>>(api_key: S, config: ClientConfig) -> Result<Self, Error> {
        let api_key = api_key.into();
        if api_key.trim().is_empty() {
            return Err(Error::MissingApiKey);
        }

        let http = HttpClient::builder()
            .user_agent(&config.user_agent)
            .timeout(config.timeout)
            .build()?;

        Ok(Self { http, api_key, config })
    }
}
```

***

## 6. Error Handling

### 6.1 Error Enum

```rust
// src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing API key")]
    MissingApiKey,

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("API error {status}: {message}")]
    Api {
        status: u16,
        message: String,
        code: Option<u32>,
    },
}
```

- Consider adding variants later if maintainers require.

***

## 7. HTTP Layer

### 7.1 Auth & Request Builder

```rust
// src/http/request.rs
use crate::{Error, GoldRushClient};
use reqwest::{RequestBuilder, Method};

impl GoldRushClient {
    pub(crate) fn build_request(
        &self,
        method: Method,
        path: &str,
    ) -> RequestBuilder {
        let url = format!("{}/{}", self.config.base_url.trim_end_matches('/'), path.trim_start_matches('/'));

        self.http
            .request(method, &url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Accept", "application/json")
    }
}
```

### 7.2 Retry Logic (Simple Exponential Backoff)

```rust
// src/http/retry.rs
use crate::{Error, GoldRushClient};
use reqwest::{Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use std::{thread, time::Duration};

impl GoldRushClient {
    pub(crate) async fn send_with_retry<T>(
        &self,
        builder: RequestBuilder,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let mut attempt: u8 = 0;

        loop {
            let resp = builder.try_clone().expect("builder clone failed").send().await;

            match resp {
                Err(e) => {
                    attempt += 1;
                    if attempt > self.config.max_retries {
                        return Err(Error::Http(e));
                    } else {
                        let backoff = Duration::from_millis(200 * (attempt as u64));
                        tokio::time::sleep(backoff).await;
                        continue;
                    }
                }
                Ok(resp) => {
                    let status = resp.status();
                    let text = resp.text().await?;
                    if !status.is_success() {
                        // Try to parse API error
                        let api_err = serde_json::from_str::<crate::models::ApiErrorEnvelope>(&text).ok();
                        let (code, message) = if let Some(env) = api_err {
                            (env.error.and_then(|e| e.code), env.error.and_then(|e| e.message))
                        } else {
                            (None, None)
                        };
                        return Err(Error::Api {
                            status: status.as_u16(),
                            message: message.unwrap_or_else(|| text.clone()),
                            code,
                        });
                    }

                    let parsed = serde_json::from_str::<T>(&text)?;
                    return Ok(parsed);
                }
            }
        }
    }
}
```

***

## 8. Models (Shared & Services)

### 8.1 Shared Response Envelope

```rust
// src/models/mod.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub has_more: Option<bool>,
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
    pub total_count: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub code: Option<u32>,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorEnvelope {
    pub error: Option<ApiError>,
}
```

We also define generic envelopes where needed per endpoint.

***

## 9. Balances Service

### 9.1 Models

```rust
// src/models/balances.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BalanceItem {
    pub contract_address: String,
    #[serde(rename = "contract_ticker_symbol")]
    pub contract_ticker_symbol: Option<String>,
    pub balance: String,
    pub quote_rate: Option<f64>,
    pub quote: Option<f64>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    // Add more fields based on docs / TS models[web:13][web:41]
}

#[derive(Debug, Deserialize)]
pub struct BalancesData {
    pub items: Vec<BalanceItem>,
}

#[derive(Debug, Deserialize)]
pub struct BalancesResponse {
    pub data: Option<BalancesData>,
    pub error: Option<crate::models::ApiError>,
    pub pagination: Option<crate::models::Pagination>,
}
```

### 9.2 Options

```rust
// src/balances.rs
#[derive(Debug, Default)]
pub struct BalancesOptions {
    pub quote_currency: Option<String>,
    pub nft: Option<bool>,
    // Add additional options if docs support them[web:41]
}
```

### 9.3 Client Method

```rust
// src/balances.rs
use crate::{Error, GoldRushClient};
use crate::models::balances::BalancesResponse;
use reqwest::Method;

impl GoldRushClient {
    pub async fn get_token_balances_for_wallet_address(
        &self,
        chain_id: &str,
        address: &str,
        options: Option<BalancesOptions>,
    ) -> Result<BalancesResponse, Error> {
        let path = format!("/v1/{}/address/{}/balances_v2/", chain_id, address); // confirm path vs docs[web:41]

        let builder = self.build_request(Method::GET, &path);
        let builder = if let Some(opts) = options {
            let mut b = builder;
            if let Some(q) = opts.quote_currency {
                b = b.query(&[("quote-currency", q)]);
            }
            if let Some(nft) = opts.nft {
                b = b.query(&[("nft", nft.to_string())]);
            }
            b
        } else {
            builder
        };

        self.send_with_retry::<BalancesResponse>(builder).await
    }
}
```

***

## 10. Transactions Service

### 10.1 Models

```rust
// src/models/transactions.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransactionItem {
    pub tx_hash: String,
    pub from_address: String,
    pub to_address: Option<String>,
    pub value: String,
    pub successful: Option<bool>,
    // Add more fields per docs / TS models[web:13][web:41]
}

#[derive(Debug, Deserialize)]
pub struct TransactionsData {
    pub items: Vec<TransactionItem>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionsResponse {
    pub data: Option<TransactionsData>,
    pub error: Option<crate::models::ApiError>,
    pub pagination: Option<crate::models::Pagination>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionResponse {
    pub data: Option<TransactionItem>,
    pub error: Option<crate::models::ApiError>,
}
```

### 10.2 Options

```rust
// src/transactions.rs
#[derive(Debug, Default)]
pub struct TxOptions {
    pub page_size: Option<u32>,
    pub page_number: Option<u32>,
    // additional filters based on docs[web:41]
}
```

### 10.3 Client Methods

```rust
// src/transactions.rs
use crate::{Error, GoldRushClient};
use crate::models::transactions::{TransactionsResponse, TransactionResponse};
use reqwest::Method;

impl GoldRushClient {
    pub async fn get_all_transactions_for_address(
        &self,
        chain_id: &str,
        address: &str,
        options: Option<TxOptions>,
    ) -> Result<TransactionsResponse, Error> {
        let path = format!("/v1/{}/address/{}/transactions_v2/", chain_id, address); // verify exact path[web:28][web:41]

        let mut builder = self.build_request(Method::GET, &path);

        if let Some(opts) = options {
            if let Some(ps) = opts.page_size {
                builder = builder.query(&[("page-size", ps.to_string())]);
            }
            if let Some(pn) = opts.page_number {
                builder = builder.query(&[("page-number", pn.to_string())]);
            }
        }

        self.send_with_retry::<TransactionsResponse>(builder).await
    }

    pub async fn get_transaction(
        &self,
        chain_id: &str,
        tx_hash: &str,
    ) -> Result<TransactionResponse, Error> {
        let path = format!("/v1/{}/transaction_v2/{}", chain_id, tx_hash); // verify from docs[web:28][web:41]
        let builder = self.build_request(Method::GET, &path);

        self.send_with_retry::<TransactionResponse>(builder).await
    }
}
```

***

## 11. NFTs Service

### 11.1 Models

```rust
// src/models/nfts.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NftItem {
    pub contract_address: String,
    pub token_id: String,
    pub token_url: Option<String>,
    pub token_balance: Option<String>,
    // add fields aligned with docs[web:41]
}

#[derive(Debug, Deserialize)]
pub struct NftsData {
    pub items: Vec<NftItem>,
}

#[derive(Debug, Deserialize)]
pub struct NftsResponse {
    pub data: Option<NftsData>,
    pub error: Option<crate::models::ApiError>,
    pub pagination: Option<crate::models::Pagination>,
}

#[derive(Debug, Deserialize)]
pub struct NftMetadataItem {
    pub contract_address: String,
    pub token_id: String,
    pub token_uri: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct NftMetadataResponse {
    pub data: Option<Vec<NftMetadataItem>>,
    pub error: Option<crate::models::ApiError>,
}
```

### 11.2 Options & Methods

```rust
// src/nfts.rs
use crate::{Error, GoldRushClient};
use crate::models::nfts::{NftsResponse, NftMetadataResponse};
use reqwest::Method;

#[derive(Debug, Default)]
pub struct NftOptions {
    pub page_size: Option<u32>,
    pub page_number: Option<u32>,
}

impl GoldRushClient {
    pub async fn get_nfts_for_address(
        &self,
        chain_id: &str,
        address: &str,
        options: Option<NftOptions>,
    ) -> Result<NftsResponse, Error> {
        let path = format!("/v1/{}/address/{}/balances_nft/", chain_id, address); // confirm exact path[web:41]
        let mut builder = self.build_request(Method::GET, &path);

        if let Some(opts) = options {
            if let Some(ps) = opts.page_size {
                builder = builder.query(&[("page-size", ps.to_string())]);
            }
            if let Some(pn) = opts.page_number {
                builder = builder.query(&[("page-number", pn.to_string())]);
            }
        }

        self.send_with_retry::<NftsResponse>(builder).await
    }

    pub async fn get_nft_metadata(
        &self,
        chain_id: &str,
        contract_address: &str,
        token_id: &str,
    ) -> Result<NftMetadataResponse, Error> {
        let path = format!(
            "/v1/{}/tokens/{}/nft_metadata/{}",
            chain_id, contract_address, token_id
        ); // confirm from docs[web:41]

        let builder = self.build_request(Method::GET, &path);
        self.send_with_retry::<NftMetadataResponse>(builder).await
    }
}
```

***

## 12. Pagination Helpers (Optional v1, Nice to Have)

Claude can implement a simple iterator type:

```rust
// in src/transactions.rs or a shared pagination module
pub struct TransactionsPageIter<'a> {
    client: &'a GoldRushClient,
    chain_id: String,
    address: String,
    options: TxOptions,
    finished: bool,
}

impl<'a> TransactionsPageIter<'a> {
    pub fn new<C: Into<String>, A: Into<String>>(
        client: &'a GoldRushClient,
        chain_id: C,
        address: A,
        options: TxOptions,
    ) -> Self {
        Self {
            client,
            chain_id: chain_id.into(),
            address: address.into(),
            options,
            finished: false,
        }
    }

    pub async fn next(&mut self) -> Result<Option<Vec<crate::models::transactions::TransactionItem>>, Error> {
        if self.finished {
            return Ok(None);
        }

        let resp = self
            .client
            .get_all_transactions_for_address(&self.chain_id, &self.address, Some(self.options.clone()))
            .await?;

        if let Some(data) = resp.data {
            let items = data.items;
            if items.is_empty() || resp.pagination.as_ref().and_then(|p| p.has_more).unwrap_or(false) == false {
                self.finished = true;
            } else if let Some(p) = resp.pagination {
                if let Some(next_page) = p.page_number.map(|n| n + 1) {
                    self.options.page_number = Some(next_page);
                } else {
                    self.finished = true;
                }
            }
            Ok(Some(items))
        } else {
            self.finished = true;
            Ok(None)
        }
    }
}
```

***

## 13. Tests

### 13.1 Unit Tests (Models & URLs)

Claude should:

- Create unit tests under `src/*_tests.rs` or `tests/` verifying:
  - URL construction for methods based on known paths in GoldRush docs. [goldrush](https://goldrush.dev/docs/overview)
  - Query parameters for options.  
  - JSON deserialization using static fixtures (small JSON strings defined in test code).

Example (balances):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize_balances_response() {
        let raw = json!({
          "data": {
            "items": [{
              "contract_address": "0x123",
              "contract_ticker_symbol": "ABC",
              "balance": "1000000000000000000",
              "quote_rate": 1.23,
              "quote": 1.23,
              "type": "cryptocurrency"
            }]
          },
          "error": null,
          "pagination": null
        }).to_string();

        let parsed: BalancesResponse = serde_json::from_str(&raw).unwrap();
        assert!(parsed.data.is_some());
        assert_eq!(parsed.data.unwrap().items[0].contract_address, "0x123");
    }
}
```

### 13.2 Integration Tests

In `tests/`:

- `integration_balances.rs`:  
  - Uses `GOLDRUSH_API_KEY` env var; if not set, skip tests.  
  - Calls `get_token_balances_for_wallet_address` on a known chain/address and asserts non‑empty response.

- `integration_transactions.rs`, `integration_nfts.rs` similarly.

***

## 14. Examples

Create `examples/` binaries:

### 14.1 `examples/balances.rs`

```rust
use goldrush_sdk::{GoldRushClient, ClientConfig};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GOLDRUSH_API_KEY")
        .expect("GOLDRUSH_API_KEY env var not set");

    let client = GoldRushClient::new(api_key, ClientConfig::default())?;

    let resp = client
        .get_token_balances_for_wallet_address(
            "eth-mainnet",
            "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
            None,
        )
        .await?;

    if let Some(data) = resp.data {
        for item in data.items {
            println!(
                "{}: {}",
                item.contract_ticker_symbol.unwrap_or_default(),
                item.balance
            );
        }
    }

    Ok(())
}
```

### 14.2 `examples/transactions.rs` & `examples/nfts.rs`

- Analogous examples:
  - Print transaction hashes.  
  - Print NFT contract + token IDs.

***

## 15. README Content (High‑Level)

Claude should generate a `README.md` with:

- Project description.  
- Install instructions.  
- Quickstart code snippet (like `examples/balances.rs`).  
- Basic API docs (methods summary).  
- Links to:
  - GoldRush docs (overview & API). [goldrush](https://goldrush.dev/docs/overview)
  - TS and Go SDKs for conceptual parity. [npmjs](https://www.npmjs.com/package/@covalenthq/client-sdk)

***

## 16. Open Items for Maintainers (Comment in Code or README)

Claude should leave TODO comments where things depend on maintainers:

- Confirm base URL (`https://api.goldrush.dev` vs exact path per latest docs). [goldrush](https://goldrush.dev/docs/overview)
- Confirm exact endpoint paths (`/balances_v2`, `/transactions_v2`, NFT endpoints). [github](https://github.com/covalenthq/covalent-api-sdk-go)
- Confirm crate name and repo URL.  
- Confirm minimum endpoint set for v1 (if more than balances/transactions/NFTs).

***

This PRD is meant for Claude to implement **E2E**:

- Set up the crate.  
- Implement modules and types as specified.  
- Add tests and examples.  
- Keep any unknowns marked by clear `TODO` comments so you can align with the actual GoldRush docs/maintainers later.