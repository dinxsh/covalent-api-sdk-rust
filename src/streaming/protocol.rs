//! GraphQL-WS Protocol Implementation
//!
//! This module implements the graphql-ws subprotocol for WebSocket connections.
//! See: https://github.com/enisdenjo/graphql-ws/blob/master/PROTOCOL.md

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// Unique identifier for a GraphQL subscription
pub type SubscriptionId = String;

/// GraphQL-WS protocol message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GraphQLMessage {
    /// Client -> Server: Initiates the connection
    ConnectionInit {
        #[serde(skip_serializing_if = "Option::is_none")]
        payload: Option<Value>,
    },

    /// Server -> Client: Acknowledges the connection
    ConnectionAck {
        #[serde(skip_serializing_if = "Option::is_none")]
        payload: Option<Value>,
    },

    /// Client -> Server: Requests a subscription or query
    Subscribe {
        id: SubscriptionId,
        payload: SubscribePayload,
    },

    /// Server -> Client: Sends subscription data
    Next {
        id: SubscriptionId,
        payload: Value,
    },

    /// Server -> Client: Sends error for a subscription
    Error {
        id: SubscriptionId,
        payload: Vec<GraphQLError>,
    },

    /// Client -> Server or Server -> Client: Completes a subscription
    Complete {
        id: SubscriptionId,
    },

    /// Bidirectional: Connection keep-alive
    Ping {
        #[serde(skip_serializing_if = "Option::is_none")]
        payload: Option<Value>,
    },

    /// Bidirectional: Connection keep-alive response
    Pong {
        #[serde(skip_serializing_if = "Option::is_none")]
        payload: Option<Value>,
    },
}

/// Payload for a subscription request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribePayload {
    /// The GraphQL query or subscription
    pub query: String,

    /// Optional operation name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "operationName")]
    pub operation_name: Option<String>,

    /// Optional variables for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Value>,

    /// Optional extensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Value>,
}

/// GraphQL error structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLError {
    /// Error message
    pub message: String,

    /// Optional error locations in the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<ErrorLocation>>,

    /// Optional path to the field that caused the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<PathSegment>>,

    /// Optional extensions with additional error information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Value>,
}

/// Location in a GraphQL query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLocation {
    pub line: u32,
    pub column: u32,
}

/// Path segment in a GraphQL query result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PathSegment {
    Field(String),
    Index(usize),
}

impl GraphQLMessage {
    /// Creates a connection initialization message
    pub fn connection_init(payload: Option<Value>) -> Self {
        Self::ConnectionInit { payload }
    }

    /// Creates a subscription message
    pub fn subscribe(
        id: SubscriptionId,
        query: String,
        variables: Option<Value>,
        operation_name: Option<String>,
    ) -> Self {
        Self::Subscribe {
            id,
            payload: SubscribePayload {
                query,
                operation_name,
                variables,
                extensions: None,
            },
        }
    }

    /// Creates a complete message
    pub fn complete(id: SubscriptionId) -> Self {
        Self::Complete { id }
    }

    /// Creates a ping message
    pub fn ping(payload: Option<Value>) -> Self {
        Self::Ping { payload }
    }

    /// Creates a pong message
    pub fn pong(payload: Option<Value>) -> Self {
        Self::Pong { payload }
    }

    /// Serializes the message to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserializes a message from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Gets the subscription ID if this message has one
    pub fn subscription_id(&self) -> Option<&str> {
        match self {
            Self::Subscribe { id, .. }
            | Self::Next { id, .. }
            | Self::Error { id, .. }
            | Self::Complete { id } => Some(id),
            _ => None,
        }
    }

    /// Checks if this is a connection acknowledgment
    pub fn is_connection_ack(&self) -> bool {
        matches!(self, Self::ConnectionAck { .. })
    }

    /// Checks if this is a ping message
    pub fn is_ping(&self) -> bool {
        matches!(self, Self::Ping { .. })
    }

    /// Checks if this is a pong message
    pub fn is_pong(&self) -> bool {
        matches!(self, Self::Pong { .. })
    }
}

impl fmt::Display for GraphQLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(locations) = &self.locations {
            write!(f, " at ")?;
            for (i, loc) in locations.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}:{}", loc.line, loc.column)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_init_serialization() {
        let msg = GraphQLMessage::connection_init(None);
        let json = msg.to_json().unwrap();
        assert!(json.contains(r#""type":"connection_init"#));
    }

    #[test]
    fn test_subscribe_serialization() {
        let msg = GraphQLMessage::subscribe(
            "1".to_string(),
            "subscription { test }".to_string(),
            None,
            None,
        );
        let json = msg.to_json().unwrap();
        assert!(json.contains(r#""type":"subscribe"#));
        assert!(json.contains(r#""id":"1"#));
    }

    #[test]
    fn test_message_deserialization() {
        let json = r#"{"type":"connection_ack"}"#;
        let msg = GraphQLMessage::from_json(json).unwrap();
        assert!(msg.is_connection_ack());
    }

    #[test]
    fn test_next_message_with_payload() {
        let json = r#"{
            "type": "next",
            "id": "1",
            "payload": {"data": {"test": "value"}}
        }"#;
        let msg = GraphQLMessage::from_json(json).unwrap();
        assert_eq!(msg.subscription_id(), Some("1"));
    }

    #[test]
    fn test_error_message() {
        let json = r#"{
            "type": "error",
            "id": "1",
            "payload": [{
                "message": "Test error",
                "locations": [{"line": 1, "column": 1}]
            }]
        }"#;
        let msg = GraphQLMessage::from_json(json).unwrap();
        if let GraphQLMessage::Error { payload, .. } = msg {
            assert_eq!(payload[0].message, "Test error");
        } else {
            panic!("Expected Error message");
        }
    }
}
