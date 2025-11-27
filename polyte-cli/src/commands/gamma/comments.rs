use clap::Subcommand;
use color_eyre::eyre::Result;
use polyte_gamma::Gamma;

#[derive(Subcommand)]
pub enum CommentsCommand {
    /// List comments
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
        /// Order by field
        #[arg(long)]
        order: Option<String>,
        /// Filter by parent entity type (Event, Series, market)
        #[arg(long)]
        parent_entity_type: Option<String>,
        /// Filter by parent entity ID
        #[arg(long)]
        parent_entity_id: Option<i64>,
        /// Include position data
        #[arg(long)]
        get_positions: Option<bool>,
        /// Filter to position holders only
        #[arg(long)]
        holders_only: Option<bool>,
    },
}

impl CommentsCommand {
    pub async fn run(self, gamma: &Gamma) -> Result<()> {
        match self {
            Self::List {
                limit,
                offset,
                ascending,
                order,
                parent_entity_type,
                parent_entity_id,
                get_positions,
                holders_only,
            } => {
                let mut request = gamma.comments().list();

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
                if let Some(pet) = parent_entity_type {
                    request = request.parent_entity_type(pet);
                }
                if let Some(pei) = parent_entity_id {
                    request = request.parent_entity_id(pei);
                }
                if let Some(gp) = get_positions {
                    request = request.get_positions(gp);
                }
                if let Some(ho) = holders_only {
                    request = request.holders_only(ho);
                }

                let comments = request.send().await?;
                println!("{}", serde_json::to_string_pretty(&comments)?);
            }
        }
        Ok(())
    }
}
