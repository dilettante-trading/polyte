use polyoxide_core::{
    HttpClient, HttpClientBuilder, RateLimiter, DEFAULT_POOL_SIZE, DEFAULT_TIMEOUT_MS,
};

use crate::{
    api::{
        comments::Comments, events::Events, health::Health, markets::Markets, series::Series,
        sports::Sports, tags::Tags, user::User,
    },
    error::GammaError,
};

const DEFAULT_BASE_URL: &str = "https://gamma-api.polymarket.com";

/// Main Gamma API client
#[derive(Clone)]
pub struct Gamma {
    pub(crate) http_client: HttpClient,
}

impl Gamma {
    /// Create a new Gamma client with default configuration
    pub fn new() -> Result<Self, GammaError> {
        Self::builder().build()
    }

    /// Create a builder for configuring the client
    pub fn builder() -> GammaBuilder {
        GammaBuilder::new()
    }

    /// Get markets namespace
    pub fn markets(&self) -> Markets {
        Markets {
            http_client: self.http_client.clone(),
        }
    }

    /// Get events namespace
    pub fn events(&self) -> Events {
        Events {
            http_client: self.http_client.clone(),
        }
    }

    /// Get series namespace
    pub fn series(&self) -> Series {
        Series {
            http_client: self.http_client.clone(),
        }
    }

    /// Get tags namespace
    pub fn tags(&self) -> Tags {
        Tags {
            http_client: self.http_client.clone(),
        }
    }

    /// Get sports namespace
    pub fn sports(&self) -> Sports {
        Sports {
            http_client: self.http_client.clone(),
        }
    }

    /// Get comments namespace
    pub fn comments(&self) -> Comments {
        Comments {
            http_client: self.http_client.clone(),
        }
    }

    /// Get user namespace
    pub fn user(&self) -> User {
        User {
            http_client: self.http_client.clone(),
        }
    }

    /// Get health namespace
    pub fn health(&self) -> Health {
        Health {
            http_client: self.http_client.clone(),
        }
    }
}

/// Builder for configuring Gamma client
pub struct GammaBuilder {
    base_url: String,
    timeout_ms: u64,
    pool_size: usize,
}

impl GammaBuilder {
    fn new() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout_ms: DEFAULT_TIMEOUT_MS,
            pool_size: DEFAULT_POOL_SIZE,
        }
    }

    /// Set base URL for the API
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set request timeout in milliseconds
    pub fn timeout_ms(mut self, timeout: u64) -> Self {
        self.timeout_ms = timeout;
        self
    }

    /// Set connection pool size
    pub fn pool_size(mut self, size: usize) -> Self {
        self.pool_size = size;
        self
    }

    /// Build the Gamma client
    pub fn build(self) -> Result<Gamma, GammaError> {
        let http_client = HttpClientBuilder::new(&self.base_url)
            .timeout_ms(self.timeout_ms)
            .pool_size(self.pool_size)
            .with_rate_limiter(RateLimiter::gamma_default())
            .build()?;

        Ok(Gamma { http_client })
    }
}

impl Default for GammaBuilder {
    fn default() -> Self {
        Self::new()
    }
}
