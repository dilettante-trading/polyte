use polyoxide_core::{HttpClient, QueryBuilder, Request};

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
            request: Request::new(self.http_client.clone(), "/oi"),
        }
    }
}

/// Request builder for getting open interest
pub struct GetOpenInterest {
    request: Request<Vec<OpenInterest>, DataApiError>,
}

impl GetOpenInterest {
    /// Filter by specific market condition IDs
    pub fn market(mut self, condition_ids: impl IntoIterator<Item = impl ToString>) -> Self {
        let ids: Vec<String> = condition_ids.into_iter().map(|s| s.to_string()).collect();
        if !ids.is_empty() {
            self.request = self.request.query("market", ids.join(","));
        }
        self
    }

    /// Execute the request
    pub async fn send(self) -> Result<Vec<OpenInterest>, DataApiError> {
        self.request.send().await
    }
}
