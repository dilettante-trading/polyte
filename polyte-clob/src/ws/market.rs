//! Market channel message types.
//!
//! The market channel provides real-time order book and price updates.

use serde::{Deserialize, Serialize};

/// Order summary in the order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSummary {
    /// Price level
    pub price: String,
    /// Size at this price level
    pub size: String,
}

/// Book message - full order book snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMessage {
    /// Event type (always "book")
    pub event_type: String,
    /// Asset ID (token ID)
    pub asset_id: String,
    /// Market condition ID
    pub market: String,
    /// Timestamp in milliseconds (as string)
    pub timestamp: String,
    /// Order book hash
    pub hash: String,
    /// Buy orders (bids)
    pub bids: Vec<OrderSummary>,
    /// Sell orders (asks)
    pub asks: Vec<OrderSummary>,
    /// Last trade price
    pub last_trade_price: Option<String>,
}

/// Price change entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceChange {
    /// Asset ID (token ID)
    pub asset_id: String,
    /// Price level
    pub price: String,
    /// Size at this price level
    pub size: String,
    /// Order side (BUY or SELL)
    pub side: String,
    /// Order book hash
    pub hash: String,
    /// Best bid price
    pub best_bid: Option<String>,
    /// Best ask price
    pub best_ask: Option<String>,
}

/// Price change message - incremental order book update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceChangeMessage {
    /// Event type (always "price_change")
    pub event_type: String,
    /// Market condition ID
    pub market: String,
    /// List of price changes
    pub price_changes: Vec<PriceChange>,
    /// Timestamp in milliseconds (as string)
    pub timestamp: String,
}

/// Tick size change message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickSizeChangeMessage {
    /// Event type (always "tick_size_change")
    pub event_type: String,
    /// Asset ID (token ID)
    pub asset_id: String,
    /// Market condition ID
    pub market: String,
    /// Old tick size
    pub old_tick_size: String,
    /// New tick size
    pub new_tick_size: String,
    /// Side (BUY or SELL)
    pub side: String,
    /// Timestamp in milliseconds (as string)
    pub timestamp: String,
}

/// Last trade price message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastTradePriceMessage {
    /// Event type (always "last_trade_price")
    pub event_type: String,
    /// Asset ID (token ID)
    pub asset_id: String,
    /// Market condition ID
    pub market: String,
    /// Trade price
    pub price: String,
    /// Trade side (BUY or SELL)
    pub side: String,
    /// Trade size
    pub size: String,
    /// Fee rate
    pub fee_rate_bps: Option<String>,
    /// Timestamp in milliseconds (as string)
    pub timestamp: String,
}

/// Market channel message types
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MarketMessage {
    /// Full order book snapshot
    Book(BookMessage),
    /// Incremental price change
    PriceChange(PriceChangeMessage),
    /// Tick size change
    TickSizeChange(TickSizeChangeMessage),
    /// Last trade price
    LastTradePrice(LastTradePriceMessage),
}

impl MarketMessage {
    /// Parse a market channel message from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        // Book messages come as an array with a single element
        if json.starts_with('[') {
            let books: Vec<BookMessage> = serde_json::from_str(json)?;
            if let Some(book) = books.into_iter().next() {
                return Ok(MarketMessage::Book(book));
            }
            return Err(serde::de::Error::custom("Empty book array"));
        }

        #[derive(Deserialize)]
        struct RawMessage {
            event_type: String,
        }

        let raw: RawMessage = serde_json::from_str(json)?;
        match raw.event_type.as_str() {
            "book" => Ok(MarketMessage::Book(serde_json::from_str(json)?)),
            "price_change" => Ok(MarketMessage::PriceChange(serde_json::from_str(json)?)),
            "tick_size_change" => Ok(MarketMessage::TickSizeChange(serde_json::from_str(json)?)),
            "last_trade_price" => Ok(MarketMessage::LastTradePrice(serde_json::from_str(json)?)),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown market event type: {}",
                raw.event_type
            ))),
        }
    }
}
