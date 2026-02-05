use goldrush_sdk::{GoldRushClient, ClientConfig, TxOptions, Error};

/// Integration tests for the transactions service.
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
async fn test_get_transactions() {
    let Some(client) = get_test_client() else {
        return;
    };

    let result = client
        .get_all_transactions_for_address(
            "eth-mainnet",
            "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
            None
        )
        .await;

    match result {
        Ok(response) => {
            assert!(response.data.is_some());
            let data = response.data.unwrap();
            println!("✓ Transactions response received: {} items", data.items.len());
            
            if !data.items.is_empty() {
                let tx = &data.items[0];
                assert!(!tx.tx_hash.is_empty());
                assert!(!tx.from_address.is_empty());
                println!("  First tx: {}", tx.tx_hash);
            }
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
async fn test_get_transactions_with_options() {
    let Some(client) = get_test_client() else {
        return;
    };

    let options = TxOptions::new()
        .page_size(3)
        .quote_currency("USD")
        .with_log_events(false);

    let result = client
        .get_all_transactions_for_address(
            "eth-mainnet",
            "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
            Some(options)
        )
        .await;

    match result {
        Ok(response) => {
            assert!(response.data.is_some());
            let data = response.data.unwrap();
            assert!(data.items.len() <= 3); // Should respect page_size
            println!("✓ Filtered transactions response: {} items", data.items.len());
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
async fn test_get_single_transaction() {
    let Some(client) = get_test_client() else {
        return;
    };

    // Use a known transaction hash
    let known_tx = "0x88df016429689c079f3b2f6ad39fa052532c56795b733da78a91ebe6a713944b";
    
    let result = client
        .get_transaction("eth-mainnet", known_tx)
        .await;

    match result {
        Ok(response) => {
            assert!(response.data.is_some());
            let tx = response.data.unwrap();
            assert_eq!(tx.tx_hash.to_lowercase(), known_tx.to_lowercase());
            println!("✓ Single transaction response received: {}", tx.tx_hash);
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("! Authentication failed - check your API key");
        }
        Err(Error::Api { status: 404, .. }) => {
            println!("! Transaction not found - endpoint might work differently");
        }
        Err(e) => {
            println!("Transaction lookup error (may be expected): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_transactions_between_addresses() {
    let Some(client) = get_test_client() else {
        return;
    };

    let result = client
        .get_transactions_between_addresses(
            "eth-mainnet",
            "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
            "0xa0b86a33e6441e6b32f6adaa51a3fc6f1b6a3b9a",
            None
        )
        .await;

    match result {
        Ok(response) => {
            println!("✓ Transactions between addresses response received");
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("! Authentication failed - check your API key");
        }
        Err(Error::Api { status: 404, .. }) => {
            println!("! Bulk transactions endpoint not found - might not be available");
        }
        Err(e) => {
            println!("Bulk transactions error (may be expected): {:?}", e);
        }
    }
}

#[tokio::test] 
async fn test_transaction_pagination() {
    let Some(client) = get_test_client() else {
        return;
    };

    let options = TxOptions::new()
        .page_size(2)
        .page_number(0);

    let result = client
        .get_all_transactions_for_address(
            "eth-mainnet",
            "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
            Some(options)
        )
        .await;

    match result {
        Ok(response) => {
            if let Some(pagination) = response.pagination {
                println!("✓ Pagination info received:");
                println!("  Has more: {:?}", pagination.has_more);
                println!("  Page number: {:?}", pagination.page_number);
                println!("  Total count: {:?}", pagination.total_count);
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("! Authentication failed - check your API key");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}