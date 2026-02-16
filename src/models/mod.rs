pub mod balances;
pub mod transactions;
pub mod nfts;
pub mod base;
pub mod pricing;
pub mod approvals;
pub mod bitcoin;
pub mod all_chains;

#[cfg(feature = "streaming")]
pub mod streaming;

use serde::Deserialize;

/// Pagination information returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct Pagination {
    /// Whether there are more pages available.
    pub has_more: Option<bool>,

    /// Current page number.
    pub page_number: Option<u32>,

    /// Number of items per page.
    pub page_size: Option<u32>,

    /// Total number of items available.
    pub total_count: Option<u64>,
}

/// Cursor-based pagination links returned by v3 endpoints.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationLinks {
    /// Previous page cursor URL.
    pub prev: Option<String>,

    /// Next page cursor URL.
    pub next: Option<String>,
}

/// Error information returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct ApiError {
    /// Error code from the API.
    pub code: Option<u32>,

    /// Human-readable error message.
    pub message: Option<String>,

    /// Additional error details.
    pub details: Option<serde_json::Value>,
}

/// Error response envelope from the API.
#[derive(Debug, Clone, Deserialize)]
pub struct ApiErrorEnvelope {
    /// Error information.
    pub error: Option<ApiError>,

    /// Additional metadata.
    pub meta: Option<serde_json::Value>,
}

/// Standard response wrapper for successful API responses.
#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse<T> {
    /// The main data payload.
    pub data: Option<T>,

    /// Error information (usually null for successful responses).
    pub error: Option<ApiError>,

    /// Pagination information for paginated endpoints.
    pub pagination: Option<Pagination>,

    /// Cursor-based pagination links (v3 endpoints).
    pub links: Option<PaginationLinks>,

    /// Additional metadata.
    pub meta: Option<serde_json::Value>,
}
