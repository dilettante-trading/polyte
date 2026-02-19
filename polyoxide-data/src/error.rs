use polyoxide_core::{ApiError, RequestError};
use thiserror::Error;

/// Error types for Data API operations
#[derive(Error, Debug)]
pub enum DataApiError {
    /// Core API error
    #[error(transparent)]
    Api(#[from] ApiError),
}

impl RequestError for DataApiError {
    async fn from_response(response: reqwest::Response) -> Self {
        Self::Api(ApiError::from_response(response).await)
    }
}

impl From<reqwest::Error> for DataApiError {
    fn from(err: reqwest::Error) -> Self {
        Self::Api(ApiError::Network(err))
    }
}

impl From<url::ParseError> for DataApiError {
    fn from(err: url::ParseError) -> Self {
        Self::Api(ApiError::Url(err))
    }
}
