//! Macros for reducing boilerplate in API client implementations

/// Implements standard error conversions for API error wrapper types
///
/// This macro generates `From` implementations for `reqwest::Error` and `url::ParseError`
/// that wrap them in the `Api` variant of the error type.
///
/// # Example
///
/// ```ignore
/// use polyoxide_core::{ApiError, RequestError};
/// use thiserror::Error;
///
/// #[derive(Error, Debug)]
/// pub enum MyApiError {
///     #[error(transparent)]
///     Api(#[from] ApiError),
/// }
///
/// impl RequestError for MyApiError {
///     async fn from_response(response: reqwest::Response) -> Self {
///         Self::Api(ApiError::from_response(response).await)
///     }
/// }
///
/// // Instead of writing these manually:
/// // impl From<reqwest::Error> for MyApiError { ... }
/// // impl From<url::ParseError> for MyApiError { ... }
///
/// // Use the macro:
/// polyoxide_core::impl_api_error_conversions!(MyApiError);
/// ```
#[macro_export]
macro_rules! impl_api_error_conversions {
    ($error_type:ty) => {
        impl From<reqwest::Error> for $error_type {
            fn from(err: reqwest::Error) -> Self {
                Self::Api($crate::ApiError::Network(err))
            }
        }

        impl From<url::ParseError> for $error_type {
            fn from(err: url::ParseError) -> Self {
                Self::Api($crate::ApiError::Url(err))
            }
        }
    };
}
