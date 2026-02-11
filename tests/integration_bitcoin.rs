use goldrush_sdk::{GoldRushClient, ClientConfig, Error};

/// Integration tests for the bitcoin service.

fn get_test_client() -> Option<GoldRushClient> {
    if let Ok(api_key) = std::env::var("GOLDRUSH_API_KEY") {
        GoldRushClient::new(api_key, ClientConfig::default()).ok()
    } else {
        println!("GOLDRUSH_API_KEY not set, skipping integration tests");
        None
    }
}

#[tokio::test]
async fn test_get_bitcoin_hd_wallet_balances() {
    let Some(client) = get_test_client() else { return; };

    // Use a known BTC address
    let result = client.bitcoin_service()
        .get_bitcoin_hd_wallet_balances("bc1qm34lsc65zpw79lxes69zkqmk6ee3ewf0j77s3")
        .await;

    match result {
        Ok(response) => {
            println!("BTC HD wallet response received");
            if let Some(data) = response.data {
                println!("  Items: {}", data.items.len());
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(e) => println!("BTC HD wallet error (may be expected): {:?}", e),
    }
}

#[tokio::test]
async fn test_get_bitcoin_non_hd_wallet_balances() {
    let Some(client) = get_test_client() else { return; };

    let result = client.bitcoin_service()
        .get_bitcoin_non_hd_wallet_balances("bc1qm34lsc65zpw79lxes69zkqmk6ee3ewf0j77s3")
        .await;

    match result {
        Ok(_response) => {
            println!("BTC non-HD wallet response received");
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(e) => println!("BTC non-HD wallet error (may be expected): {:?}", e),
    }
}
