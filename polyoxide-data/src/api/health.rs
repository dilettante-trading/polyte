use polyoxide_core::{HttpClient, RequestError};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

use crate::error::DataApiError;

/// Health namespace for API health operations
#[derive(Clone)]
pub struct Health {
    pub(crate) http_client: HttpClient,
}

impl Health {
    /// Check API health status
    pub async fn check(&self) -> Result<HealthResponse, DataApiError> {
        let response = self
            .http_client
            .client
            .get(self.http_client.base_url.clone())
            .send()
            .await?;
        let status = response.status();

        if !status.is_success() {
            return Err(DataApiError::from_response(response).await);
        }

        let health: HealthResponse = response.json().await?;
        Ok(health)
    }

    /// Measure the round-trip time (RTT) to the Polymarket Data API.
    ///
    /// Makes a GET request to the API root and returns the latency.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use polyoxide_data::DataApi;
    ///
    /// # async fn example() -> Result<(), polyoxide_data::DataApiError> {
    /// let client = DataApi::new()?;
    /// let latency = client.health().ping().await?;
    /// println!("API latency: {}ms", latency.as_millis());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ping(&self) -> Result<Duration, DataApiError> {
        let start = Instant::now();
        let response = self
            .http_client
            .client
            .get(self.http_client.base_url.clone())
            .send()
            .await?;
        let latency = start.elapsed();

        if !response.status().is_success() {
            return Err(DataApiError::from_response(response).await);
        }

        Ok(latency)
    }
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Status indicator (returns "OK" when healthy)
    pub data: String,
}
