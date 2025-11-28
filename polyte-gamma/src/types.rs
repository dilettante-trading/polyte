use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Market data from Gamma API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Market {
    #[serde(default)]
    pub condition_id: Option<String>,
    #[serde(default)]
    pub question_id: Option<String>,
    #[serde(default)]
    pub tokens: Vec<MarketToken>,
    #[serde(default)]
    pub rewards: Option<HashMap<String, serde_json::Value>>,
    pub minimum_order_size: Option<String>,
    pub minimum_tick_size: Option<String>,
    pub description: String,
    #[serde(default)]
    pub category: Option<String>,
    pub end_date_iso: Option<String>,
    #[serde(default)]
    pub game_start_time: Option<String>,
    pub question: String,
    #[serde(default)]
    pub market_slug: Option<String>,
    pub min_incentive_size: Option<String>,
    pub max_incentive_spread: Option<String>,
    pub active: bool,
    pub closed: bool,
    pub archived: bool,
    pub new: Option<bool>,
    pub featured: Option<bool>,
    pub submitted_by: Option<String>,
    pub volume: Option<String>,
    #[serde(default)]
    pub volume_24hr: Option<f64>,
    pub liquidity: Option<String>,
    #[serde(default)]
    pub competitive: Option<f64>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub group_item_threshold: Option<serde_json::Value>,
    #[serde(default)]
    pub group_item_title: Option<String>,
    #[serde(default)]
    pub neg_risk: bool,
    #[serde(default)]
    pub neg_risk_market_id: Option<String>,
    #[serde(default)]
    pub neg_risk_request_id: Option<String>,
    #[serde(default)]
    pub enable_order_book: bool,
    #[serde(default)]
    pub order_price_min_tick_size: Option<serde_json::Value>,
    #[serde(default)]
    pub order_min_size: Option<serde_json::Value>,
    #[serde(default)]
    pub seconds_delay: Option<i32>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub comment_count: u32,
}

/// Market token (outcome)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MarketToken {
    pub token_id: String,
    pub outcome: String,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub winner: bool,
}

/// Event containing multiple markets
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Event {
    pub id: String,
    pub slug: String,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    pub start_date_iso: Option<String>,
    pub end_date_iso: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    pub active: bool,
    pub closed: bool,
    pub archived: bool,
    pub new: Option<bool>,
    pub featured: bool,
    pub restricted: bool,
    pub liquidity: Option<f64>,
    pub volume: Option<f64>,
    pub volume_24hr: Option<f64>,
    pub comment_count: Option<u32>,
    #[serde(default)]
    pub markets: Vec<Market>,
    #[serde(default)]
    pub cyom: bool,
    #[serde(default)]
    pub competitive: Option<f64>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub series: Vec<SeriesInfo>,
}

/// Series information within an event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SeriesInfo {
    pub id: String,
    pub slug: String,
    pub title: String,
}

/// Series data (tournament/season grouping)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SeriesData {
    pub id: String,
    pub slug: String,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    pub active: bool,
    pub closed: bool,
    pub archived: bool,
    #[serde(default)]
    pub tags: Vec<String>,
    pub volume: Option<f64>,
    pub liquidity: Option<f64>,
    #[serde(default)]
    pub events: Vec<Event>,
    #[serde(default)]
    pub competitive: Option<String>,
}

/// Tag for categorizing markets/events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Tag {
    pub id: String,
    pub slug: String,
    pub label: String,
    pub force_show: Option<bool>,
    pub published_at: Option<String>,
    pub created_by: Option<u64>,
    pub updated_by: Option<u64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub force_hide: Option<bool>,
    pub is_carousel: Option<bool>,
}

/// Sports metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SportMetadata {
    pub id: u64,
    pub sport: String,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub resolution: Option<String>,
    #[serde(default)]
    pub ordering: Option<String>,
    #[serde(default)]
    pub tags: Option<String>,
    #[serde(default)]
    pub series: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Sports team
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Team {
    pub id: i64,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub league: Option<String>,
    #[serde(default)]
    pub record: Option<String>,
    #[serde(default)]
    pub logo: Option<String>,
    #[serde(default)]
    pub abbreviation: Option<String>,
    #[serde(default)]
    pub alias: Option<String>,
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Comment on a market/event/series
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Comment {
    pub id: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub deleted_at: Option<DateTime<Utc>>,
    pub user: CommentUser,
    #[serde(default)]
    pub market_id: Option<String>,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub series_id: Option<String>,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub reactions: Vec<CommentReaction>,
    #[serde(default)]
    pub positions: Vec<CommentPosition>,
    pub like_count: u32,
    pub dislike_count: u32,
    pub reply_count: u32,
}

/// User who created a comment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CommentUser {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub avatar: Option<String>,
}

/// Reaction to a comment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CommentReaction {
    pub user_id: String,
    pub reaction_type: String,
}

/// Position held by comment author
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CommentPosition {
    pub token_id: String,
    pub outcome: String,
    pub shares: String,
}

/// Pagination cursor for list operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Cursor {
    #[serde(default)]
    pub next_cursor: Option<String>,
}

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    #[serde(default)]
    pub next_cursor: Option<String>,
}
