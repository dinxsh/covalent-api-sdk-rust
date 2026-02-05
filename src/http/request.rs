use crate::GoldRushClient;
use reqwest::{Method, RequestBuilder};

impl GoldRushClient {
    /// Build a request with the appropriate authentication and headers.
    pub(crate) fn build_request(&self, method: Method, path: &str) -> RequestBuilder {
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
    pub(crate) fn get(&self, path: &str) -> RequestBuilder {
        self.build_request(Method::GET, path)
    }

    /// Build a POST request with the given path.
    pub(crate) fn post(&self, path: &str) -> RequestBuilder {
        self.build_request(Method::POST, path)
    }

    /// Build a PUT request with the given path.
    pub(crate) fn put(&self, path: &str) -> RequestBuilder {
        self.build_request(Method::PUT, path)
    }

    /// Build a DELETE request with the given path.
    pub(crate) fn delete(&self, path: &str) -> RequestBuilder {
        self.build_request(Method::DELETE, path)
    }
}