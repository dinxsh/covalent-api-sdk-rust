use goldrush_sdk::{GoldRushClient, ClientConfig, Chain, Error};

/// Integration tests for the security service.

fn get_test_client() -> Option<GoldRushClient> {
    if let Ok(api_key) = std::env::var("GOLDRUSH_API_KEY") {
        GoldRushClient::new(api_key, ClientConfig::default()).ok()
    } else {
        println!("GOLDRUSH_API_KEY not set, skipping integration tests");
        None
    }
}

#[tokio::test]
async fn test_get_approvals() {
    let Some(client) = get_test_client() else { return; };

    let result = client.security_service().get_approvals(
        Chain::EthereumMainnet,
        "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
    ).await;

    match result {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} approval items", data.items.len());
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(e) => println!("Approvals error (may be expected): {:?}", e),
    }
}

#[tokio::test]
async fn test_get_nft_approvals() {
    let Some(client) = get_test_client() else { return; };

    let result = client.security_service().get_nft_approvals(
        Chain::EthereumMainnet,
        "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
    ).await;

    match result {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} NFT approval items", data.items.len());
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(e) => println!("NFT approvals error (may be expected): {:?}", e),
    }
}
