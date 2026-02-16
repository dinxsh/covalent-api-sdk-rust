//! Streaming Types
//!
//! Common types for streaming functionality.

use std::sync::Arc;
use tokio::sync::Mutex;

use super::protocol::SubscriptionId;

/// Handle for managing a subscription
#[derive(Clone)]
pub struct SubscriptionHandle {
    id: SubscriptionId,
    client: Arc<Mutex<Option<super::client::WebSocketClient>>>,
}

impl SubscriptionHandle {
    /// Creates a new subscription handle
    pub(crate) fn new(
        id: SubscriptionId,
        client: Arc<Mutex<Option<super::client::WebSocketClient>>>,
    ) -> Self {
        Self { id, client }
    }

    /// Gets the subscription ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Unsubscribes from the stream
    pub async fn unsubscribe(self) -> Result<(), crate::error::GoldRushError> {
        let client_guard = self.client.lock().await;
        if let Some(client) = client_guard.as_ref() {
            client.unsubscribe(&self.id).await?;
        }
        Ok(())
    }
}

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Not connected
    Disconnected,
    /// Attempting to connect
    Connecting,
    /// Connected and ready
    Connected,
    /// Connection failed
    Failed,
}

impl ConnectionState {
    /// Checks if the connection is active
    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Connected)
    }

    /// Checks if the connection is attempting to connect
    pub fn is_connecting(&self) -> bool {
        matches!(self, Self::Connecting)
    }
}

impl std::fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disconnected => write!(f, "disconnected"),
            Self::Connecting => write!(f, "connecting"),
            Self::Connected => write!(f, "connected"),
            Self::Failed => write!(f, "failed"),
        }
    }
}
