//! Query parameter trait for eliminating repetitive Option handling.

use reqwest::RequestBuilder;

/// Trait for applying query parameters to a request builder.
///
/// Implement this on Options structs to eliminate repetitive `if let Some(x)` boilerplate.
pub(crate) trait QueryParams {
    fn apply_to(self, builder: RequestBuilder) -> RequestBuilder;
}
