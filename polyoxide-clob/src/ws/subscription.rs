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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_type_serialization() {
        let market = serde_json::to_value(ChannelType::Market).unwrap();
        let user = serde_json::to_value(ChannelType::User).unwrap();

        assert_eq!(market, "market");
        assert_eq!(user, "user");
    }

    #[test]
    fn channel_type_deserialization() {
        let market: ChannelType = serde_json::from_str("\"market\"").unwrap();
        let user: ChannelType = serde_json::from_str("\"user\"").unwrap();

        assert_eq!(market, ChannelType::Market);
        assert_eq!(user, ChannelType::User);
    }

    #[test]
    fn channel_type_rejects_uppercase() {
        let result = serde_json::from_str::<ChannelType>("\"MARKET\"");
        assert!(result.is_err(), "Should reject uppercase channel type");
    }

    #[test]
    fn market_subscription_new_sets_channel_type() {
        let sub = MarketSubscription::new(vec!["asset1".into(), "asset2".into()]);
        assert_eq!(sub.channel_type, ChannelType::Market);
        assert_eq!(sub.assets_ids.len(), 2);
        assert_eq!(sub.assets_ids[0], "asset1");
        assert_eq!(sub.assets_ids[1], "asset2");
    }

    #[test]
    fn market_subscription_serialization() {
        let sub = MarketSubscription::new(vec!["token123".into()]);
        let json = serde_json::to_value(&sub).unwrap();

        assert_eq!(json["type"], "market");
        assert_eq!(json["assets_ids"][0], "token123");
    }

    #[test]
    fn market_subscription_empty_assets() {
        let sub = MarketSubscription::new(vec![]);
        let json = serde_json::to_value(&sub).unwrap();

        assert_eq!(json["type"], "market");
        assert!(json["assets_ids"].as_array().unwrap().is_empty());
    }

    #[test]
    fn user_subscription_new_sets_channel_type() {
        let creds = ApiCredentials::new("key", "secret", "pass");
        let sub = UserSubscription::new(vec!["cond1".into()], creds);
        assert_eq!(sub.channel_type, ChannelType::User);
        assert_eq!(sub.markets.len(), 1);
        assert_eq!(sub.markets[0], "cond1");
    }

    #[test]
    fn user_subscription_serialization() {
        let creds = ApiCredentials::new("my_key", "my_secret", "my_pass");
        let sub = UserSubscription::new(vec!["market1".into(), "market2".into()], creds);
        let json = serde_json::to_value(&sub).unwrap();

        assert_eq!(json["type"], "user");
        assert_eq!(json["markets"][0], "market1");
        assert_eq!(json["markets"][1], "market2");
        assert_eq!(json["auth"]["apiKey"], "my_key");
        assert_eq!(json["auth"]["secret"], "my_secret");
        assert_eq!(json["auth"]["passphrase"], "my_pass");
    }

    #[test]
    fn ws_url_constants() {
        assert!(WS_MARKET_URL.starts_with("wss://"));
        assert!(WS_MARKET_URL.contains("market"));
        assert!(WS_USER_URL.starts_with("wss://"));
        assert!(WS_USER_URL.contains("user"));
    }
}
