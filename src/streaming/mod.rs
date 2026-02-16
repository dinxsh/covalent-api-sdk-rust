//! Streaming Module
//!
//! WebSocket-based GraphQL streaming for real-time data subscriptions.

pub mod client;
pub mod config;
pub mod protocol;
pub mod types;

pub use client::WebSocketClient;
pub use config::{StreamingConfig, StreamingConfigBuilder};
pub use types::{ConnectionState, SubscriptionHandle};
