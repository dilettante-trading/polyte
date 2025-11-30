use clap::{Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use polyte_gamma::Gamma;

use crate::commands::gamma::SortOrder;

/// Market status filter
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum MarketStatus {
    /// Open markets (not closed, not archived)
    #[default]
    Open,
    /// Closed markets
    Closed,
    /// Archived markets
    Archived,
}

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
        #[arg(short, long, default_value = "20")]
        limit: u32,
        /// Pagination offset
        #[arg(short, long, default_value = "0")]
        offset: u32,
        /// Filter by active status
        #[arg(long, default_value = "true")]
        active: bool,
        /// Filter by status (open, closed, archived)
        #[arg(short, long, value_enum, default_value = "open")]
        status: MarketStatus,
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
        /// Sort order
        #[arg(long, value_enum, default_value = "desc")]
        sort: SortOrder,
        /// Order by field
        #[arg(long)]
        order: Option<String>,
    },
    /// Get a market by ID
    Get {
        /// Market ID
        id: String,
    },
    /// Get a market by slug
    GetBySlug {
        /// Market slug
        slug: String,
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
                status,
                liquidity_min,
                liquidity_max,
                volume_min,
                volume_max,
                sort,
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
                request = request.limit(limit).offset(offset).active(active);
                match status {
                    MarketStatus::Open => {
                        request = request.closed(false).archived(false);
                    }
                    MarketStatus::Closed => {
                        request = request.closed(true);
                    }
                    MarketStatus::Archived => {
                        request = request.archived(true);
                    }
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
                request = request.ascending(matches!(sort, SortOrder::Asc));
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
            Self::GetBySlug { slug } => {
                let market = gamma.markets().get_by_slug(&slug).send().await?;
                println!("{}", serde_json::to_string_pretty(&market)?);
            }
        }
        Ok(())
    }
}
