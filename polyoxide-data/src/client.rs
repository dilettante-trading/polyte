use polyoxide_core::{HttpClient, HttpClientBuilder, DEFAULT_POOL_SIZE, DEFAULT_TIMEOUT_MS};
use reqwest::Client;
use url::Url;

use crate::{
    api::{
        builders::BuildersApi,
        health::Health,
        holders::Holders,
        live_volume::LiveVolumeApi,
        open_interest::OpenInterestApi,
        trades::Trades,
        users::{UserApi, UserTraded},
    },
    error::DataApiError,
};

const DEFAULT_BASE_URL: &str = "https://data-api.polymarket.com";

/// Main Data API client
#[derive(Clone)]
pub struct DataApi {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
}

impl DataApi {
    /// Create a new Data API client with default configuration
    pub fn new() -> Result<Self, DataApiError> {
        Self::builder().build()
    }

    /// Create a builder for configuring the client
    pub fn builder() -> DataApiBuilder {
        DataApiBuilder::new()
    }

    /// Get health namespace
    pub fn health(&self) -> Health {
        Health {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
        }
    }

    /// Get user namespace for user-specific operations
    pub fn user(&self, user_address: impl Into<String>) -> UserApi {
        UserApi {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            user_address: user_address.into(),
        }
    }

    /// Alias for `user()` - for backwards compatibility
    pub fn positions(&self, user_address: impl Into<String>) -> UserApi {
        self.user(user_address)
    }

    /// Get traded namespace for backwards compatibility
    pub fn traded(&self, user_address: impl Into<String>) -> Traded {
        Traded {
            user_api: self.user(user_address),
        }
    }

    /// Get trades namespace
    pub fn trades(&self) -> Trades {
        Trades {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
        }
    }

    /// Get holders namespace
    pub fn holders(&self) -> Holders {
        Holders {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
        }
    }

    /// Get open interest namespace
    pub fn open_interest(&self) -> OpenInterestApi {
        OpenInterestApi {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
        }
    }

    /// Get live volume namespace
    pub fn live_volume(&self) -> LiveVolumeApi {
        LiveVolumeApi {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
        }
    }

    /// Get builders namespace
    pub fn builders(&self) -> BuildersApi {
        BuildersApi {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
        }
    }
}

/// Builder for configuring Data API client
pub struct DataApiBuilder {
    base_url: String,
    timeout_ms: u64,
    pool_size: usize,
}

impl DataApiBuilder {
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

    /// Build the Data API client
    pub fn build(self) -> Result<DataApi, DataApiError> {
        let HttpClient { client, base_url } = HttpClientBuilder::new(&self.base_url)
            .timeout_ms(self.timeout_ms)
            .pool_size(self.pool_size)
            .build()?;

        Ok(DataApi { client, base_url })
    }
}

impl Default for DataApiBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Wrapper for backwards compatibility with traded() API
pub struct Traded {
    user_api: UserApi,
}

impl Traded {
    /// Get total markets traded by the user
    pub async fn get(self) -> std::result::Result<UserTraded, DataApiError> {
        self.user_api.traded().await
    }
}
