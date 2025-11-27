use clap::Subcommand;
use color_eyre::eyre::Result;
use polyte_gamma::Gamma;

#[derive(Subcommand)]
pub enum SportsCommand {
    /// List sports metadata
    List,
    /// List teams
    Teams {
        /// Maximum number of results
        #[arg(short, long)]
        limit: Option<u32>,
        /// Pagination offset
        #[arg(short, long)]
        offset: Option<u32>,
        /// Sort in ascending order
        #[arg(long)]
        ascending: Option<bool>,
        /// Order by field
        #[arg(long)]
        order: Option<String>,
        /// Filter by league
        #[arg(long)]
        league: Option<String>,
    },
}

impl SportsCommand {
    pub async fn run(self, gamma: &Gamma) -> Result<()> {
        match self {
            Self::List => {
                let sports = gamma.sport().list().send().await?;
                println!("{}", serde_json::to_string_pretty(&sports)?);
            }
            Self::Teams {
                limit,
                offset,
                ascending,
                order,
                league,
            } => {
                let mut request = gamma.sport().list_teams();

                if let Some(l) = limit {
                    request = request.limit(l);
                }
                if let Some(o) = offset {
                    request = request.offset(o);
                }
                if let Some(asc) = ascending {
                    request = request.ascending(asc);
                }
                if let Some(ord) = order {
                    request = request.order(ord);
                }
                if let Some(l) = league {
                    request = request.league([l]);
                }

                let teams = request.send().await?;
                println!("{}", serde_json::to_string_pretty(&teams)?);
            }
        }
        Ok(())
    }
}
