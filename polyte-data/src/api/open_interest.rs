use polyte_core::RequestError;
use reqwest::Client;
use url::Url;

use crate::{error::DataApiError, types::OpenInterest};

/// OpenInterest namespace for open interest operations
#[derive(Clone)]
pub struct OpenInterestApi {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
}

impl OpenInterestApi {
    /// Get open interest for markets
    pub fn get(&self) -> GetOpenInterest {
        GetOpenInterest {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            markets: None,
        }
    }
}

/// Request builder for getting open interest
pub struct GetOpenInterest {
    client: Client,
    base_url: Url,
    markets: Option<Vec<String>>,
}

impl GetOpenInterest {
    /// Filter by specific market condition IDs
    pub fn market(mut self, condition_ids: impl IntoIterator<Item = impl ToString>) -> Self {
        let ids: Vec<String> = condition_ids.into_iter().map(|s| s.to_string()).collect();
        if !ids.is_empty() {
            self.markets = Some(ids);
        }
        self
    }

    /// Execute the request
    pub async fn send(self) -> Result<Vec<OpenInterest>, DataApiError> {
        let url = self.base_url.join("/oi")?;
        let mut request = self.client.get(url);

        if let Some(markets) = self.markets {
            request = request.query(&[("market", markets.join(","))]);
        }

        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() {
            return Err(DataApiError::from_response(response).await);
        }

        let oi: Vec<OpenInterest> = response.json().await?;
        Ok(oi)
    }
}
