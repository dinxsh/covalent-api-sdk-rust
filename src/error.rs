use thiserror::Error;

/// Error types that can occur when using the GoldRush SDK.
#[derive(Debug, Error)]
pub enum Error {
    /// The API key was not provided or is empty.
    #[error("missing API key")]
    MissingApiKey,

    /// HTTP client errors (network issues, timeouts, etc.).
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization errors.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// API errors returned by the GoldRush service.
    #[error("API error {status}: {message}")]
    Api {
        /// HTTP status code
        status: u16,
        /// Error message from the API
        message: String,
        /// Optional error code from the API
        code: Option<u32>,
    },

    /// Invalid configuration provided.
    #[error("configuration error: {0}")]
    Config(String),
}

/// Result type alias for GoldRush SDK operations.
pub type Result<T> = std::result::Result<T, Error>;