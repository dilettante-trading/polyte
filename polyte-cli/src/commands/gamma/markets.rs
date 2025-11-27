use clap::Subcommand;
use color_eyre::eyre::Result;
use polyte_gamma::Gamma;

#[derive(Subcommand)]
pub enum MarketsCommand {
    /// List markets
    List {
        /// Maximum number of results
        #[arg(short, long)]
        limit: Option<u32>,
        /// Pagination offset
        #[arg(short, long)]
        offset: Option<u32>,
        /// Filter by active status
        #[arg(short, long)]
        active: Option<bool>,
        /// Filter by closed status
        #[arg(short, long)]
        closed: Option<bool>,
        /// Filter by archived status
        #[arg(long)]
        archived: Option<bool>,
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
        #[arg(long)]
        ascending: Option<bool>,
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
                limit,
                offset,
                active,
                closed,
                archived,
                liquidity_min,
                liquidity_max,
                volume_min,
                volume_max,
                ascending,
                order,
            } => {
                let mut request = gamma.markets().list();

                if let Some(l) = limit {
                    request = request.limit(l);
                }
                if let Some(o) = offset {
                    request = request.offset(o);
                }
                if let Some(a) = active {
                    request = request.active(a);
                }
                if let Some(c) = closed {
                    request = request.closed(c);
                }
                if let Some(a) = archived {
                    request = request.archived(a);
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
                if let Some(asc) = ascending {
                    request = request.ascending(asc);
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
