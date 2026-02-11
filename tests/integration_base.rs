use goldrush_sdk::{GoldRushClient, ClientConfig, Chain, Error};

/// Integration tests for the base service.

fn get_test_client() -> Option<GoldRushClient> {
    if let Ok(api_key) = std::env::var("GOLDRUSH_API_KEY") {
        GoldRushClient::new(api_key, ClientConfig::default()).ok()
    } else {
        println!("GOLDRUSH_API_KEY not set, skipping integration tests");
        None
    }
}

#[tokio::test]
async fn test_get_all_chains() {
    let Some(client) = get_test_client() else { return; };

    let result = client.base_service().get_all_chains().await;

    match result {
        Ok(response) => {
            if let Some(data) = response.data {
                assert!(!data.items.is_empty());
                println!("Found {} chains", data.items.len());
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[tokio::test]
async fn test_get_block() {
    let Some(client) = get_test_client() else { return; };

    let result = client.base_service().get_block(Chain::EthereumMainnet, "18000000").await;

    match result {
        Ok(response) => {
            println!("Block response received");
            if let Some(data) = response.data {
                for block in &data.items {
                    println!("  Block height: {:?}", block.height);
                }
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(e) => println!("Block error (may be expected): {:?}", e),
    }
}

#[tokio::test]
async fn test_get_block_heights() {
    let Some(client) = get_test_client() else { return; };

    let result = client.base_service()
        .get_block_heights(Chain::EthereumMainnet, "2024-01-01", "2024-01-02", None)
        .await;

    match result {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} blocks in date range", data.items.len());
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(e) => println!("Block heights error (may be expected): {:?}", e),
    }
}

#[tokio::test]
async fn test_get_gas_prices() {
    let Some(client) = get_test_client() else { return; };

    let result = client.base_service()
        .get_gas_prices(Chain::EthereumMainnet, "erc20")
        .await;

    match result {
        Ok(response) => {
            println!("Gas prices response received");
            if let Some(data) = response.data {
                println!("  Found {} gas price items", data.items.len());
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(e) => println!("Gas prices error (may be expected): {:?}", e),
    }
}
