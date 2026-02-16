//! Service modules for the GoldRush SDK.
//!
//! Each service groups related API endpoints together.

pub mod balance_service;
pub mod transaction_service;
pub mod nft_service;
pub mod base_service;
pub mod pricing_service;
pub mod security_service;
pub mod bitcoin_service;
pub mod all_chains_service;

#[cfg(feature = "streaming")]
pub mod streaming_service;

use crate::{ClientConfig, Error, MetricsCollector};
use reqwest::{Client as HttpClient, Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::time::Duration;

/// Shared context for all service implementations.
pub(crate) struct ServiceContext {
    pub http: HttpClient,
    pub api_key: String,
    pub config: ClientConfig,
    pub metrics: Option<Arc<MetricsCollector>>,
}

impl ServiceContext {
    /// Build a request with the appropriate authentication and headers.
    pub fn build_request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = format!(
            "{}/{}",
            self.config.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        );

        self.http
            .request(method, &url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
    }

    /// Build a GET request with the given path.
    pub fn get(&self, path: &str) -> RequestBuilder {
        self.build_request(Method::GET, path)
    }

    /// Send a request with retry logic for transient failures.
    pub async fn send_with_retry<T>(&self, builder: RequestBuilder) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let mut attempt = 0u8;

        loop {
            let request = match builder.try_clone() {
                Some(req) => req,
                None => {
                    return Err(Error::Config("Failed to clone request for retry".to_string()));
                }
            };

            let response = request.send().await;

            match response {
                Err(e) => {
                    attempt += 1;
                    if attempt > self.config.max_retries {
                        return Err(Error::Http(e));
                    }

                    if self.should_retry_error(&e) {
                        let backoff_ms = self.calculate_backoff(attempt);
                        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                        continue;
                    } else {
                        return Err(Error::Http(e));
                    }
                }
                Ok(response) => {
                    let status = response.status();

                    if self.should_retry_status(status) {
                        attempt += 1;
                        if attempt > self.config.max_retries {
                            let text = response.text().await.unwrap_or_default();
                            return self.handle_error_response(status, text);
                        }

                        let backoff_ms = self.calculate_backoff(attempt);
                        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                        continue;
                    }

                    let text = response.text().await?;

                    if !status.is_success() {
                        return self.handle_error_response(status, text);
                    }

                    match serde_json::from_str::<T>(&text) {
                        Ok(parsed) => return Ok(parsed),
                        Err(e) => return Err(Error::Serialization(e)),
                    }
                }
            }
        }
    }

    fn should_retry_error(&self, error: &reqwest::Error) -> bool {
        error.is_timeout() || error.is_connect() || error.is_request()
    }

    fn should_retry_status(&self, status: StatusCode) -> bool {
        status.is_server_error() || status == StatusCode::TOO_MANY_REQUESTS
    }

    fn calculate_backoff(&self, attempt: u8) -> u64 {
        let base_delay = 200u64;
        let exponential_delay = base_delay * 2_u64.pow((attempt - 1) as u32);
        let jitter = (attempt as u64) * 50;
        std::cmp::min(exponential_delay + jitter, 5000)
    }

    fn handle_error_response<T>(&self, status: StatusCode, text: String) -> Result<T, Error> {
        let (code, message) = if let Ok(error_envelope) =
            serde_json::from_str::<crate::models::ApiErrorEnvelope>(&text)
        {
            if let Some(api_error) = error_envelope.error {
                (
                    api_error.code,
                    api_error.message.unwrap_or_else(|| text.clone()),
                )
            } else {
                (None, text.clone())
            }
        } else {
            (None, text)
        };

        Err(Error::Api {
            status: status.as_u16(),
            message,
            code,
        })
    }
}

pub use balance_service::BalanceService;
pub use transaction_service::TransactionService;
pub use nft_service::NftService;
pub use base_service::BaseService;
pub use pricing_service::PricingService;
pub use security_service::SecurityService;
pub use bitcoin_service::BitcoinService;
pub use all_chains_service::AllChainsService;

#[cfg(feature = "streaming")]
pub use streaming_service::StreamingService;
