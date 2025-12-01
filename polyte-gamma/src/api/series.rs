use polyte_core::{QueryBuilder, Request};
use reqwest::Client;
use url::Url;

use crate::{error::GammaError, types::SeriesData};

/// Series namespace for series-related operations
#[derive(Clone)]
pub struct Series {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
}

impl Series {
    /// List series with optional filtering
    pub fn list(&self) -> ListSeries {
        ListSeries {
            request: Request::new(self.client.clone(), self.base_url.clone(), "/series"),
        }
    }

    /// Get a series by ID
    pub fn get(&self, id: impl Into<String>) -> Request<SeriesData, GammaError> {
        Request::new(
            self.client.clone(),
            self.base_url.clone(),
            format!("/series/{}", urlencoding::encode(&id.into())),
        )
    }
}

/// Request builder for listing series
pub struct ListSeries {
    request: Request<Vec<SeriesData>, GammaError>,
}

impl ListSeries {
    /// Limit the number of results
    pub fn limit(mut self, limit: u32) -> Self {
        self.request = self.request.query("limit", limit);
        self
    }

    /// Offset the results
    pub fn offset(mut self, offset: u32) -> Self {
        self.request = self.request.query("offset", offset);
        self
    }

    /// Sort in ascending order
    pub fn ascending(mut self, ascending: bool) -> Self {
        self.request = self.request.query("ascending", ascending);
        self
    }

    /// Filter by closed status
    pub fn closed(mut self, closed: bool) -> Self {
        self.request = self.request.query("closed", closed);
        self
    }

    /// Execute the request
    pub async fn send(self) -> crate::error::Result<Vec<SeriesData>> {
        self.request.send().await
    }
}
