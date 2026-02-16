//! WebSocket Client Implementation
//!
//! Core WebSocket client for GraphQL subscriptions with connection management,
//! automatic reconnection, and subscription multiplexing.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::{sleep, timeout};
use tokio_tungstenite::{
    connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream,
};
use tracing::{debug, error, info, instrument, warn};

use super::config::StreamingConfig;
use super::protocol::{GraphQLMessage, SubscriptionId};
use super::types::ConnectionState;
use crate::error::{Error, Result};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type MessageSender = mpsc::UnboundedSender<Result<Value>>;

/// WebSocket client for GraphQL subscriptions
#[derive(Clone)]
pub struct WebSocketClient {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    config: StreamingConfig,
    api_key: String,
    state: RwLock<ConnectionState>,
    subscriptions: RwLock<HashMap<SubscriptionId, MessageSender>>,
    reconnect_attempts: RwLock<u32>,
    control_tx: Mutex<Option<mpsc::UnboundedSender<ControlMessage>>>,
}

enum ControlMessage {
    Subscribe {
        id: SubscriptionId,
        query: String,
        variables: Option<Value>,
        sender: MessageSender,
    },
    Unsubscribe {
        id: SubscriptionId,
    },
    Disconnect,
}

impl WebSocketClient {
    /// Creates a new WebSocket client
    pub fn new(api_key: String, config: StreamingConfig) -> Self {
        Self {
            inner: Arc::new(ClientInner {
                config,
                api_key,
                state: RwLock::new(ConnectionState::Disconnected),
                subscriptions: RwLock::new(HashMap::new()),
                reconnect_attempts: RwLock::new(0),
                control_tx: Mutex::new(None),
            }),
        }
    }

    /// Connects to the WebSocket server
    #[instrument(skip(self))]
    pub async fn connect(&self) -> Result<()> {
        let mut state = self.inner.state.write().await;
        if *state == ConnectionState::Connected || *state == ConnectionState::Connecting {
            return Ok(());
        }

        *state = ConnectionState::Connecting;
        drop(state);

        if let Some(ref callback) = self.inner.config.on_connecting {
            callback();
        }

        let url = format!("{}?key={}", self.inner.config.ws_url, self.inner.api_key);

        match timeout(
            self.inner.config.connection_timeout,
            connect_async(&url),
        )
        .await
        {
            Ok(Ok((ws_stream, _))) => {
                info!("WebSocket connected to {}", self.inner.config.ws_url);

                let (control_tx, control_rx) = mpsc::unbounded_channel();

                // Store control sender
                {
                    let mut tx = self.inner.control_tx.lock().await;
                    *tx = Some(control_tx);
                }

                self.spawn_connection_handler(ws_stream, control_rx);

                let mut state = self.inner.state.write().await;
                *state = ConnectionState::Connected;
                drop(state);

                let mut attempts = self.inner.reconnect_attempts.write().await;
                *attempts = 0;
                drop(attempts);

                if let Some(ref callback) = self.inner.config.on_connected {
                    callback();
                }

                Ok(())
            }
            Ok(Err(e)) => {
                error!("WebSocket connection failed: {}", e);
                let mut state = self.inner.state.write().await;
                *state = ConnectionState::Failed;
                drop(state);

                let err = Error::WebSocket(format!("Connection failed: {}", e));
                if let Some(ref callback) = self.inner.config.on_error {
                    callback(&err);
                }
                Err(err)
            }
            Err(_) => {
                error!("WebSocket connection timeout");
                let mut state = self.inner.state.write().await;
                *state = ConnectionState::Failed;
                drop(state);

                let err = Error::WebSocket("Connection timeout".to_string());
                if let Some(ref callback) = self.inner.config.on_error {
                    callback(&err);
                }
                Err(err)
            }
        }
    }

    /// Subscribes to a GraphQL subscription
    #[instrument(skip(self, variables))]
    pub async fn subscribe(
        &self,
        query: String,
        variables: Option<Value>,
    ) -> Result<(SubscriptionId, mpsc::UnboundedReceiver<Result<Value>>)> {
        // Ensure connected
        if self.state().await != ConnectionState::Connected {
            self.connect().await?;
        }

        let id = uuid::Uuid::new_v4().to_string();
        let (tx, rx) = mpsc::unbounded_channel();

        // Store subscription
        {
            let mut subs = self.inner.subscriptions.write().await;
            subs.insert(id.clone(), tx.clone());
        }

        // Send subscribe message
        let control_tx = self.inner.control_tx.lock().await;
        if let Some(ref control_sender) = *control_tx {
            control_sender.send(ControlMessage::Subscribe {
                id: id.clone(),
                query,
                variables,
                sender: tx,
            })
            .map_err(|e| Error::Streaming(format!("Failed to send subscribe: {}", e)))?;
        } else {
            return Err(Error::Streaming("Connection not established".to_string()));
        }

        debug!("Subscribed with ID: {}", id);
        Ok((id, rx))
    }

    /// Unsubscribes from a subscription
    #[instrument(skip(self))]
    pub async fn unsubscribe(&self, id: &str) -> Result<()> {
        {
            let mut subs = self.inner.subscriptions.write().await;
            subs.remove(id);
        }

        let control_tx = self.inner.control_tx.lock().await;
        if let Some(ref tx) = *control_tx {
            tx.send(ControlMessage::Unsubscribe {
                id: id.to_string(),
            })
            .map_err(|e| Error::Streaming(format!("Failed to send unsubscribe: {}", e)))?;
        }

        debug!("Unsubscribed: {}", id);
        Ok(())
    }

    /// Disconnects the WebSocket connection
    #[instrument(skip(self))]
    pub async fn disconnect(&self) -> Result<()> {
        let control_tx = self.inner.control_tx.lock().await;
        if let Some(ref tx) = *control_tx {
            let _ = tx.send(ControlMessage::Disconnect);
        }

        let mut state = self.inner.state.write().await;
        *state = ConnectionState::Disconnected;
        drop(state);

        if let Some(ref callback) = self.inner.config.on_closed {
            callback();
        }

        info!("WebSocket disconnected");
        Ok(())
    }

    /// Gets the current connection state
    pub async fn state(&self) -> ConnectionState {
        *self.inner.state.read().await
    }

    /// Spawns the connection handler task
    fn spawn_connection_handler(
        &self,
        ws_stream: WsStream,
        mut control_rx: mpsc::UnboundedReceiver<ControlMessage>,
    ) {
        let inner = self.inner.clone();
        let config = self.inner.config.clone();

        tokio::spawn(async move {
            let (mut write, mut read) = ws_stream.split();

            // Send connection_init
            let init_msg = GraphQLMessage::connection_init(None);
            if let Err(e) = write.send(Message::Text(init_msg.to_json().unwrap())).await {
                error!("Failed to send init: {}", e);
                return;
            }

            let mut ping_interval = tokio::time::interval(config.ping_interval);
            ping_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                tokio::select! {
                    // Handle incoming messages
                    msg = read.next() => {
                        match msg {
                            Some(Ok(Message::Text(text))) => {
                                if let Err(e) = Self::handle_message(&inner, &text).await {
                                    error!("Error handling message: {}", e);
                                }
                            }
                            Some(Ok(Message::Close(_))) => {
                                info!("WebSocket closed by server");
                                break;
                            }
                            Some(Err(e)) => {
                                error!("WebSocket error: {}", e);
                                break;
                            }
                            None => {
                                warn!("WebSocket stream ended");
                                break;
                            }
                            _ => {}
                        }
                    }

                    // Handle control messages
                    msg = control_rx.recv() => {
                        match msg {
                            Some(ControlMessage::Subscribe { id, query, variables, sender: _ }) => {
                                let sub_msg = GraphQLMessage::subscribe(
                                    id.clone(),
                                    query,
                                    variables,
                                    None,
                                );
                                if let Err(e) = write.send(Message::Text(sub_msg.to_json().unwrap())).await {
                                    error!("Failed to send subscribe: {}", e);
                                }
                            }
                            Some(ControlMessage::Unsubscribe { id }) => {
                                let complete_msg = GraphQLMessage::complete(id);
                                if let Err(e) = write.send(Message::Text(complete_msg.to_json().unwrap())).await {
                                    error!("Failed to send complete: {}", e);
                                }
                            }
                            Some(ControlMessage::Disconnect) => {
                                break;
                            }
                            None => break,
                        }
                    }

                    // Send periodic pings
                    _ = ping_interval.tick() => {
                        let ping_msg = GraphQLMessage::ping(None);
                        if let Err(e) = write.send(Message::Text(ping_msg.to_json().unwrap())).await {
                            error!("Failed to send ping: {}", e);
                            break;
                        }
                    }
                }
            }

            // Cleanup
            let mut state = inner.state.write().await;
            *state = ConnectionState::Disconnected;
            drop(state);

            if let Some(ref callback) = config.on_closed {
                callback();
            }

            // Attempt reconnection if configured
            if config.auto_resubscribe {
                Self::attempt_reconnection(inner).await;
            }
        });
    }

    /// Handles incoming WebSocket messages
    async fn handle_message(inner: &Arc<ClientInner>, text: &str) -> Result<()> {
        let msg = GraphQLMessage::from_json(text)
            .map_err(|e| Error::Streaming(format!("Failed to parse message: {}", e)))?;

        match msg {
            GraphQLMessage::ConnectionAck { .. } => {
                debug!("Connection acknowledged");
            }
            GraphQLMessage::Next { id, payload } => {
                let subs = inner.subscriptions.read().await;
                if let Some(sender) = subs.get(&id) {
                    let _ = sender.send(Ok(payload));
                }
            }
            GraphQLMessage::Error { id, payload } => {
                let error_msg = payload
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                let subs = inner.subscriptions.read().await;
                if let Some(sender) = subs.get(&id) {
                    let _ = sender.send(Err(Error::GraphQL(error_msg.clone())));
                }
                drop(subs);

                let err = Error::GraphQL(error_msg);
                if let Some(ref callback) = inner.config.on_error {
                    callback(&err);
                }
            }
            GraphQLMessage::Complete { id } => {
                debug!("Subscription {} completed", id);
                let mut subs = inner.subscriptions.write().await;
                subs.remove(&id);
            }
            GraphQLMessage::Ping { .. } => {
                debug!("Received ping");
            }
            GraphQLMessage::Pong { .. } => {
                debug!("Received pong");
            }
            _ => {}
        }

        Ok(())
    }

    /// Attempts to reconnect with exponential backoff
    async fn attempt_reconnection(inner: Arc<ClientInner>) {
        let mut attempts = inner.reconnect_attempts.write().await;
        *attempts += 1;
        let attempt = *attempts;
        drop(attempts);

        if !(inner.config.should_retry)(attempt) {
            error!("Max reconnection attempts reached");
            return;
        }

        let backoff = Duration::from_secs(2u64.pow(attempt.min(5)));
        warn!(
            "Reconnecting in {} seconds (attempt {})",
            backoff.as_secs(),
            attempt
        );

        sleep(backoff).await;

        // Create a temporary client to attempt reconnection
        let client = WebSocketClient {
            inner: inner.clone(),
        };

        if let Err(e) = client.connect().await {
            error!("Reconnection failed: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = StreamingConfig::default();
        let client = WebSocketClient::new("test_key".to_string(), config);
        // Client should be created successfully
        assert!(true);
    }
}
