use clap::Subcommand;
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
        #[arg(long)]
        ascending: Option<bool>,
        /// Filter by closed status
        #[arg(short, long)]
        closed: Option<bool>,
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
                ascending,
                closed,
            } => {
                let mut request = gamma.series().list();

                if let Some(l) = limit {
                    request = request.limit(l);
                }
                if let Some(o) = offset {
                    request = request.offset(o);
                }
                if let Some(asc) = ascending {
                    request = request.ascending(asc);
                }
                if let Some(c) = closed {
                    request = request.closed(c);
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
