use clap::{Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use polyte_data::DataApi;

use crate::commands::common::parsing::parse_comma_separated;

#[derive(Subcommand)]
pub enum TradesCommand {
    /// List trades for a user or markets
    List {
        /// User address (0x-prefixed, 40 hex chars)
        #[arg(short, long)]
        user: Option<String>,
        /// Filter by market condition IDs (comma-separated)
        #[arg(short, long, value_parser = parse_comma_separated)]
        market: Option<Vec<String>>,
        /// Filter by event IDs (comma-separated)
        #[arg(short, long, value_parser = parse_comma_separated)]
        event_id: Option<Vec<String>>,
        /// Filter by trade side
        #[arg(short, long, value_enum)]
        side: Option<TradeSideFilter>,
        /// Filter for taker trades only (default: true)
        #[arg(long, default_value = "true")]
        taker_only: bool,
        /// Filter type (must be paired with --filter-amount)
        #[arg(long, value_enum)]
        filter_type: Option<TradeFilterField>,
        /// Filter amount (must be paired with --filter-type)
        #[arg(long)]
        filter_amount: Option<f64>,
        /// Maximum number of results (0-10000, default: 100)
        #[arg(short, long, default_value = "100")]
        limit: u32,
        /// Pagination offset (0-10000, default: 0)
        #[arg(short, long, default_value = "0")]
        offset: u32,
    },
}

impl TradesCommand {
    pub async fn run(self, data: &DataApi) -> Result<()> {
        match self {
            Self::List {
                user,
                market,
                event_id,
                side,
                taker_only,
                filter_type,
                filter_amount,
                limit,
                offset,
            } => {
                let trades = if let Some(u) = user {
                    let mut request = data
                        .positions(&u)
                        .trades()
                        .limit(limit)
                        .offset(offset)
                        .taker_only(taker_only);

                    if let Some(ref ids) = market {
                        let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
                        request = request.market(ids);
                    }
                    if let Some(ref ids) = event_id {
                        let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
                        request = request.event_id(ids);
                    }
                    if let Some(s) = side {
                        request = request.side(s.into());
                    }
                    if let Some(ft) = filter_type {
                        request = request.filter_type(ft.into());
                    }
                    if let Some(fa) = filter_amount {
                        request = request.filter_amount(fa);
                    }

                    request.send().await?
                } else {
                    let mut request = data
                        .trades()
                        .list()
                        .limit(limit)
                        .offset(offset)
                        .taker_only(taker_only);

                    if let Some(ref ids) = market {
                        let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
                        request = request.market(ids);
                    }
                    if let Some(ref ids) = event_id {
                        let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
                        request = request.event_id(ids);
                    }
                    if let Some(s) = side {
                        request = request.side(s.into());
                    }
                    if let Some(ft) = filter_type {
                        request = request.filter_type(ft.into());
                    }
                    if let Some(fa) = filter_amount {
                        request = request.filter_amount(fa);
                    }

                    request.send().await?
                };

                println!("{}", serde_json::to_string_pretty(&trades)?);
            }
        }
        Ok(())
    }
}

/// Trade side filter
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TradeSideFilter {
    /// Buy trades
    Buy,
    /// Sell trades
    Sell,
}

impl From<TradeSideFilter> for polyte_data::types::TradeSide {
    fn from(side: TradeSideFilter) -> Self {
        match side {
            TradeSideFilter::Buy => Self::Buy,
            TradeSideFilter::Sell => Self::Sell,
        }
    }
}

/// Trade filter type
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TradeFilterField {
    /// Filter by cash amount
    Cash,
    /// Filter by token amount
    Tokens,
}

impl From<TradeFilterField> for polyte_data::types::TradeFilterType {
    fn from(filter: TradeFilterField) -> Self {
        match filter {
            TradeFilterField::Cash => Self::Cash,
            TradeFilterField::Tokens => Self::Tokens,
        }
    }
}
