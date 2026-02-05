use crate::{Error, GoldRushClient};
use reqwest::{RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use std::time::Duration;

impl GoldRushClient {
    /// Send a request with retry logic for transient failures.
    ///
    /// This method will retry requests on certain HTTP status codes and network errors
    /// with exponential backoff.
    pub(crate) async fn send_with_retry<T>(&self, builder: RequestBuilder) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let mut attempt = 0u8;

        loop {
            // Clone the request builder for retry attempts
            let request = match builder.try_clone() {
                Some(req) => req,
                None => {
                    // Return a custom error since we can't clone the request
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
                    
                    // Only retry on certain error types
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
                    
                    // Check if we should retry based on status code
                    if self.should_retry_status(status) {
                        attempt += 1;
                        if attempt > self.config.max_retries {
                            let text = response.text().await.unwrap_or_default();
                            return self.handle_error_response(status, text).await;
                        }
                        
                        let backoff_ms = self.calculate_backoff(attempt);
                        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                        continue;
                    }

                    let text = response.text().await?;
                    
                    if !status.is_success() {
                        return self.handle_error_response(status, text).await;
                    }

                    // Parse successful response
                    match serde_json::from_str::<T>(&text) {
                        Ok(parsed) => return Ok(parsed),
                        Err(e) => return Err(Error::Serialization(e)),
                    }
                }
            }
        }
    }

    /// Determine if an error should be retried.
    fn should_retry_error(&self, error: &reqwest::Error) -> bool {
        // Retry on timeout, connection errors, and certain other network issues
        error.is_timeout() || error.is_connect() || error.is_request()
    }

    /// Determine if a status code should trigger a retry.
    fn should_retry_status(&self, status: StatusCode) -> bool {
        // Retry on 5xx server errors and 429 rate limiting
        status.is_server_error() || status == StatusCode::TOO_MANY_REQUESTS
    }

    /// Calculate exponential backoff delay in milliseconds.
    fn calculate_backoff(&self, attempt: u8) -> u64 {
        // Base delay of 200ms, exponentially increasing with jitter
        let base_delay = 200u64;
        let exponential_delay = base_delay * 2_u64.pow((attempt - 1) as u32);
        
        // Add some jitter to avoid thundering herd
        let jitter = (attempt as u64) * 50;
        
        std::cmp::min(exponential_delay + jitter, 5000) // Cap at 5 seconds
    }

    /// Handle error responses from the API.
    async fn handle_error_response<T>(&self, status: StatusCode, text: String) -> Result<T, Error> {
        // Try to parse the error response to get structured error information
        let (code, message) = if let Ok(error_envelope) = serde_json::from_str::<crate::models::ApiErrorEnvelope>(&text) {
            if let Some(api_error) = error_envelope.error {
                (api_error.code, api_error.message.unwrap_or_else(|| text.clone()))
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