use clap::{Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use polyoxide_data::DataApi;

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

impl From<TradeSideFilter> for polyoxide_data::types::TradeSide {
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

impl From<TradeFilterField> for polyoxide_data::types::TradeFilterType {
    fn from(filter: TradeFilterField) -> Self {
        match filter {
            TradeFilterField::Cash => Self::Cash,
            TradeFilterField::Tokens => Self::Tokens,
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use polyoxide_data::types::{TradeFilterType, TradeSide};

    use super::*;

    fn try_parse(args: &[&str]) -> Result<TradesCommand, clap::Error> {
        #[derive(Parser)]
        struct Wrapper {
            #[command(subcommand)]
            cmd: TradesCommand,
        }
        Wrapper::try_parse_from(args).map(|w| w.cmd)
    }

    #[test]
    fn trade_side_filter_from_buy() {
        let side: TradeSide = TradeSideFilter::Buy.into();
        assert!(matches!(side, TradeSide::Buy));
    }

    #[test]
    fn trade_side_filter_from_sell() {
        let side: TradeSide = TradeSideFilter::Sell.into();
        assert!(matches!(side, TradeSide::Sell));
    }

    #[test]
    fn trade_filter_field_from_cash() {
        let ft: TradeFilterType = TradeFilterField::Cash.into();
        assert!(matches!(ft, TradeFilterType::Cash));
    }

    #[test]
    fn trade_filter_field_from_tokens() {
        let ft: TradeFilterType = TradeFilterField::Tokens.into();
        assert!(matches!(ft, TradeFilterType::Tokens));
    }

    #[test]
    fn list_defaults() {
        let cmd = try_parse(&["test", "list"]).unwrap();
        match cmd {
            TradesCommand::List {
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
                assert!(user.is_none());
                assert!(market.is_none());
                assert!(event_id.is_none());
                assert!(side.is_none());
                assert!(taker_only);
                assert!(filter_type.is_none());
                assert!(filter_amount.is_none());
                assert_eq!(limit, 100);
                assert_eq!(offset, 0);
            }
        }
    }

    #[test]
    fn list_with_user() {
        let cmd = try_parse(&["test", "list", "--user", "0xabc"]).unwrap();
        match cmd {
            TradesCommand::List { user, .. } => {
                assert_eq!(user.unwrap(), "0xabc");
            }
        }
    }

    #[test]
    fn list_with_side_buy() {
        let cmd = try_parse(&["test", "list", "--side", "buy"]).unwrap();
        match cmd {
            TradesCommand::List { side, .. } => {
                assert!(matches!(side.unwrap(), TradeSideFilter::Buy));
            }
        }
    }

    #[test]
    fn list_with_side_sell() {
        let cmd = try_parse(&["test", "list", "--side", "sell"]).unwrap();
        match cmd {
            TradesCommand::List { side, .. } => {
                assert!(matches!(side.unwrap(), TradeSideFilter::Sell));
            }
        }
    }

    #[test]
    fn list_invalid_side_errors() {
        let result = try_parse(&["test", "list", "--side", "short"]);
        assert!(result.is_err());
    }

    #[test]
    fn list_with_filter_type_cash() {
        let cmd = try_parse(&["test", "list", "--filter-type", "cash"]).unwrap();
        match cmd {
            TradesCommand::List { filter_type, .. } => {
                assert!(matches!(filter_type.unwrap(), TradeFilterField::Cash));
            }
        }
    }

    #[test]
    fn list_with_filter_type_tokens() {
        let cmd = try_parse(&["test", "list", "--filter-type", "tokens"]).unwrap();
        match cmd {
            TradesCommand::List { filter_type, .. } => {
                assert!(matches!(filter_type.unwrap(), TradeFilterField::Tokens));
            }
        }
    }

    #[test]
    fn list_invalid_filter_type_errors() {
        let result = try_parse(&["test", "list", "--filter-type", "volume"]);
        assert!(result.is_err());
    }

    #[test]
    fn list_without_market_is_none() {
        let cmd = try_parse(&["test", "list"]).unwrap();
        match cmd {
            TradesCommand::List { market, .. } => {
                assert!(market.is_none());
            }
        }
    }

    #[test]
    fn list_with_custom_limit_offset() {
        let cmd = try_parse(&["test", "list", "-l", "50", "-o", "200"]).unwrap();
        match cmd {
            TradesCommand::List { limit, offset, .. } => {
                assert_eq!(limit, 50);
                assert_eq!(offset, 200);
            }
        }
    }
}
