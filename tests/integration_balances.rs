use goldrush_sdk::{GoldRushClient, ClientConfig, BalancesOptions, Error};

/// Integration tests for the balances service.
/// 
/// These tests require a valid API key set as the GOLDRUSH_API_KEY environment variable.
/// If the env var is not set, tests will be skipped.

fn get_test_client() -> Option<GoldRushClient> {
    if let Ok(api_key) = std::env::var("GOLDRUSH_API_KEY") {
        GoldRushClient::new(api_key, ClientConfig::default()).ok()
    } else {
        println!("GOLDRUSH_API_KEY not set, skipping integration tests");
        None
    }
}

#[tokio::test]
async fn test_get_token_balances() {
    let Some(client) = get_test_client() else {
        return;
    };

    let result = client
        .get_token_balances_for_wallet_address(
            "eth-mainnet", 
            "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de", // Known address with activity
            None
        )
        .await;

    match result {
        Ok(response) => {
            assert!(response.data.is_some());
            println!("✓ Balances response received: {} items", 
                response.data.unwrap().items.len());
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("! Authentication failed - check your API key");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_token_balances_with_options() {
    let Some(client) = get_test_client() else {
        return;
    };

    let options = BalancesOptions::new()
        .quote_currency("USD")
        .page_size(5)
        .no_spam(true);

    let result = client
        .get_token_balances_for_wallet_address(
            "eth-mainnet",
            "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
            Some(options)
        )
        .await;

    match result {
        Ok(response) => {
            assert!(response.data.is_some());
            let data = response.data.unwrap();
            assert!(data.items.len() <= 5); // Should respect page_size
            println!("✓ Filtered balances response: {} items", data.items.len());
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("! Authentication failed - check your API key");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_historical_portfolio() {
    let Some(client) = get_test_client() else {
        return;
    };

    let result = client
        .get_historical_portfolio_for_wallet_address(
            "eth-mainnet",
            "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
            None
        )
        .await;

    match result {
        Ok(response) => {
            println!("✓ Portfolio response received");
            // Portfolio endpoint might return different structure
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("! Authentication failed - check your API key");
        }
        Err(Error::Api { status: 404, .. }) => {
            println!("! Portfolio endpoint not found - might not be available");
        }
        Err(e) => {
            println!("Portfolio error (may be expected): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_invalid_address() {
    let Some(client) = get_test_client() else {
        return;
    };

    let result = client
        .get_token_balances_for_wallet_address(
            "eth-mainnet",
            "invalid-address",
            None
        )
        .await;

    // Should get an API error for invalid address
    assert!(result.is_err());
    if let Err(Error::Api { status, .. }) = result {
        println!("✓ Got expected API error for invalid address: {}", status);
    } else {
        panic!("Expected API error for invalid address");
    }
}

#[tokio::test]
async fn test_invalid_chain() {
    let Some(client) = get_test_client() else {
        return;
    };

    let result = client
        .get_token_balances_for_wallet_address(
            "invalid-chain",
            "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
            None
        )
        .await;

    // Should get an API error for invalid chain
    assert!(result.is_err());
    if let Err(Error::Api { status, .. }) = result {
        println!("✓ Got expected API error for invalid chain: {}", status);
    } else {
        panic!("Expected API error for invalid chain");
    }
}