use polyte_core::RequestError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::DataApiError;

/// Health namespace for API health operations
#[derive(Clone)]
pub struct Health {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
}

impl Health {
    /// Check API health status
    pub async fn check(&self) -> Result<HealthResponse, DataApiError> {
        let response = self.client.get(self.base_url.clone()).send().await?;
        let status = response.status();

        if !status.is_success() {
            return Err(DataApiError::from_response(response).await);
        }

        let health: HealthResponse = response.json().await?;
        Ok(health)
    }
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Status indicator (returns "OK" when healthy)
    pub data: String,
}
