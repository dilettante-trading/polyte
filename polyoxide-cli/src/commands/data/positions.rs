use clap::{Args, Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use polyoxide_data::DataApi;

use super::SortOrder;
use crate::commands::common::parsing::{parse_activity_types, parse_comma_separated};
use crate::commands::data::trades::TradeSideFilter;

#[derive(Args)]
pub struct PositionsCommand {
    /// User address (0x-prefixed, 40 hex chars)
    #[arg(short, long)]
    pub user: String,

    #[command(subcommand)]
    pub command: PositionsSubcommand,
}

#[derive(Subcommand)]
pub enum PositionsSubcommand {
    /// List positions for the user
    List {
        /// Filter by market condition IDs (comma-separated)
        #[arg(short, long, value_parser = parse_comma_separated)]
        market: Option<Vec<String>>,
        /// Filter by event IDs (comma-separated)
        #[arg(short, long, value_parser = parse_comma_separated)]
        event_id: Option<Vec<String>>,
        /// Minimum position size filter (default: 1)
        #[arg(long)]
        size_threshold: Option<f64>,
        /// Filter for redeemable positions only
        #[arg(long)]
        redeemable: bool,
        /// Filter for mergeable positions only
        #[arg(long)]
        mergeable: bool,
        /// Maximum number of results (0-500, default: 100)
        #[arg(short, long, default_value = "100")]
        limit: u32,
        /// Pagination offset (0-10000, default: 0)
        #[arg(short, long, default_value = "0")]
        offset: u32,
        /// Sort field
        #[arg(long, value_enum, default_value = "current")]
        sort_by: PositionSortField,
        /// Sort direction
        #[arg(long, value_enum, default_value = "desc")]
        sort_direction: SortOrder,
        /// Filter by market title (max 100 chars)
        #[arg(short, long)]
        title: Option<String>,
    },
    /// Get total value of the user's positions
    Value {
        /// Filter by market condition IDs (comma-separated)
        #[arg(short, long, value_parser = parse_comma_separated)]
        market: Option<Vec<String>>,
    },
    /// List closed positions for the user
    Closed {
        /// Filter by market condition IDs (comma-separated)
        #[arg(short, long, value_parser = parse_comma_separated)]
        market: Option<Vec<String>>,
        /// Filter by event IDs (comma-separated)
        #[arg(short, long, value_parser = parse_comma_separated)]
        event_id: Option<Vec<String>>,
        /// Filter by market title (max 100 chars)
        #[arg(short, long)]
        title: Option<String>,
        /// Maximum number of results (0-50, default: 10)
        #[arg(short, long, default_value = "10")]
        limit: u32,
        /// Pagination offset (0-100000, default: 0)
        #[arg(short, long, default_value = "0")]
        offset: u32,
        /// Sort field
        #[arg(long, value_enum, default_value = "realized-pnl")]
        sort_by: ClosedPositionSortField,
        /// Sort direction
        #[arg(long, value_enum, default_value = "desc")]
        sort_direction: SortOrder,
    },
    /// List activity for the user
    Activity {
        /// Filter by market condition IDs (comma-separated)
        #[arg(short, long, value_parser = parse_comma_separated)]
        market: Option<Vec<String>>,
        /// Filter by event IDs (comma-separated)
        #[arg(short, long, value_parser = parse_comma_separated)]
        event_id: Option<Vec<String>>,
        /// Filter by activity types (comma-separated: trade, split, merge, redeem, reward, conversion)
        #[arg(short = 'T', long)]
        activity_type: Option<String>,
        /// Filter by trade side
        #[arg(short, long, value_enum)]
        side: Option<TradeSideFilter>,
        /// Start timestamp filter
        #[arg(long)]
        start: Option<i64>,
        /// End timestamp filter
        #[arg(long)]
        end: Option<i64>,
        /// Maximum number of results (0-10000, default: 100)
        #[arg(short, long, default_value = "100")]
        limit: u32,
        /// Pagination offset (0-10000, default: 0)
        #[arg(short, long, default_value = "0")]
        offset: u32,
        /// Sort field
        #[arg(long, value_enum, default_value = "timestamp")]
        sort_by: ActivitySortField,
        /// Sort direction
        #[arg(long, value_enum, default_value = "desc")]
        sort_direction: SortOrder,
    },
}

impl PositionsCommand {
    pub async fn run(self, data: &DataApi) -> Result<()> {
        let positions_api = data.positions(&self.user);

        match self.command {
            PositionsSubcommand::List {
                market,
                event_id,
                size_threshold,
                redeemable,
                mergeable,
                limit,
                offset,
                sort_by,
                sort_direction,
                title,
            } => {
                let mut request = positions_api.list_positions();

                if let Some(ref ids) = market {
                    let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
                    request = request.market(ids);
                }
                if let Some(ref ids) = event_id {
                    let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
                    request = request.event_id(ids);
                }
                if let Some(threshold) = size_threshold {
                    request = request.size_threshold(threshold);
                }
                if redeemable {
                    request = request.redeemable(true);
                }
                if mergeable {
                    request = request.mergeable(true);
                }
                request = request
                    .limit(limit)
                    .offset(offset)
                    .sort_by(sort_by.into())
                    .sort_direction(sort_direction.into());
                if let Some(t) = title {
                    request = request.title(t);
                }

                let positions = request.send().await?;
                println!("{}", serde_json::to_string_pretty(&positions)?);
            }
            PositionsSubcommand::Value { market } => {
                let mut request = positions_api.positions_value();
                if let Some(ref ids) = market {
                    let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
                    request = request.market(ids);
                }
                let value = request.send().await?;
                println!("{}", serde_json::to_string_pretty(&value)?);
            }
            PositionsSubcommand::Closed {
                market,
                event_id,
                title,
                limit,
                offset,
                sort_by,
                sort_direction,
            } => {
                let mut request = positions_api
                    .closed_positions()
                    .limit(limit)
                    .offset(offset)
                    .sort_by(sort_by.into())
                    .sort_direction(sort_direction.into());

                if let Some(ref ids) = market {
                    let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
                    request = request.market(ids);
                }
                if let Some(ref ids) = event_id {
                    let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
                    request = request.event_id(ids);
                }
                if let Some(t) = title {
                    request = request.title(t);
                }

                let positions = request.send().await?;
                println!("{}", serde_json::to_string_pretty(&positions)?);
            }
            PositionsSubcommand::Activity {
                market,
                event_id,
                activity_type,
                side,
                start,
                end,
                limit,
                offset,
                sort_by,
                sort_direction,
            } => {
                let mut request = positions_api
                    .activity()
                    .limit(limit)
                    .offset(offset)
                    .sort_by(sort_by.into())
                    .sort_direction(sort_direction.into());

                if let Some(ref ids) = market {
                    let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
                    request = request.market(ids);
                }
                if let Some(ref ids) = event_id {
                    let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
                    request = request.event_id(ids);
                }
                if let Some(types) = activity_type {
                    let activity_types = parse_activity_types(&types)?;
                    if !activity_types.is_empty() {
                        request = request.activity_type(activity_types);
                    }
                }
                if let Some(s) = side {
                    request = request.side(s.into());
                }
                if let Some(ts) = start {
                    request = request.start(ts);
                }
                if let Some(ts) = end {
                    request = request.end(ts);
                }

                let activity = request.send().await?;
                println!("{}", serde_json::to_string_pretty(&activity)?);
            }
        }
        Ok(())
    }
}

/// Sort field for positions
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum PositionSortField {
    /// Sort by current value
    #[default]
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

impl From<PositionSortField> for polyoxide_data::types::PositionSortBy {
    fn from(field: PositionSortField) -> Self {
        match field {
            PositionSortField::Current => Self::Current,
            PositionSortField::Initial => Self::Initial,
            PositionSortField::Tokens => Self::Tokens,
            PositionSortField::CashPnl => Self::CashPnl,
            PositionSortField::PercentPnl => Self::PercentPnl,
            PositionSortField::Title => Self::Title,
            PositionSortField::Resolving => Self::Resolving,
            PositionSortField::Price => Self::Price,
            PositionSortField::AvgPrice => Self::AvgPrice,
        }
    }
}

/// Sort field for closed positions
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum ClosedPositionSortField {
    /// Sort by realized P&L
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

impl From<ClosedPositionSortField> for polyoxide_data::types::ClosedPositionSortBy {
    fn from(field: ClosedPositionSortField) -> Self {
        match field {
            ClosedPositionSortField::RealizedPnl => Self::RealizedPnl,
            ClosedPositionSortField::Title => Self::Title,
            ClosedPositionSortField::Price => Self::Price,
            ClosedPositionSortField::AvgPrice => Self::AvgPrice,
            ClosedPositionSortField::Timestamp => Self::Timestamp,
        }
    }
}

/// Sort field for activity
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum ActivitySortField {
    /// Sort by timestamp
    #[default]
    Timestamp,
    /// Sort by token amount
    Tokens,
    /// Sort by cash amount
    Cash,
}

impl From<ActivitySortField> for polyoxide_data::types::ActivitySortBy {
    fn from(field: ActivitySortField) -> Self {
        match field {
            ActivitySortField::Timestamp => Self::Timestamp,
            ActivitySortField::Tokens => Self::Tokens,
            ActivitySortField::Cash => Self::Cash,
        }
    }
}
