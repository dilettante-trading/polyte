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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_api_error_from_api_error() {
        let api_err = ApiError::Api {
            status: 404,
            message: "not found".to_string(),
        };
        let data_err = DataApiError::from(api_err);
        let msg = format!("{}", data_err);
        assert!(
            msg.contains("not found"),
            "DataApiError should forward ApiError message: {}",
            msg
        );
    }

    #[test]
    fn data_api_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        // DataApiError wraps ApiError which should be Send + Sync
        // This is a compile-time check
        assert_send_sync::<DataApiError>();
    }
}
