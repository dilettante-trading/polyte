use clap::{ArgAction, Subcommand};
use color_eyre::eyre::Result;
use polyte_gamma::Gamma;

#[derive(Subcommand)]
pub enum EventsCommand {
    /// List events
    List {
        /// Maximum number of results
        #[arg(short, long)]
        limit: Option<u32>,
        /// Pagination offset
        #[arg(short, long)]
        offset: Option<u32>,
        /// Show only active events
        #[arg(long, action = ArgAction::SetTrue, conflicts_with = "inactive")]
        active: bool,
        /// Show only inactive events
        #[arg(long, action = ArgAction::SetTrue)]
        inactive: bool,
        /// Show only closed events
        #[arg(long, action = ArgAction::SetTrue, conflicts_with = "open")]
        closed: bool,
        /// Show only open events
        #[arg(long, action = ArgAction::SetTrue)]
        open: bool,
        /// Show only archived events
        #[arg(long, action = ArgAction::SetTrue, conflicts_with = "not_archived")]
        archived: bool,
        /// Exclude archived events
        #[arg(long, action = ArgAction::SetTrue)]
        not_archived: bool,
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
                inactive,
                closed,
                open,
                archived,
                not_archived,
                featured,
                not_featured,
                liquidity_min,
                liquidity_max,
                volume_min,
                volume_max,
                asc,
                desc,
                order,
            } => {
                let mut request = gamma.events().list();

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
                if asc {
                    request = request.ascending(true);
                } else if desc {
                    request = request.ascending(false);
                }
                if let Some(ord) = order {
                    request = request.order(ord);
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
