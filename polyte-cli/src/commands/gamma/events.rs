use clap::{ArgAction, Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use polyte_gamma::Gamma;

use crate::commands::gamma::SortOrder;

/// Event status filter
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum EventStatus {
    /// Open events (not closed, not archived)
    #[default]
    Open,
    /// Closed events
    Closed,
    /// Archived events
    Archived,
}

#[derive(Subcommand)]
pub enum EventsCommand {
    /// List events
    List {
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
        status: EventStatus,
        /// Show only featured events
        #[arg(long, action = ArgAction::SetTrue, conflicts_with = "not_featured")]
        featured: bool,
        /// Exclude featured events
        #[arg(long, action = ArgAction::SetTrue)]
        not_featured: bool,
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
        #[arg(long, default_value = "startDate")]
        order: String,
    },
    /// Get an event by ID
    Get {
        /// Event ID
        id: String,
    },
    /// Get an event by slug
    GetBySlug {
        /// Event slug
        slug: String,
    },
    /// Get related events by slug
    Related {
        /// Event slug
        slug: String,
    },
}

impl EventsCommand {
    pub async fn run(self, gamma: &Gamma) -> Result<()> {
        match self {
            Self::List {
                limit,
                offset,
                active,
                status,
                featured,
                not_featured,
                liquidity_min,
                liquidity_max,
                volume_min,
                volume_max,
                sort,
                order,
            } => {
                let mut request = gamma
                    .events()
                    .list()
                    .limit(limit)
                    .offset(offset)
                    .order(&order)
                    .active(active)
                    .ascending(matches!(sort, SortOrder::Asc));

                match status {
                    EventStatus::Open => {
                        request = request.closed(false).archived(false);
                    }
                    EventStatus::Closed => {
                        request = request.closed(true);
                    }
                    EventStatus::Archived => {
                        request = request.archived(true);
                    }
                }
                if featured {
                    request = request.featured(true);
                } else if not_featured {
                    request = request.featured(false);
                }
                if let Some(min) = liquidity_min {
                    request = request.liquidity_min(min);
                }
                if let Some(max) = liquidity_max {
                    request = request.liquidity_max(max);
                }
                if let Some(min) = volume_min {
                    request = request.volume_min(min);
                }
                if let Some(max) = volume_max {
                    request = request.volume_max(max);
                }

                let events = request.send().await?;
                println!("{}", serde_json::to_string_pretty(&events)?);
            }
            Self::Get { id } => {
                let event = gamma.events().get(&id).send().await?;
                println!("{}", serde_json::to_string_pretty(&event)?);
            }
            Self::GetBySlug { slug } => {
                let event = gamma.events().get_by_slug(&slug).send().await?;
                println!("{}", serde_json::to_string_pretty(&event)?);
            }
            Self::Related { slug } => {
                let events = gamma.events().get_related_by_slug(&slug).send().await?;
                println!("{}", serde_json::to_string_pretty(&events)?);
            }
        }
        Ok(())
    }
}
