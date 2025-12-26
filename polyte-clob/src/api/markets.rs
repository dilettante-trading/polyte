use polyte_core::QueryBuilder;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    request::{AuthMode, Request},
    types::OrderSide,
};

/// Markets namespace for market-related operations
#[derive(Clone)]
pub struct Markets {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
    pub(crate) chain_id: u64,
}

impl Markets {
    /// Get a market by condition ID
    pub fn get(&self, condition_id: impl Into<String>) -> Request<Market> {
        Request::get(
            self.client.clone(),
            self.base_url.clone(),
            format!("/markets/{}", urlencoding::encode(&condition_id.into())),
            AuthMode::None,
            self.chain_id,
        )
    }

    /// List all markets
    pub fn list(&self) -> Request<ListMarketsResponse> {
        Request::get(
            self.client.clone(),
            self.base_url.clone(),
            "/markets",
            AuthMode::None,
            self.chain_id,
        )
    }

    /// Get order book for a token
    pub fn order_book(&self, token_id: impl Into<String>) -> Request<OrderBook> {
        Request::get(
            self.client.clone(),
            self.base_url.clone(),
            "/book",
            AuthMode::None,
            self.chain_id,
        )
        .query("token_id", token_id.into())
    }

    /// Get price for a token and side
    pub fn price(&self, token_id: impl Into<String>, side: OrderSide) -> Request<PriceResponse> {
        Request::get(
            self.client.clone(),
            self.base_url.clone(),
            "/price",
            AuthMode::None,
            self.chain_id,
        )
        .query("token_id", token_id.into())
        .query("side", side.to_string())
    }

    /// Get midpoint price for a token
    pub fn midpoint(&self, token_id: impl Into<String>) -> Request<MidpointResponse> {
        Request::get(
            self.client.clone(),
            self.base_url.clone(),
            "/midpoint",
            AuthMode::None,
            self.chain_id,
        )
        .query("token_id", token_id.into())
    }
}

/// Market information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub condition_id: String,
    pub question_id: String,
    pub tokens: Vec<MarketToken>,
    pub rewards: Option<serde_json::Value>,
    pub minimum_order_size: f64,
    pub minimum_tick_size: f64,
    pub description: String,
    pub category: Option<String>,
    pub end_date_iso: Option<String>,
    pub question: String,
    pub active: bool,
    pub closed: bool,
    pub archived: bool,
    pub neg_risk: Option<bool>,
    pub neg_risk_market_id: Option<String>,
    pub enable_order_book: Option<bool>,
}

/// Markets list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMarketsResponse {
    pub data: Vec<Market>,
    pub next_cursor: Option<String>,
}

/// Market token (outcome)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketToken {
    pub token_id: Option<String>,
    pub outcome: String,
    pub price: Option<f64>,
    pub winner: Option<bool>,
}

/// Order book level (price and size)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLevel {
    pub price: String,
    pub size: String,
}

/// Order book data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub market: String,
    pub asset_id: String,
    pub bids: Vec<OrderLevel>,
    pub asks: Vec<OrderLevel>,
    pub timestamp: String,
    pub hash: String,
}

/// Price response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceResponse {
    pub price: String,
}

/// Midpoint price response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidpointResponse {
    pub mid: String,
}
