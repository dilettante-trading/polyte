use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Market data from Gamma API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Market {
    pub id: String,
    pub condition_id: String,
    pub question_id: Option<String>,
    pub slug: Option<String>,
    #[serde(default)]
    pub tokens: Vec<MarketToken>,
    pub rewards: Option<HashMap<String, serde_json::Value>>,
    pub minimum_order_size: Option<String>,
    pub minimum_tick_size: Option<String>,
    pub description: String,
    pub category: Option<String>,
    pub end_date_iso: Option<String>,
    pub start_date_iso: Option<String>,
    pub question: String,
    pub min_incentive_size: Option<String>,
    pub max_incentive_spread: Option<String>,
    pub submitted_by: Option<String>,
    pub volume_24hr: Option<f64>,
    pub volume_1wk: Option<f64>,
    pub volume_1mo: Option<f64>,
    pub volume_1yr: Option<f64>,
    pub liquidity: Option<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    pub neg_risk: Option<bool>,
    pub neg_risk_market_id: Option<String>,
    pub neg_risk_request_id: Option<String>,
    pub comment_count: Option<u32>,
    pub twitter_card_image: Option<String>,
    pub resolution_source: Option<String>,
    pub amm_type: Option<String>,
    pub sponsor_name: Option<String>,
    pub sponsor_image: Option<String>,
    pub x_axis_value: Option<String>,
    pub y_axis_value: Option<String>,
    pub denomation_token: Option<String>,
    pub fee: Option<String>,
    pub image: Option<String>,
    pub icon: Option<String>,
    pub lower_bound: Option<String>,
    pub upper_bound: Option<String>,
    pub outcomes: Option<String>,
    pub outcome_prices: Option<String>,
    pub volume: Option<String>,
    pub active: Option<bool>,
    pub market_type: Option<String>,
    pub format_type: Option<String>,
    pub lower_bound_date: Option<String>,
    pub upper_bound_date: Option<String>,
    pub closed: Option<bool>,
    pub market_maker_address: String,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub closed_time: Option<String>,
    pub wide_format: Option<bool>,
    pub new: Option<bool>,
    pub mailchimp_tag: Option<String>,
    pub featured: Option<bool>,
    pub archived: Option<bool>,
    pub resolved_by: Option<String>,
    pub restricted: Option<bool>,
    pub market_group: Option<i64>,
    pub group_item_title: Option<String>,
    pub group_item_threshold: Option<String>,
    pub uma_end_date: Option<String>,
    pub uma_resolution_status: Option<String>,
    pub uma_end_date_iso: Option<String>,
    pub uma_resolution_statuses: Option<String>,
    pub enable_order_book: Option<bool>,
    pub order_price_min_tick_size: Option<f64>,
    pub order_min_size: Option<f64>,
    pub curation_order: Option<i64>,
    pub volume_num: Option<f64>,
    pub liquidity_num: Option<f64>,
    pub has_review_dates: Option<bool>,
    pub ready_for_cron: Option<bool>,
    pub comments_enabled: Option<bool>,
    pub game_start_time: Option<String>,
    pub seconds_delay: Option<i64>,
    pub clob_token_ids: Option<String>,
    pub disqus_thread: Option<String>,
    pub short_outcomes: Option<String>,
    pub team_aid: Option<String>,
    pub team_bid: Option<String>,
    pub uma_bond: Option<String>,
    pub uma_reward: Option<String>,
    pub fpmm_live: Option<bool>,
    pub volume_24hr_amm: Option<f64>,
    pub volume_1wk_amm: Option<f64>,
    pub volume_1mo_amm: Option<f64>,
    pub volume_1yr_amm: Option<f64>,
    pub volume_24hr_clob: Option<f64>,
    pub volume_1wk_clob: Option<f64>,
    pub volume_1mo_clob: Option<f64>,
    pub volume_1yr_clob: Option<f64>,
    pub volume_amm: Option<f64>,
    pub volume_clob: Option<f64>,
    pub liquidity_amm: Option<f64>,
    pub liquidity_clob: Option<f64>,
    pub maker_base_fee: Option<i64>,
    pub taker_base_fee: Option<i64>,
    pub custom_liveness: Option<i64>,
    pub accepting_orders: Option<bool>,
    pub notifications_enabled: Option<bool>,
    pub score: Option<i64>,
    pub creator: Option<String>,
    pub ready: Option<bool>,
    pub funded: Option<bool>,
    pub past_slugs: Option<String>,
    pub ready_timestamp: Option<String>,
    pub funded_timestamp: Option<String>,
    pub accepting_orders_timestamp: Option<String>,
    pub competitive: Option<f64>,
    pub rewards_min_size: Option<f64>,
    pub rewards_max_spreads: Option<f64>,
    pub spread: Option<f64>,
    pub automatically_resolved: Option<bool>,
    pub automatically_active: Option<bool>,
    pub one_day_price_change: Option<f64>,
    pub one_hour_price_change: Option<f64>,
    pub one_week_price_change: Option<f64>,
    pub one_month_price_change: Option<f64>,
    pub one_year_price_change: Option<f64>,
    pub last_trade_price: Option<f64>,
    pub best_bid: Option<f64>,
    pub best_ask: Option<f64>,
    pub clear_book_on_start: Option<bool>,
    pub chart_color: Option<String>,
    pub series_color: Option<String>,
    pub show_gmp_series: Option<bool>,
    pub show_gmp_outcome: Option<bool>,
    pub manual_activation: Option<bool>,
    pub neg_risk_other: Option<bool>,
    pub game_id: Option<String>,
    pub group_item_range: Option<String>,
    pub sports_market_type: Option<String>,
    pub line: Option<f64>,
    pub pending_deployment: Option<bool>,
    pub deploying: Option<bool>,
    pub deploying_timestamp: Option<String>,
    pub schedule_deployment_timestamp: Option<String>,
    pub rfq_enabled: Option<bool>,
    pub event_start_time: Option<String>,
}

/// Market token (outcome)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MarketToken {
    pub token_id: String,
    pub outcome: String,
    pub price: Option<String>,
    pub winner: Option<bool>,
}

/// Event containing multiple markets
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Event {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub start_date_iso: Option<String>,
    pub end_date_iso: Option<String>,
    pub image: Option<String>,
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
    pub cyom: Option<bool>,
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
    pub description: Option<String>,
    pub image: Option<String>,
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
    pub image: Option<String>,
    pub resolution: Option<String>,
    pub ordering: Option<String>,
    pub tags: Option<String>,
    pub series: Option<String>,
    pub created_at: Option<String>,
}

/// Sports team
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Team {
    pub id: i64,
    pub name: Option<String>,
    pub league: Option<String>,
    pub record: Option<String>,
    pub logo: Option<String>,
    pub abbreviation: Option<String>,
    pub alias: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
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
    pub deleted_at: Option<DateTime<Utc>>,
    pub user: CommentUser,
    pub market_id: Option<String>,
    pub event_id: Option<String>,
    pub series_id: Option<String>,
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
    pub next_cursor: Option<String>,
}

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub next_cursor: Option<String>,
}
