use polyoxide_core::{HttpClient, RequestError};

use crate::{error::DataApiError, types::OpenInterest};

/// OpenInterest namespace for open interest operations
#[derive(Clone)]
pub struct OpenInterestApi {
    pub(crate) http_client: HttpClient,
}

impl OpenInterestApi {
    /// Get open interest for markets
    pub fn get(&self) -> GetOpenInterest {
        GetOpenInterest {
            http_client: self.http_client.clone(),
            markets: None,
        }
    }
}

/// Request builder for getting open interest
pub struct GetOpenInterest {
    http_client: HttpClient,
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
        let url = self.http_client.base_url.join("/oi")?;
        let mut request = self.http_client.client.get(url);

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
