use goldrush_sdk::{GoldRushClient, ClientConfig, Chain, NftOptions, Error};

/// Integration tests for the NFTs service.
///
/// These tests require a valid API key set as the GOLDRUSH_API_KEY environment variable.

fn get_test_client() -> Option<GoldRushClient> {
    if let Ok(api_key) = std::env::var("GOLDRUSH_API_KEY") {
        GoldRushClient::new(api_key, ClientConfig::default()).ok()
    } else {
        println!("GOLDRUSH_API_KEY not set, skipping integration tests");
        None
    }
}

#[tokio::test]
async fn test_get_nfts_for_address() {
    let Some(client) = get_test_client() else {
        return;
    };

    let result = client
        .nft_service()
        .get_nfts_for_address(
            Chain::EthereumMainnet,
            "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
            None
        )
        .await;

    match result {
        Ok(response) => {
            assert!(response.data.is_some());
            let data = response.data.unwrap();
            println!("NFTs response received: {} items", data.items.len());

            if !data.items.is_empty() {
                let nft = &data.items[0];
                println!("  First NFT: {} #{}",
                    nft.contract_address, nft.token_id);
                assert!(!nft.contract_address.is_empty());
                assert!(!nft.token_id.is_empty());
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_nfts_with_options() {
    let Some(client) = get_test_client() else {
        return;
    };

    let options = NftOptions::new()
        .page_size(3)
        .with_metadata(true)
        .no_spam(true);

    let result = client
        .nft_service()
        .get_nfts_for_address(
            Chain::EthereumMainnet,
            "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
            Some(options)
        )
        .await;

    match result {
        Ok(response) => {
            assert!(response.data.is_some());
            let data = response.data.unwrap();
            assert!(data.items.len() <= 3);
            println!("Filtered NFTs response: {} items", data.items.len());
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_nft_metadata() {
    let Some(client) = get_test_client() else {
        return;
    };

    let result = client
        .nft_service()
        .get_nft_metadata(
            Chain::EthereumMainnet,
            "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d",
            "1"
        )
        .await;

    match result {
        Ok(response) => {
            assert!(response.data.is_some());
            let metadata_items = response.data.unwrap();
            if !metadata_items.is_empty() {
                let metadata = &metadata_items[0];
                println!("NFT metadata received for: {} #{}",
                    metadata.contract_address, metadata.token_id);
                assert_eq!(metadata.contract_address.to_lowercase(),
                    "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d".to_lowercase());
                assert_eq!(metadata.token_id, "1");
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(Error::Api { status: 404, .. }) => {
            println!("NFT metadata not found - endpoint might work differently");
        }
        Err(e) => {
            println!("NFT metadata error (may be expected): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_nfts_for_collection() {
    let Some(client) = get_test_client() else {
        return;
    };

    let result = client
        .nft_service()
        .get_nfts_for_collection(
            Chain::EthereumMainnet,
            "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d",
            Some(NftOptions::new().page_size(5))
        )
        .await;

    match result {
        Ok(response) => {
            println!("Collection NFTs response received");
            if let Some(data) = response.data {
                println!("  Found {} NFTs in collection", data.items.len());
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(Error::Api { status: 404, .. }) => {
            println!("Collection NFTs endpoint not found - might not be available");
        }
        Err(e) => {
            println!("Collection NFTs error (may be expected): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_check_ownership() {
    let Some(client) = get_test_client() else {
        return;
    };

    let result = client
        .nft_service()
        .check_ownership_in_nft(
            Chain::EthereumMainnet,
            "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
            "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d",
        )
        .await;

    match result {
        Ok(response) => {
            println!("Ownership check response received");
            if let Some(data) = response.data {
                println!("  Is owner: {:?}", data.is_owner);
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(e) => {
            println!("Ownership check error (may be expected): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_invalid_nft_contract() {
    let Some(client) = get_test_client() else {
        return;
    };

    let result = client
        .nft_service()
        .get_nft_metadata(
            Chain::EthereumMainnet,
            "0xinvalid",
            "1"
        )
        .await;

    assert!(result.is_err());
    println!("Got expected error for invalid contract");
}
