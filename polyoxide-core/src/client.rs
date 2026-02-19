use std::time::Duration;

use url::Url;

use crate::error::ApiError;

/// Default request timeout in milliseconds
pub const DEFAULT_TIMEOUT_MS: u64 = 30_000;
/// Default connection pool size per host
pub const DEFAULT_POOL_SIZE: usize = 10;

/// Shared HTTP client with base URL.
///
/// This is the common structure used by all API clients to hold
/// the configured reqwest client and base URL.
#[derive(Debug, Clone)]
pub struct HttpClient {
    /// The underlying reqwest HTTP client
    pub client: reqwest::Client,
    /// Base URL for API requests
    pub base_url: Url,
}

/// Builder for configuring HTTP clients.
///
/// Provides a consistent way to configure HTTP clients across all API crates
/// with sensible defaults.
///
/// # Example
///
/// ```
/// use polyoxide_core::HttpClientBuilder;
///
/// let client = HttpClientBuilder::new("https://api.example.com")
///     .timeout_ms(60_000)
///     .pool_size(20)
///     .build()
///     .unwrap();
/// ```
pub struct HttpClientBuilder {
    base_url: String,
    timeout_ms: u64,
    pool_size: usize,
}

impl HttpClientBuilder {
    /// Create a new HTTP client builder with the given base URL.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            timeout_ms: DEFAULT_TIMEOUT_MS,
            pool_size: DEFAULT_POOL_SIZE,
        }
    }

    /// Set request timeout in milliseconds.
    ///
    /// Default: 30,000ms (30 seconds)
    pub fn timeout_ms(mut self, timeout: u64) -> Self {
        self.timeout_ms = timeout;
        self
    }

    /// Set connection pool size per host.
    ///
    /// Default: 10 connections
    pub fn pool_size(mut self, size: usize) -> Self {
        self.pool_size = size;
        self
    }

    /// Build the HTTP client.
    pub fn build(self) -> Result<HttpClient, ApiError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(self.timeout_ms))
            .pool_max_idle_per_host(self.pool_size)
            .build()?;

        let base_url = Url::parse(&self.base_url)?;

        Ok(HttpClient { client, base_url })
    }
}

impl Default for HttpClientBuilder {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            timeout_ms: DEFAULT_TIMEOUT_MS,
            pool_size: DEFAULT_POOL_SIZE,
        }
    }
}
