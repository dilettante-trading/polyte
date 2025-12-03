//! User channel message types.
//!
//! The user channel provides real-time order and trade updates for authenticated users.

use serde::{Deserialize, Serialize};

/// Maker order in a trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakerOrder {
    /// Order ID
    pub order_id: String,
    /// Maker address
    pub maker_address: String,
    /// Matched amount
    pub matched_amount: String,
    /// Fee rate
    pub fee_rate_bps: Option<String>,
    /// Asset ID
    pub asset_id: String,
    /// Price
    pub price: String,
}

/// Trade status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TradeStatus {
    /// Trade matched
    Matched,
    /// Trade mined on chain
    Mined,
    /// Trade confirmed
    Confirmed,
    /// Trade retrying
    Retrying,
    /// Trade failed
    Failed,
}

/// Trade message - user trade update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeMessage {
    /// Event type (always "trade")
    pub event_type: String,
    /// Trade ID
    pub id: String,
    /// Asset ID (token ID)
    pub asset_id: String,
    /// Market condition ID
    pub market: String,
    /// Outcome (YES or NO)
    pub outcome: String,
    /// Trade price
    pub price: String,
    /// Trade size
    pub size: String,
    /// Trade side (BUY or SELL)
    pub side: String,
    /// Trade status
    pub status: TradeStatus,
    /// Taker order ID
    pub taker_order_id: String,
    /// Maker orders involved in the trade
    pub maker_orders: Vec<MakerOrder>,
    /// Trade owner address
    pub owner: Option<String>,
    /// Transaction hash (when mined/confirmed)
    pub transaction_hash: Option<String>,
    /// Timestamp
    pub timestamp: String,
}

/// Order event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderEventType {
    /// New order placement
    Placement,
    /// Order updated (partial fill)
    Update,
    /// Order cancelled
    Cancellation,
}

/// Order message - user order update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMessage {
    /// Event type (always "order")
    pub event_type: String,
    /// Order ID
    pub id: String,
    /// Asset ID (token ID)
    pub asset_id: String,
    /// Market condition ID
    pub market: String,
    /// Outcome (YES or NO)
    pub outcome: String,
    /// Order price
    pub price: String,
    /// Order side (BUY or SELL)
    pub side: String,
    /// Original order size
    pub original_size: String,
    /// Size matched so far
    pub size_matched: String,
    /// Order event type
    #[serde(rename = "type")]
    pub order_type: OrderEventType,
    /// Order owner address
    pub order_owner: Option<String>,
    /// Timestamp
    pub timestamp: String,
}

/// User channel message types
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum UserMessage {
    /// Trade update
    Trade(TradeMessage),
    /// Order update
    Order(OrderMessage),
}

impl UserMessage {
    /// Parse a user channel message from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        #[derive(Deserialize)]
        struct RawMessage {
            event_type: String,
        }

        let raw: RawMessage = serde_json::from_str(json)?;
        match raw.event_type.as_str() {
            "trade" => Ok(UserMessage::Trade(serde_json::from_str(json)?)),
            "order" => Ok(UserMessage::Order(serde_json::from_str(json)?)),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown user event type: {}",
                raw.event_type
            ))),
        }
    }
}
