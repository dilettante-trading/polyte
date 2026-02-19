use polyoxide_core::{QueryBuilder, Request};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::DataApiError;

/// Builders namespace for builder-related operations
#[derive(Clone)]
pub struct BuildersApi {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
}

impl BuildersApi {
    /// Get the aggregated builder leaderboard
    pub fn leaderboard(&self) -> GetBuilderLeaderboard {
        let request = Request::new(
            self.client.clone(),
            self.base_url.clone(),
            "/v1/builders/leaderboard",
        );

        GetBuilderLeaderboard { request }
    }

    /// Get daily builder volume time series
    pub fn volume(&self) -> GetBuilderVolume {
        let request = Request::new(
            self.client.clone(),
            self.base_url.clone(),
            "/v1/builders/volume",
        );

        GetBuilderVolume { request }
    }
}

/// Request builder for getting the builder leaderboard
pub struct GetBuilderLeaderboard {
    request: Request<Vec<BuilderRanking>, DataApiError>,
}

impl GetBuilderLeaderboard {
    /// Set the aggregation time period (default: DAY)
    pub fn time_period(mut self, period: TimePeriod) -> Self {
        self.request = self.request.query("timePeriod", period);
        self
    }

    /// Set maximum number of results (0-50, default: 25)
    pub fn limit(mut self, limit: u32) -> Self {
        self.request = self.request.query("limit", limit);
        self
    }

    /// Set pagination offset (0-1000, default: 0)
    pub fn offset(mut self, offset: u32) -> Self {
        self.request = self.request.query("offset", offset);
        self
    }

    /// Execute the request
    pub async fn send(self) -> Result<Vec<BuilderRanking>, DataApiError> {
        self.request.send().await
    }
}

/// Time period for aggregation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimePeriod {
    /// Daily aggregation (default)
    #[default]
    Day,
    /// Weekly aggregation
    Week,
    /// Monthly aggregation
    Month,
    /// All time aggregation
    All,
}

impl std::fmt::Display for TimePeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Day => write!(f, "DAY"),
            Self::Week => write!(f, "WEEK"),
            Self::Month => write!(f, "MONTH"),
            Self::All => write!(f, "ALL"),
        }
    }
}

/// Builder ranking entry in the leaderboard
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BuilderRanking {
    /// Builder's ranking position
    pub rank: String,
    /// Builder identifier/name
    pub builder: String,
    /// Trading volume metric
    pub volume: f64,
    /// Count of active users
    pub active_users: u64,
    /// Verification status
    pub verified: bool,
    /// Logo image URL
    pub builder_logo: Option<String>,
}

/// Request builder for getting the builder volume time series
pub struct GetBuilderVolume {
    request: Request<Vec<BuilderVolume>, DataApiError>,
}

impl GetBuilderVolume {
    /// Set the time period filter (default: DAY)
    pub fn time_period(mut self, period: TimePeriod) -> Self {
        self.request = self.request.query("timePeriod", period);
        self
    }

    /// Execute the request
    pub async fn send(self) -> Result<Vec<BuilderVolume>, DataApiError> {
        self.request.send().await
    }
}

/// Builder volume entry in the time series
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BuilderVolume {
    /// Date/time of the volume record (ISO 8601)
    pub dt: String,
    /// Builder identifier/name
    pub builder: String,
    /// Logo image URL
    pub builder_logo: Option<String>,
    /// Verification status
    pub verified: bool,
    /// Trading volume metric
    pub volume: f64,
    /// Count of active users
    pub active_users: u64,
    /// Builder's ranking position
    pub rank: String,
}
