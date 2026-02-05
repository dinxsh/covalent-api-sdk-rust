use tracing::{debug, error, info, instrument, warn};
use uuid::Uuid;

/// Request correlation ID for tracing requests across the system.
#[derive(Debug, Clone)]
pub struct RequestId(pub String);

impl RequestId {
    /// Generate a new random request ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    
    /// Create a request ID from an existing string.
    pub fn from_string<S: Into<String>>(id: S) -> Self {
        Self(id.into())
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Tracing context for SDK operations.
#[derive(Debug, Clone)]
pub struct TracingContext {
    pub request_id: RequestId,
    pub operation: String,
    pub chain_name: Option<String>,
}

impl TracingContext {
    pub fn new<S: Into<String>>(operation: S) -> Self {
        Self {
            request_id: RequestId::new(),
            operation: operation.into(),
            chain_name: None,
        }
    }
    
    pub fn with_chain<S: Into<String>>(mut self, chain_name: S) -> Self {
        self.chain_name = Some(chain_name.into());
        self
    }
}

/// Macro for creating instrumented spans with request context.
#[macro_export]
macro_rules! trace_request {
    ($ctx:expr, $msg:expr) => {
        tracing::info!(
            request_id = %$ctx.request_id,
            operation = %$ctx.operation,
            chain = ?$ctx.chain_name,
            $msg
        )
    };
    ($ctx:expr, $msg:expr, $($field:tt)*) => {
        tracing::info!(
            request_id = %$ctx.request_id,
            operation = %$ctx.operation,
            chain = ?$ctx.chain_name,
            $msg,
            $($field)*
        )
    };
}

/// Macro for error logging with context.
#[macro_export]
macro_rules! trace_error {
    ($ctx:expr, $err:expr, $msg:expr) => {
        tracing::error!(
            request_id = %$ctx.request_id,
            operation = %$ctx.operation,
            chain = ?$ctx.chain_name,
            error = %$err,
            $msg
        )
    };
}

/// Instrument a function with tracing context.
pub fn instrument_request<F, T>(ctx: &TracingContext, f: F) -> T
where
    F: FnOnce() -> T,
{
    let span = tracing::info_span!(
        "sdk_request",
        request_id = %ctx.request_id,
        operation = %ctx.operation,
        chain = ?ctx.chain_name
    );
    
    let _enter = span.enter();
    f()
}