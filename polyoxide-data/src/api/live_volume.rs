use polyoxide_core::{HttpClient, QueryBuilder, Request};
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
        Request::<Vec<LiveVolume>, DataApiError>::new(self.http_client.clone(), "/live-volume")
            .query("id", event_id)
            .send()
            .await
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
