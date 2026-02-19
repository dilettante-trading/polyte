//! WebSocket subscription message types.

use serde::{Deserialize, Serialize};

use super::auth::ApiCredentials;

/// WebSocket endpoint URL for market channel
pub const WS_MARKET_URL: &str = "wss://ws-subscriptions-clob.polymarket.com/ws/market";

/// WebSocket endpoint URL for user channel
pub const WS_USER_URL: &str = "wss://ws-subscriptions-clob.polymarket.com/ws/user";

/// Channel type for WebSocket subscription
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChannelType {
    /// Market channel for public order book and price updates
    Market,
    /// User channel for authenticated order and trade updates
    User,
}

/// Subscription message for market channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSubscription {
    /// Asset IDs (token IDs) to subscribe to
    pub assets_ids: Vec<String>,
    /// Channel type (always "market")
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
}

impl MarketSubscription {
    /// Create a new market subscription
    pub fn new(assets_ids: Vec<String>) -> Self {
        Self {
            assets_ids,
            channel_type: ChannelType::Market,
        }
    }
}

/// Subscription message for user channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubscription {
    /// Market condition IDs to subscribe to
    pub markets: Vec<String>,
    /// Authentication credentials
    pub auth: ApiCredentials,
    /// Channel type (always "user")
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
}

impl UserSubscription {
    /// Create a new user subscription
    pub fn new(markets: Vec<String>, credentials: ApiCredentials) -> Self {
        Self {
            markets,
            auth: credentials,
            channel_type: ChannelType::User,
        }
    }
}
