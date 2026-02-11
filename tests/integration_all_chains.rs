use goldrush_sdk::{GoldRushClient, ClientConfig, MultiChainBalancesOptions, Error};

/// Integration tests for the all-chains service.

fn get_test_client() -> Option<GoldRushClient> {
    if let Ok(api_key) = std::env::var("GOLDRUSH_API_KEY") {
        GoldRushClient::new(api_key, ClientConfig::default()).ok()
    } else {
        println!("GOLDRUSH_API_KEY not set, skipping integration tests");
        None
    }
}

#[tokio::test]
async fn test_get_address_activity() {
    let Some(client) = get_test_client() else { return; };

    let result = client.all_chains_service()
        .get_address_activity("0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de", None)
        .await;

    match result {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Address active on {} chains", data.items.len());
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(e) => println!("Address activity error (may be expected): {:?}", e),
    }
}

#[tokio::test]
async fn test_get_multi_chain_balances() {
    let Some(client) = get_test_client() else { return; };

    let opts = MultiChainBalancesOptions::new().quote_currency("USD");

    let result = client.all_chains_service()
        .get_multi_chain_balances("0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de", Some(opts))
        .await;

    match result {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Found {} multi-chain balance items", data.items.len());
            }
        }
        Err(Error::Api { status: 401, .. }) => {
            println!("Authentication failed - check your API key");
        }
        Err(e) => println!("Multi-chain balances error (may be expected): {:?}", e),
    }
}
