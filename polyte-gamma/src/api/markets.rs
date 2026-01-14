use polyte_core::{QueryBuilder, Request};
use reqwest::Client;
use url::Url;

use crate::{error::GammaError, types::Market};

/// Markets namespace for market-related operations
#[derive(Clone)]
pub struct Markets {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
}

impl Markets {
    /// Get a specific market by ID
    pub fn get(&self, id: impl Into<String>) -> Request<Market, GammaError> {
        Request::new(
            self.client.clone(),
            self.base_url.clone(),
            format!("/markets/{}", urlencoding::encode(&id.into())),
        )
    }

    /// Get a market by its slug
    pub fn get_by_slug(&self, slug: impl Into<String>) -> Request<Market, GammaError> {
        Request::new(
            self.client.clone(),
            self.base_url.clone(),
            format!("/markets/slug/{}", urlencoding::encode(&slug.into())),
        )
    }

    /// List markets with optional filtering
    pub fn list(&self) -> ListMarkets {
        ListMarkets {
            request: Request::new(self.client.clone(), self.base_url.clone(), "/markets"),
        }
    }
}

/// Request builder for listing markets
pub struct ListMarkets {
    request: Request<Vec<Market>, GammaError>,
}

impl ListMarkets {
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

    /// Filter by specific market IDs
    pub fn id(mut self, ids: impl IntoIterator<Item = i64>) -> Self {
        self.request = self.request.query_many("id", ids);
        self
    }

    /// Filter by market slugs
    pub fn slug(mut self, slugs: impl IntoIterator<Item = impl ToString>) -> Self {
        self.request = self.request.query_many("slug", slugs);
        self
    }

    /// Filter by CLOB token IDs
    pub fn clob_token_ids(mut self, token_ids: impl IntoIterator<Item = impl ToString>) -> Self {
        self.request = self.request.query_many("clob_token_ids", token_ids);
        self
    }

    /// Filter by condition IDs
    pub fn condition_ids(mut self, condition_ids: impl IntoIterator<Item = impl ToString>) -> Self {
        self.request = self.request.query_many("condition_ids", condition_ids);
        self
    }

    /// Filter by market maker addresses
    pub fn market_maker_address(
        mut self,
        addresses: impl IntoIterator<Item = impl ToString>,
    ) -> Self {
        self.request = self.request.query_many("market_maker_address", addresses);
        self
    }

    /// Set minimum liquidity threshold
    pub fn liquidity_num_min(mut self, min: f64) -> Self {
        self.request = self.request.query("liquidity_num_min", min);
        self
    }

    /// Set maximum liquidity threshold
    pub fn liquidity_num_max(mut self, max: f64) -> Self {
        self.request = self.request.query("liquidity_num_max", max);
        self
    }

    /// Set minimum trading volume
    pub fn volume_num_min(mut self, min: f64) -> Self {
        self.request = self.request.query("volume_num_min", min);
        self
    }

    /// Set maximum trading volume
    pub fn volume_num_max(mut self, max: f64) -> Self {
        self.request = self.request.query("volume_num_max", max);
        self
    }

    /// Set earliest market start date (ISO 8601 format)
    pub fn start_date_min(mut self, date: impl Into<String>) -> Self {
        self.request = self.request.query("start_date_min", date.into());
        self
    }

    /// Set latest market start date (ISO 8601 format)
    pub fn start_date_max(mut self, date: impl Into<String>) -> Self {
        self.request = self.request.query("start_date_max", date.into());
        self
    }

    /// Set earliest market end date (ISO 8601 format)
    pub fn end_date_min(mut self, date: impl Into<String>) -> Self {
        self.request = self.request.query("end_date_min", date.into());
        self
    }

    /// Set latest market end date (ISO 8601 format)
    pub fn end_date_max(mut self, date: impl Into<String>) -> Self {
        self.request = self.request.query("end_date_max", date.into());
        self
    }

    /// Filter by tag identifier
    pub fn tag_id(mut self, tag_id: i64) -> Self {
        self.request = self.request.query("tag_id", tag_id);
        self
    }

    /// Include related tags in response
    pub fn related_tags(mut self, include: bool) -> Self {
        self.request = self.request.query("related_tags", include);
        self
    }

    /// Filter for create-your-own markets
    pub fn cyom(mut self, cyom: bool) -> Self {
        self.request = self.request.query("cyom", cyom);
        self
    }

    /// Filter by UMA resolution status
    pub fn uma_resolution_status(mut self, status: impl Into<String>) -> Self {
        self.request = self.request.query("uma_resolution_status", status.into());
        self
    }

    /// Filter by game identifier
    pub fn game_id(mut self, game_id: impl Into<String>) -> Self {
        self.request = self.request.query("game_id", game_id.into());
        self
    }

    /// Filter by sports market types
    pub fn sports_market_types(mut self, types: impl IntoIterator<Item = impl ToString>) -> Self {
        self.request = self.request.query_many("sports_market_types", types);
        self
    }

    /// Set minimum rewards threshold
    pub fn rewards_min_size(mut self, min: f64) -> Self {
        self.request = self.request.query("rewards_min_size", min);
        self
    }

    /// Filter by question identifiers
    pub fn question_ids(mut self, question_ids: impl IntoIterator<Item = impl ToString>) -> Self {
        self.request = self.request.query_many("question_ids", question_ids);
        self
    }

    /// Include tag data in results
    pub fn include_tag(mut self, include: bool) -> Self {
        self.request = self.request.query("include_tag", include);
        self
    }

    /// Filter for closed or active markets
    pub fn closed(mut self, closed: bool) -> Self {
        self.request = self.request.query("closed", closed);
        self
    }

    /// Filter by active status (convenience method, opposite of closed)
    pub fn active(mut self, active: bool) -> Self {
        self.request = self.request.query("closed", !active);
        self
    }

    /// Filter by archived status
    pub fn archived(mut self, archived: bool) -> Self {
        self.request = self.request.query("archived", archived);
        self
    }

    /// Execute the request
    pub async fn send(self) -> Result<Vec<Market>, GammaError> {
        self.request.send().await
    }
}
