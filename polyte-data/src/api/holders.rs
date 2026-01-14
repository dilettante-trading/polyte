use polyte_core::{QueryBuilder, Request};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::DataApiError;

/// Holders namespace for holder-related operations
#[derive(Clone)]
pub struct Holders {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
}

impl Holders {
    /// Get top holders for markets
    pub fn list(&self, markets: impl IntoIterator<Item = impl ToString>) -> ListHolders {
        let market_ids: Vec<String> = markets.into_iter().map(|s| s.to_string()).collect();
        let mut request = Request::new(self.client.clone(), self.base_url.clone(), "/holders");
        if !market_ids.is_empty() {
            request = request.query("market", market_ids.join(","));
        }

        ListHolders { request }
    }
}

/// Request builder for getting top holders
pub struct ListHolders {
    request: Request<Vec<MarketHolders>, DataApiError>,
}

impl ListHolders {
    /// Set maximum number of results per market (0-500, default: 100)
    pub fn limit(mut self, limit: u32) -> Self {
        self.request = self.request.query("limit", limit);
        self
    }

    /// Set minimum balance filter (0-999999, default: 1)
    pub fn min_balance(mut self, min_balance: u32) -> Self {
        self.request = self.request.query("minBalance", min_balance);
        self
    }

    /// Execute the request
    pub async fn send(self) -> Result<Vec<MarketHolders>, DataApiError> {
        self.request.send().await
    }
}

/// Market holders response containing token and its holders
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MarketHolders {
    /// Token identifier
    pub token: String,
    /// List of holders for this token
    pub holders: Vec<Holder>,
}

/// Individual holder of a market token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Holder {
    /// Proxy wallet address
    pub proxy_wallet: String,
    /// User bio
    pub bio: Option<String>,
    /// Asset identifier (token ID)
    pub asset: Option<String>,
    /// User pseudonym
    pub pseudonym: Option<String>,
    /// Amount held
    pub amount: f64,
    /// Whether username is displayed publicly
    pub display_username_public: Option<bool>,
    /// Outcome index (0 or 1 for binary markets)
    pub outcome_index: u32,
    /// User display name
    pub name: Option<String>,
    /// User profile image URL
    pub profile_image: Option<String>,
    /// Optimized profile image URL
    pub profile_image_optimized: Option<String>,
}
