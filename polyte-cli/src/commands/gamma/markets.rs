use clap::{ArgAction, Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use polyte_gamma::Gamma;

/// Preset filters for common market queries
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum MarketPreset {
    /// Active markets with high volume (>$100k) sorted by 24h volume
    Trending,
    /// Active markets sorted by total volume (descending)
    TopVolume,
    /// Active markets with high liquidity (>$50k)
    HighLiquidity,
    /// New markets (recently created)
    New,
    /// Active competitive markets
    Competitive,
}

#[derive(Subcommand)]
pub enum MarketsCommand {
    /// List markets
    List {
        /// Use a preset filter for common queries
        #[arg(short, long, value_enum)]
        preset: Option<MarketPreset>,
        /// Maximum number of results
        #[arg(short, long)]
        limit: Option<u32>,
        /// Pagination offset
        #[arg(short, long)]
        offset: Option<u32>,
        /// Show only active markets
        #[arg(long, action = ArgAction::SetTrue, conflicts_with = "inactive")]
        active: bool,
        /// Show only inactive markets
        #[arg(long, action = ArgAction::SetTrue)]
        inactive: bool,
        /// Show only closed markets
        #[arg(long, action = ArgAction::SetTrue, conflicts_with = "open")]
        closed: bool,
        /// Show only open markets
        #[arg(long, action = ArgAction::SetTrue)]
        open: bool,
        /// Show only archived markets
        #[arg(long, action = ArgAction::SetTrue, conflicts_with = "not_archived")]
        archived: bool,
        /// Exclude archived markets
        #[arg(long, action = ArgAction::SetTrue)]
        not_archived: bool,
        /// Minimum liquidity
        #[arg(long)]
        liquidity_min: Option<f64>,
        /// Maximum liquidity
        #[arg(long)]
        liquidity_max: Option<f64>,
        /// Minimum volume
        #[arg(long)]
        volume_min: Option<f64>,
        /// Maximum volume
        #[arg(long)]
        volume_max: Option<f64>,
        /// Sort in ascending order
        #[arg(long, action = ArgAction::SetTrue, conflicts_with = "desc")]
        asc: bool,
        /// Sort in descending order
        #[arg(long, action = ArgAction::SetTrue)]
        desc: bool,
        /// Order by field
        #[arg(long)]
        order: Option<String>,
    },
    /// Get a market by condition ID
    Get {
        /// Market condition ID
        id: String,
    },
}

impl MarketsCommand {
    pub async fn run(self, gamma: &Gamma) -> Result<()> {
        match self {
            Self::List {
                preset,
                limit,
                offset,
                active,
                inactive,
                closed,
                open,
                archived,
                not_archived,
                liquidity_min,
                liquidity_max,
                volume_min,
                volume_max,
                asc,
                desc,
                order,
            } => {
                let mut request = gamma.markets().list();

                // Apply preset filters first (can be overridden by explicit flags)
                request = match preset {
                    Some(MarketPreset::Trending) => request
                        .active(true)
                        .volume_num_min(100_000.0)
                        .order("volume24hr")
                        .ascending(false),
                    Some(MarketPreset::TopVolume) => {
                        request.active(true).order("volume").ascending(false)
                    }
                    Some(MarketPreset::HighLiquidity) => request
                        .active(true)
                        .liquidity_num_min(50_000.0)
                        .order("liquidity")
                        .ascending(false),
                    Some(MarketPreset::New) => {
                        request.active(true).order("startDate").ascending(false)
                    }
                    Some(MarketPreset::Competitive) => {
                        request.active(true).order("competitive").ascending(false)
                    }
                    None => request,
                };

                // Apply explicit overrides (these take precedence over presets)
                if let Some(l) = limit {
                    request = request.limit(l);
                }
                if let Some(o) = offset {
                    request = request.offset(o);
                }
                if active {
                    request = request.active(true);
                } else if inactive {
                    request = request.active(false);
                }
                if closed {
                    request = request.closed(true);
                } else if open {
                    request = request.closed(false);
                }
                if archived {
                    request = request.archived(true);
                } else if not_archived {
                    request = request.archived(false);
                }
                if let Some(min) = liquidity_min {
                    request = request.liquidity_num_min(min);
                }
                if let Some(max) = liquidity_max {
                    request = request.liquidity_num_max(max);
                }
                if let Some(min) = volume_min {
                    request = request.volume_num_min(min);
                }
                if let Some(max) = volume_max {
                    request = request.volume_num_max(max);
                }
                if asc {
                    request = request.ascending(true);
                } else if desc {
                    request = request.ascending(false);
                }
                if let Some(ord) = order {
                    request = request.order(ord);
                }

                let markets = request.send().await?;
                println!("{}", serde_json::to_string_pretty(&markets)?);
            }
            Self::Get { id } => {
                let market = gamma.markets().get(&id).send().await?;
                println!("{}", serde_json::to_string_pretty(&market)?);
            }
        }
        Ok(())
    }
}
