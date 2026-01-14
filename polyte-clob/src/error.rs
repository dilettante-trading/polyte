use polyte_core::ApiError;
use thiserror::Error;

use crate::types::ParseTickSizeError;

/// Error types for CLOB API operations
#[derive(Error, Debug)]
pub enum ClobError {
    /// Core API error
    #[error(transparent)]
    Api(#[from] ApiError),

    /// Cryptographic operation failed
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// Alloy (Ethereum library) error
    #[error("Alloy error: {0}")]
    Alloy(String),

    /// Invalid tick size
    #[error(transparent)]
    InvalidTickSize(#[from] ParseTickSizeError),
}

impl ClobError {
    /// Create error from HTTP response
    pub(crate) async fn from_response(response: reqwest::Response) -> Self {
        Self::Api(ApiError::from_response(response).await)
    }

    /// Create validation error
    pub(crate) fn validation(msg: impl Into<String>) -> Self {
        Self::Api(ApiError::Validation(msg.into()))
    }
}

impl From<alloy::signers::Error> for ClobError {
    fn from(err: alloy::signers::Error) -> Self {
        Self::Alloy(err.to_string())
    }
}

impl From<alloy::hex::FromHexError> for ClobError {
    fn from(err: alloy::hex::FromHexError) -> Self {
        Self::Alloy(err.to_string())
    }
}

impl From<reqwest::Error> for ClobError {
    fn from(err: reqwest::Error) -> Self {
        Self::Api(ApiError::Network(err))
    }
}

impl From<url::ParseError> for ClobError {
    fn from(err: url::ParseError) -> Self {
        Self::Api(ApiError::Url(err))
    }
}

impl From<serde_json::Error> for ClobError {
    fn from(err: serde_json::Error) -> Self {
        Self::Api(ApiError::Serialization(err))
    }
}
