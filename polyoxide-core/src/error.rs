use thiserror::Error;

/// Core API error types shared across Polyoxide clients
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
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

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
        tracing::debug!("API error response body: {}", body_text);

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
            429 => Self::RateLimit(message),
            408 => Self::Timeout,
            _ => Self::Api { status, message },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_error_carries_message() {
        let err = ApiError::RateLimit("too many requests, retry after 5s".to_string());
        let display = format!("{}", err);
        assert!(
            display.contains("too many requests"),
            "RateLimit display should contain the message: {}",
            display
        );
    }

    #[test]
    fn test_rate_limit_error_display_format() {
        let err = ApiError::RateLimit("slow down".to_string());
        assert_eq!(format!("{}", err), "Rate limit exceeded: slow down");
    }
}
