use polyte_core::{QueryBuilder, Request};
use reqwest::Client;
use url::Url;

use crate::{
    error::DataApiError,
    types::{Trade, TradeFilterType, TradeSide},
};

/// Trades namespace for trade-related operations
#[derive(Clone)]
pub struct Trades {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
}

impl Trades {
    /// List trades with optional filtering
    pub fn list(&self) -> ListTrades {
        ListTrades {
            request: Request::new(self.client.clone(), self.base_url.clone(), "/trades"),
        }
    }
}

/// Request builder for listing trades
pub struct ListTrades {
    request: Request<Vec<Trade>, DataApiError>,
}

impl ListTrades {
    /// Filter by user address (0x-prefixed, 40 hex chars)
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.request = self.request.query("user", user.into());
        self
    }

    /// Filter by market condition IDs (comma-separated)
    /// Note: Mutually exclusive with `event_id`
    pub fn market(mut self, condition_ids: impl IntoIterator<Item = impl ToString>) -> Self {
        let ids: Vec<String> = condition_ids.into_iter().map(|s| s.to_string()).collect();
        if !ids.is_empty() {
            self.request = self.request.query("market", ids.join(","));
        }
        self
    }

    /// Filter by event IDs (comma-separated)
    /// Note: Mutually exclusive with `market`
    pub fn event_id(mut self, event_ids: impl IntoIterator<Item = impl ToString>) -> Self {
        let ids: Vec<String> = event_ids.into_iter().map(|s| s.to_string()).collect();
        if !ids.is_empty() {
            self.request = self.request.query("eventId", ids.join(","));
        }
        self
    }

    /// Filter by trade side (BUY or SELL)
    pub fn side(mut self, side: TradeSide) -> Self {
        self.request = self.request.query("side", side);
        self
    }

    /// Filter for taker trades only (default: true)
    pub fn taker_only(mut self, taker_only: bool) -> Self {
        self.request = self.request.query("takerOnly", taker_only);
        self
    }

    /// Set filter type (must be paired with `filter_amount`)
    pub fn filter_type(mut self, filter_type: TradeFilterType) -> Self {
        self.request = self.request.query("filterType", filter_type);
        self
    }

    /// Set filter amount (must be paired with `filter_type`)
    pub fn filter_amount(mut self, amount: f64) -> Self {
        self.request = self.request.query("filterAmount", amount);
        self
    }

    /// Set maximum number of results (0-10000, default: 100)
    pub fn limit(mut self, limit: u32) -> Self {
        self.request = self.request.query("limit", limit);
        self
    }

    /// Set pagination offset (0-10000, default: 0)
    pub fn offset(mut self, offset: u32) -> Self {
        self.request = self.request.query("offset", offset);
        self
    }

    /// Execute the request
    pub async fn send(self) -> crate::error::Result<Vec<Trade>> {
        self.request.send().await
    }
}
