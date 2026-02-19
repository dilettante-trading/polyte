use clap::{Args, ValueEnum};
use color_eyre::eyre::Result;
use polyoxide_data::{types::ActivityType, DataApi};

use super::SortOrder;
use crate::commands::common::parsing::parse_comma_separated;
use crate::commands::data::trades::TradeSideFilter;

#[derive(Args)]
pub struct UserActivityCommand {
    /// User address (0x-prefixed, 40 hex chars)
    #[arg(short, long)]
    pub user: String,
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
}

impl UserActivityCommand {
    pub async fn run(self, data: &DataApi) -> Result<()> {
        let positions_api = data.positions(&self.user);

        let mut request = positions_api
            .activity()
            .limit(self.limit)
            .offset(self.offset)
            .sort_by(self.sort_by.into())
            .sort_direction(self.sort_direction.into());

        if let Some(ref ids) = self.market {
            let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
            request = request.market(ids);
        }
        if let Some(ref ids) = self.event_id {
            let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
            request = request.event_id(ids);
        }
        if let Some(types) = self.activity_type {
            let activity_types: Vec<ActivityType> = types
                .split(',')
                .filter_map(|s| match s.trim().to_uppercase().as_str() {
                    "TRADE" => Some(ActivityType::Trade),
                    "SPLIT" => Some(ActivityType::Split),
                    "MERGE" => Some(ActivityType::Merge),
                    "REDEEM" => Some(ActivityType::Redeem),
                    "REWARD" => Some(ActivityType::Reward),
                    "CONVERSION" => Some(ActivityType::Conversion),
                    _ => None,
                })
                .collect();
            if !activity_types.is_empty() {
                request = request.activity_type(activity_types);
            }
        }
        if let Some(s) = self.side {
            request = request.side(s.into());
        }
        if let Some(ts) = self.start {
            request = request.start(ts);
        }
        if let Some(ts) = self.end {
            request = request.end(ts);
        }

        let activity = request.send().await?;
        println!("{}", serde_json::to_string_pretty(&activity)?);
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
