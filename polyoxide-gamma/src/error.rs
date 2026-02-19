use polyoxide_core::{ApiError, RequestError};
use thiserror::Error;

/// Error types for gamma API operations
#[derive(Error, Debug)]
pub enum GammaError {
    /// Core API error
    #[error(transparent)]
    Api(#[from] ApiError),
}

impl RequestError for GammaError {
    async fn from_response(response: reqwest::Response) -> Self {
        Self::Api(ApiError::from_response(response).await)
    }
}

impl From<reqwest::Error> for GammaError {
    fn from(err: reqwest::Error) -> Self {
        Self::Api(ApiError::Network(err))
    }
}

impl From<url::ParseError> for GammaError {
    fn from(err: url::ParseError) -> Self {
        Self::Api(ApiError::Url(err))
    }
}
