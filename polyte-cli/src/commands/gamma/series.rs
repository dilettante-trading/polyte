use clap::{ArgAction, Subcommand};
use color_eyre::eyre::Result;
use polyte_gamma::Gamma;

#[derive(Subcommand)]
pub enum SeriesCommand {
    /// List series
    List {
        /// Maximum number of results
        #[arg(short, long)]
        limit: Option<u32>,
        /// Pagination offset
        #[arg(short, long)]
        offset: Option<u32>,
        /// Sort in ascending order
        #[arg(long, action = ArgAction::SetTrue, conflicts_with = "desc")]
        asc: bool,
        /// Sort in descending order
        #[arg(long, action = ArgAction::SetTrue)]
        desc: bool,
        /// Show only closed series
        #[arg(long, action = ArgAction::SetTrue, conflicts_with = "open")]
        closed: bool,
        /// Show only open series
        #[arg(long, action = ArgAction::SetTrue)]
        open: bool,
    },
    /// Get a series by ID
    Get {
        /// Series ID
        id: String,
    },
}

impl SeriesCommand {
    pub async fn run(self, gamma: &Gamma) -> Result<()> {
        match self {
            Self::List {
                limit,
                offset,
                asc,
                desc,
                closed,
                open,
            } => {
                let mut request = gamma.series().list();

                if let Some(l) = limit {
                    request = request.limit(l);
                }
                if let Some(o) = offset {
                    request = request.offset(o);
                }
                if asc {
                    request = request.ascending(true);
                } else if desc {
                    request = request.ascending(false);
                }
                if closed {
                    request = request.closed(true);
                } else if open {
                    request = request.closed(false);
                }

                let series = request.send().await?;
                println!("{}", serde_json::to_string_pretty(&series)?);
            }
            Self::Get { id } => {
                let series = gamma.series().get(&id).send().await?;
                println!("{}", serde_json::to_string_pretty(&series)?);
            }
        }
        Ok(())
    }
}
