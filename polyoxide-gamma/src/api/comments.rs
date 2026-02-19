use polyoxide_core::{QueryBuilder, Request};
use reqwest::Client;
use url::Url;

use crate::{error::GammaError, types::Comment};

/// Comments namespace for comment-related operations
#[derive(Clone)]
pub struct Comments {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
}

impl Comments {
    /// List comments with optional filtering
    pub fn list(&self) -> ListComments {
        ListComments {
            request: Request::new(self.client.clone(), self.base_url.clone(), "/comments"),
        }
    }
}

/// Request builder for listing comments
pub struct ListComments {
    request: Request<Vec<Comment>, GammaError>,
}

impl ListComments {
    /// Set maximum number of results (minimum: 0)
    pub fn limit(mut self, limit: u32) -> Self {
        self.request = self.request.query("limit", limit);
        self
    }

    /// Set pagination offset (minimum: 0)
    pub fn offset(mut self, offset: u32) -> Self {
        self.request = self.request.query("offset", offset);
        self
    }

    /// Set order fields (comma-separated list)
    pub fn order(mut self, order: impl Into<String>) -> Self {
        self.request = self.request.query("order", order.into());
        self
    }

    /// Set sort direction
    pub fn ascending(mut self, ascending: bool) -> Self {
        self.request = self.request.query("ascending", ascending);
        self
    }

    /// Filter by parent entity type (Event, Series, market)
    pub fn parent_entity_type(mut self, entity_type: impl Into<String>) -> Self {
        self.request = self.request.query("parent_entity_type", entity_type.into());
        self
    }

    /// Filter by parent entity ID
    pub fn parent_entity_id(mut self, id: i64) -> Self {
        self.request = self.request.query("parent_entity_id", id);
        self
    }

    /// Include position data in response
    pub fn get_positions(mut self, include: bool) -> Self {
        self.request = self.request.query("get_positions", include);
        self
    }

    /// Restrict results to position holders only
    pub fn holders_only(mut self, holders_only: bool) -> Self {
        self.request = self.request.query("holders_only", holders_only);
        self
    }

    /// Execute the request
    pub async fn send(self) -> Result<Vec<Comment>, GammaError> {
        self.request.send().await
    }
}
