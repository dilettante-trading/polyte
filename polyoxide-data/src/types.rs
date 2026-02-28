use serde::{Deserialize, Serialize};

/// User's total position value
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserValue {
    /// User address
    pub user: String,
    /// Total value of positions
    pub value: f64,
}

/// Open interest for a market
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenInterest {
    /// Market condition ID
    pub market: String,
    /// Open interest value
    pub value: f64,
}

/// Sort field options for position queries
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSortBy {
    /// Sort by current value
    Current,
    /// Sort by initial value
    Initial,
    /// Sort by token count
    Tokens,
    /// Sort by cash P&L
    CashPnl,
    /// Sort by percentage P&L
    PercentPnl,
    /// Sort by market title
    Title,
    /// Sort by resolving status
    Resolving,
    /// Sort by price
    Price,
    /// Sort by average price
    AvgPrice,
}

impl std::fmt::Display for PositionSortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Current => write!(f, "CURRENT"),
            Self::Initial => write!(f, "INITIAL"),
            Self::Tokens => write!(f, "TOKENS"),
            Self::CashPnl => write!(f, "CASH_PNL"),
            Self::PercentPnl => write!(f, "PERCENT_PNL"),
            Self::Title => write!(f, "TITLE"),
            Self::Resolving => write!(f, "RESOLVING"),
            Self::Price => write!(f, "PRICE"),
            Self::AvgPrice => write!(f, "AVG_PRICE"),
        }
    }
}

/// Sort direction for queries
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum SortDirection {
    /// Ascending order
    Asc,
    /// Descending order (default)
    #[default]
    Desc,
}

impl std::fmt::Display for SortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asc => write!(f, "ASC"),
            Self::Desc => write!(f, "DESC"),
        }
    }
}

/// Sort field options for closed position queries
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClosedPositionSortBy {
    /// Sort by realized P&L (default)
    #[default]
    RealizedPnl,
    /// Sort by market title
    Title,
    /// Sort by price
    Price,
    /// Sort by average price
    AvgPrice,
    /// Sort by timestamp
    Timestamp,
}

impl std::fmt::Display for ClosedPositionSortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RealizedPnl => write!(f, "REALIZED_PNL"),
            Self::Title => write!(f, "TITLE"),
            Self::Price => write!(f, "PRICE"),
            Self::AvgPrice => write!(f, "AVG_PRICE"),
            Self::Timestamp => write!(f, "TIMESTAMP"),
        }
    }
}

/// Closed position record
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPosition {
    /// Proxy wallet address
    pub proxy_wallet: String,
    /// Asset identifier (token ID)
    pub asset: String,
    /// Condition ID of the market
    pub condition_id: String,
    /// Average entry price
    pub avg_price: f64,
    /// Total amount bought
    pub total_bought: f64,
    /// Realized profit and loss
    pub realized_pnl: f64,
    /// Current market price
    pub cur_price: f64,
    /// Timestamp when position was closed
    #[cfg_attr(feature = "specta", specta(type = f64))]
    pub timestamp: i64,
    /// Market title
    pub title: String,
    /// Market slug
    pub slug: String,
    /// Market icon URL
    pub icon: Option<String>,
    /// Event slug
    pub event_slug: Option<String>,
    /// Outcome name (e.g., "Yes", "No")
    pub outcome: String,
    /// Outcome index (0 or 1 for binary markets)
    pub outcome_index: u32,
    /// Opposite outcome name
    pub opposite_outcome: String,
    /// Opposite outcome asset ID
    pub opposite_asset: String,
    /// Market end date
    pub end_date: Option<String>,
}

/// Trade side (buy or sell)
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TradeSide {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}

impl std::fmt::Display for TradeSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Buy => write!(f, "BUY"),
            Self::Sell => write!(f, "SELL"),
        }
    }
}

/// Filter type for trade queries
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TradeFilterType {
    /// Filter by cash amount
    Cash,
    /// Filter by token amount
    Tokens,
}

impl std::fmt::Display for TradeFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cash => write!(f, "CASH"),
            Self::Tokens => write!(f, "TOKENS"),
        }
    }
}

/// Trade record
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    /// Proxy wallet address
    pub proxy_wallet: String,
    /// Trade side (BUY or SELL)
    pub side: TradeSide,
    /// Asset identifier (token ID)
    pub asset: String,
    /// Condition ID of the market
    pub condition_id: String,
    /// Trade size (number of shares)
    pub size: f64,
    /// Trade price
    pub price: f64,
    /// Trade timestamp
    #[cfg_attr(feature = "specta", specta(type = f64))]
    pub timestamp: i64,
    /// Market title
    pub title: String,
    /// Market slug
    pub slug: String,
    /// Market icon URL
    pub icon: Option<String>,
    /// Event slug
    pub event_slug: Option<String>,
    /// Outcome name (e.g., "Yes", "No")
    pub outcome: String,
    /// Outcome index (0 or 1 for binary markets)
    pub outcome_index: u32,
    /// User display name
    pub name: Option<String>,
    /// User pseudonym
    pub pseudonym: Option<String>,
    /// User bio
    pub bio: Option<String>,
    /// User profile image URL
    pub profile_image: Option<String>,
    /// Optimized profile image URL
    pub profile_image_optimized: Option<String>,
    /// Transaction hash
    pub transaction_hash: Option<String>,
}

/// Activity type
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ActivityType {
    /// Trade activity
    Trade,
    /// Split activity
    Split,
    /// Merge activity
    Merge,
    /// Redeem activity
    Redeem,
    /// Reward activity
    Reward,
    /// Conversion activity
    Conversion,
}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trade => write!(f, "TRADE"),
            Self::Split => write!(f, "SPLIT"),
            Self::Merge => write!(f, "MERGE"),
            Self::Redeem => write!(f, "REDEEM"),
            Self::Reward => write!(f, "REWARD"),
            Self::Conversion => write!(f, "CONVERSION"),
        }
    }
}

/// Sort field options for activity queries
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivitySortBy {
    /// Sort by timestamp (default)
    #[default]
    Timestamp,
    /// Sort by token amount
    Tokens,
    /// Sort by cash amount
    Cash,
}

impl std::fmt::Display for ActivitySortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timestamp => write!(f, "TIMESTAMP"),
            Self::Tokens => write!(f, "TOKENS"),
            Self::Cash => write!(f, "CASH"),
        }
    }
}

/// User activity record
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    /// Proxy wallet address
    pub proxy_wallet: String,
    /// Activity timestamp
    #[cfg_attr(feature = "specta", specta(type = f64))]
    pub timestamp: i64,
    /// Condition ID of the market
    pub condition_id: String,
    /// Activity type
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    /// Token quantity
    pub size: f64,
    /// USD value
    pub usdc_size: f64,
    /// On-chain transaction hash
    pub transaction_hash: Option<String>,
    /// Execution price
    pub price: Option<f64>,
    /// Asset identifier (token ID)
    pub asset: Option<String>,
    // ! Deserialize into String because the API can return an empty string
    /// Trade side (BUY or SELL)
    pub side: Option<String>,
    /// Outcome index (0 or 1 for binary markets)
    pub outcome_index: Option<u32>,
    /// Market title
    pub title: Option<String>,
    /// Market slug
    pub slug: Option<String>,
    /// Market icon URL
    pub icon: Option<String>,
    /// Outcome name (e.g., "Yes", "No")
    pub outcome: Option<String>,
    /// User display name
    pub name: Option<String>,
    /// User pseudonym
    pub pseudonym: Option<String>,
    /// User bio
    pub bio: Option<String>,
    /// User profile image URL
    pub profile_image: Option<String>,
    /// Optimized profile image URL
    pub profile_image_optimized: Option<String>,
}

/// User position in a market
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// Proxy wallet address
    pub proxy_wallet: String,
    /// Asset identifier (token ID)
    pub asset: String,
    /// Condition ID of the market
    pub condition_id: String,
    /// Position size (number of shares)
    pub size: f64,
    /// Average entry price
    pub avg_price: f64,
    /// Initial value of position
    pub initial_value: f64,
    /// Current value of position
    pub current_value: f64,
    /// Cash profit and loss
    pub cash_pnl: f64,
    /// Percentage profit and loss
    pub percent_pnl: f64,
    /// Total amount bought
    pub total_bought: f64,
    /// Realized profit and loss
    pub realized_pnl: f64,
    /// Percentage realized P&L
    pub percent_realized_pnl: f64,
    /// Current market price
    pub cur_price: f64,
    /// Whether position is redeemable
    pub redeemable: bool,
    /// Whether position is mergeable
    pub mergeable: bool,
    /// Market title
    pub title: String,
    /// Market slug
    pub slug: String,
    /// Market icon URL
    pub icon: Option<String>,
    /// Event slug
    pub event_slug: Option<String>,
    /// Outcome name (e.g., "Yes", "No")
    pub outcome: String,
    /// Outcome index (0 or 1 for binary markets)
    pub outcome_index: u32,
    /// Opposite outcome name
    pub opposite_outcome: String,
    /// Opposite outcome asset ID
    pub opposite_asset: String,
    /// Market end date
    pub end_date: Option<String>,
    /// Whether this is a negative risk market
    pub negative_risk: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify Display matches serde serialization for all PositionSortBy variants.
    #[test]
    fn position_sort_by_display_matches_serde() {
        let variants = [
            PositionSortBy::Current,
            PositionSortBy::Initial,
            PositionSortBy::Tokens,
            PositionSortBy::CashPnl,
            PositionSortBy::PercentPnl,
            PositionSortBy::Title,
            PositionSortBy::Resolving,
            PositionSortBy::Price,
            PositionSortBy::AvgPrice,
        ];
        for variant in variants {
            let serialized = serde_json::to_value(variant).unwrap();
            let display = variant.to_string();
            assert_eq!(
                format!("\"{}\"", display),
                serialized.to_string(),
                "Display mismatch for {:?}",
                variant
            );
        }
    }

    /// Verify Display matches serde serialization for all ClosedPositionSortBy variants.
    #[test]
    fn closed_position_sort_by_display_matches_serde() {
        let variants = [
            ClosedPositionSortBy::RealizedPnl,
            ClosedPositionSortBy::Title,
            ClosedPositionSortBy::Price,
            ClosedPositionSortBy::AvgPrice,
            ClosedPositionSortBy::Timestamp,
        ];
        for variant in variants {
            let serialized = serde_json::to_value(variant).unwrap();
            let display = variant.to_string();
            assert_eq!(
                format!("\"{}\"", display),
                serialized.to_string(),
                "Display mismatch for {:?}",
                variant
            );
        }
    }

    #[test]
    fn activity_sort_by_display_matches_serde() {
        let variants = [
            ActivitySortBy::Timestamp,
            ActivitySortBy::Tokens,
            ActivitySortBy::Cash,
        ];
        for variant in variants {
            let serialized = serde_json::to_value(variant).unwrap();
            let display = variant.to_string();
            assert_eq!(
                format!("\"{}\"", display),
                serialized.to_string(),
                "Display mismatch for {:?}",
                variant
            );
        }
    }

    #[test]
    fn sort_direction_display_matches_serde() {
        let variants = [SortDirection::Asc, SortDirection::Desc];
        for variant in variants {
            let serialized = serde_json::to_value(variant).unwrap();
            let display = variant.to_string();
            assert_eq!(
                format!("\"{}\"", display),
                serialized.to_string(),
                "Display mismatch for {:?}",
                variant
            );
        }
    }
}
