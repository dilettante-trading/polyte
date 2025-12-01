use polyte_core::{QueryBuilder, Request};
use reqwest::Client;
use url::Url;

use crate::{error::GammaError, types::Tag};

/// Tags namespace for tag-related operations
#[derive(Clone)]
pub struct Tags {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
}

impl Tags {
    /// List tags with optional filtering
    pub fn list(&self) -> ListTags {
        ListTags {
            request: Request::new(self.client.clone(), self.base_url.clone(), "/tags"),
        }
    }

    /// Get a tag by ID
    pub fn get(&self, id: impl Into<String>) -> Request<Tag, GammaError> {
        Request::new(
            self.client.clone(),
            self.base_url.clone(),
            format!("/tags/{}", urlencoding::encode(&id.into())),
        )
    }

    /// Get a tag by slug
    pub fn get_by_slug(&self, slug: impl Into<String>) -> Request<Tag, GammaError> {
        Request::new(
            self.client.clone(),
            self.base_url.clone(),
            format!("/tags/slug/{}", urlencoding::encode(&slug.into())),
        )
    }

    /// Get related tags by tag ID
    pub fn get_related(&self, id: impl Into<String>) -> Request<Vec<Tag>, GammaError> {
        Request::new(
            self.client.clone(),
            self.base_url.clone(),
            format!("/tags/{}/related-tags", urlencoding::encode(&id.into())),
        )
    }

    /// Get related tags by tag slug
    pub fn get_related_by_slug(&self, slug: impl Into<String>) -> Request<Vec<Tag>, GammaError> {
        Request::new(
            self.client.clone(),
            self.base_url.clone(),
            format!(
                "/tags/slug/{}/related-tags",
                urlencoding::encode(&slug.into())
            ),
        )
    }
}

/// Request builder for listing tags
pub struct ListTags {
    request: Request<Vec<Tag>, GammaError>,
}

impl ListTags {
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

    /// Include template data in response
    pub fn include_template(mut self, include: bool) -> Self {
        self.request = self.request.query("include_template", include);
        self
    }

    /// Filter by carousel status
    pub fn is_carousel(mut self, is_carousel: bool) -> Self {
        self.request = self.request.query("is_carousel", is_carousel);
        self
    }

    /// Execute the request
    pub async fn send(self) -> crate::error::Result<Vec<Tag>> {
        self.request.send().await
    }
}
