use polyoxide_core::{HttpClient, RequestError};
use serde::{Deserialize, Serialize};

use crate::error::DataApiError;

/// LiveVolume namespace for live volume operations
#[derive(Clone)]
pub struct LiveVolumeApi {
    pub(crate) http_client: HttpClient,
}

impl LiveVolumeApi {
    /// Get live volume for an event
    pub async fn get(&self, event_id: u64) -> Result<Vec<LiveVolume>, DataApiError> {
        let url = self.http_client.base_url.join("/live-volume")?;
        let response = self
            .http_client
            .client
            .get(url)
            .query(&[("id", event_id)])
            .send()
            .await?;
        let status = response.status();

        if !status.is_success() {
            return Err(DataApiError::from_response(response).await);
        }

        let volume: Vec<LiveVolume> = response.json().await?;
        Ok(volume)
    }
}

/// Live volume for an event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveVolume {
    /// Total aggregated volume
    pub total: f64,
    /// Per-market volume breakdown
    pub markets: Vec<MarketVolume>,
}

/// Volume for a specific market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketVolume {
    /// Market condition ID
    pub market: String,
    /// Volume value
    pub value: f64,
}
