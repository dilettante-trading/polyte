use thiserror::Error;

/// Core API error types shared across Polyte clients
#[derive(Error, Debug)]
pub enum ApiError {
    /// HTTP request failed
    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },

    /// Authentication failed (401/403)
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Request validation failed (400)
    #[error("Validation error: {0}")]
    Validation(String),

    /// Rate limit exceeded (429)
    #[error("Rate limit exceeded")]
    RateLimit,

    /// Request timeout
    #[error("Request timeout")]
    Timeout,

    /// Network error
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// URL parsing error
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),
}

impl ApiError {
    /// Create error from HTTP response
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = response.status().as_u16();

        // Get the raw response text first for debugging
        let body_text = response.text().await.unwrap_or_default();
        tracing::error!("API error response body: {}", body_text);

        let message = serde_json::from_str::<serde_json::Value>(&body_text)
            .ok()
            .and_then(|v| {
                v.get("error")
                    .or(v.get("message"))
                    .and_then(|m| m.as_str())
                    .map(String::from)
            })
            .unwrap_or_else(|| body_text.clone());

        match status {
            401 | 403 => Self::Authentication(message),
            400 => Self::Validation(message),
            429 => Self::RateLimit,
            408 => Self::Timeout,
            _ => Self::Api { status, message },
        }
    }
}
