use thiserror::Error;

/// WebSocket-specific errors.
#[derive(Debug, Error)]
pub enum WebSocketError {
    /// WebSocket connection error
    #[error("WebSocket connection error: {0}")]
    Connection(Box<tokio_tungstenite::tungstenite::Error>),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Connection was closed
    #[error("Connection closed")]
    ConnectionClosed,

    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Invalid message received
    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    /// URL parse error
    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),
}

impl From<tokio_tungstenite::tungstenite::Error> for WebSocketError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        WebSocketError::Connection(Box::new(err))
    }
}
