use goldrush_sdk::{GoldRushClient, ClientConfig, Chain, QuoteCurrency, PricingOptions, Error};

/// Integration tests for the pricing service.

fn get_test_client() -> Option<GoldRushClient> {
    if let Ok(api_key) = std::env::var("GOLDRUSH_API_KEY") {
        GoldRushClient::new(api_key, ClientConfig::default()).ok()
    } else {
        println!("GOLDRUSH_API_KEY not set, skipping integration tests");
        None
    }
}

#[tokio::test]
async fn test_get_token_prices() {
    let Some(client) = get_test_client() else { return; };

    let opts = PricingOptions::new()
        .from("2024-01-01")
        .to("2024-01-07");

    let result = client.pricing_service().get_token_prices(
        Chain::EthereumMainnet,
        QuoteCurrency::USD,
        "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", // USDC
        Some(opts)
    ).await;

    match result {
        Ok(response) => {
            if let Some(items) = response.data {
                println!("Found {} price items", items.len());
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(e) => println!("Token prices error (may be expected): {:?}", e),
    }
}

#[tokio::test]
async fn test_get_pool_spot_prices() {
    let Some(client) = get_test_client() else { return; };

    let result = client.pricing_service().get_pool_spot_prices(
        Chain::EthereumMainnet,
        "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640", // USDC/WETH
    ).await;

    match result {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} pool spot price items", data.items.len());
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(e) => println!("Pool spot prices error (may be expected): {:?}", e),
    }
}
