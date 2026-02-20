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

// Implement standard error conversions using the macro
polyoxide_core::impl_api_error_conversions!(DataApiError);
