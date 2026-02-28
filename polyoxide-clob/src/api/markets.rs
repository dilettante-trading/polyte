use polyoxide_core::{HttpClient, QueryBuilder};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    request::{AuthMode, Request},
    types::OrderSide,
};

/// Markets namespace for market-related operations
#[derive(Clone)]
pub struct Markets {
    pub(crate) http_client: HttpClient,
    pub(crate) chain_id: u64,
}

impl Markets {
    /// Get a market by condition ID
    pub fn get(&self, condition_id: impl Into<String>) -> Request<Market> {
        Request::get(
            self.http_client.clone(),
            format!("/markets/{}", urlencoding::encode(&condition_id.into())),
            AuthMode::None,
            self.chain_id,
        )
    }

    pub fn get_by_token_ids(
        &self,
        token_ids: impl Into<Vec<String>>,
    ) -> Request<ListMarketsResponse> {
        Request::get(
            self.http_client.clone(),
            "/markets",
            AuthMode::None,
            self.chain_id,
        )
        .query_many("clob_token_ids", token_ids.into())
    }

    /// List all markets
    pub fn list(&self) -> Request<ListMarketsResponse> {
        Request::get(
            self.http_client.clone(),
            "/markets",
            AuthMode::None,
            self.chain_id,
        )
    }

    /// Get order book for a token
    pub fn order_book(&self, token_id: impl Into<String>) -> Request<OrderBook> {
        Request::get(
            self.http_client.clone(),
            "/book",
            AuthMode::None,
            self.chain_id,
        )
        .query("token_id", token_id.into())
    }

    /// Get price for a token and side
    pub fn price(&self, token_id: impl Into<String>, side: OrderSide) -> Request<PriceResponse> {
        Request::get(
            self.http_client.clone(),
            "/price",
            AuthMode::None,
            self.chain_id,
        )
        .query("token_id", token_id.into())
        .query("side", side.as_str())
    }

    /// Get midpoint price for a token
    pub fn midpoint(&self, token_id: impl Into<String>) -> Request<MidpointResponse> {
        Request::get(
            self.http_client.clone(),
            "/midpoint",
            AuthMode::None,
            self.chain_id,
        )
        .query("token_id", token_id.into())
    }

    /// Get historical prices for a token
    pub fn prices_history(&self, token_id: impl Into<String>) -> Request<PricesHistoryResponse> {
        Request::get(
            self.http_client.clone(),
            "/prices-history",
            AuthMode::None,
            self.chain_id,
        )
        .query("market", token_id.into())
    }

    /// Get neg_risk status for a token
    pub fn neg_risk(&self, token_id: impl Into<String>) -> Request<NegRiskResponse> {
        Request::get(
            self.http_client.clone(),
            "/neg-risk".to_string(),
            AuthMode::None,
            self.chain_id,
        )
        .query("token_id", token_id.into())
    }

    /// Get the current fee rate for a token
    pub fn fee_rate(&self, token_id: impl Into<String>) -> Request<FeeRateResponse> {
        Request::get(
            self.http_client.clone(),
            "/fee-rate",
            AuthMode::None,
            self.chain_id,
        )
        .query("token_id", token_id.into())
    }

    /// Get tick size for a token
    pub fn tick_size(&self, token_id: impl Into<String>) -> Request<TickSizeResponse> {
        Request::get(
            self.http_client.clone(),
            "/tick-size".to_string(),
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
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub size: Decimal,
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

/// A single point in the price history timeseries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceHistoryPoint {
    /// Unix timestamp (seconds)
    #[serde(rename = "t")]
    pub timestamp: i64,
    /// Price at this point in time
    #[serde(rename = "p")]
    pub price: f64,
}

/// Response from the prices-history endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricesHistoryResponse {
    pub history: Vec<PriceHistoryPoint>,
}

/// Response from the neg-risk endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegRiskResponse {
    pub neg_risk: bool,
}

/// Response from the fee-rate endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeRateResponse {
    pub base_fee: u32,
}

/// Response from the tick-size endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickSizeResponse {
    #[serde(deserialize_with = "deserialize_tick_size")]
    pub minimum_tick_size: String,
}

fn deserialize_tick_size<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    let v = serde_json::Value::deserialize(deserializer)?;
    match v {
        serde_json::Value::String(s) => Ok(s),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        _ => Err(serde::de::Error::custom(
            "expected string or number for tick size",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fee_rate_response_deserializes() {
        let json = r#"{"base_fee": 100}"#;
        let resp: FeeRateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.base_fee, 100);
    }

    #[test]
    fn test_fee_rate_response_deserializes_zero() {
        let json = r#"{"base_fee": 0}"#;
        let resp: FeeRateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.base_fee, 0);
    }

    #[test]
    fn test_fee_rate_response_rejects_missing_field() {
        let json = r#"{"feeRate": "100"}"#;
        let result = serde_json::from_str::<FeeRateResponse>(json);
        assert!(result.is_err(), "Should reject JSON missing base_fee field");
    }

    #[test]
    fn test_fee_rate_response_rejects_empty_json() {
        let json = r#"{}"#;
        let result = serde_json::from_str::<FeeRateResponse>(json);
        assert!(result.is_err(), "Should reject empty JSON object");
    }
}
