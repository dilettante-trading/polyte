use clap::{Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use polyte_gamma::Gamma;

/// Sort order
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum SortOrder {
    /// Ascending order
    Asc,
    /// Descending order
    #[default]
    Desc,
}

/// Parent entity type for comments
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ParentEntityType {
    /// Event comments
    Event,
    /// Series comments
    Series,
    /// Market comments
    Market,
}

impl ParentEntityType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Event => "Event",
            Self::Series => "Series",
            Self::Market => "market",
        }
    }
}

#[derive(Subcommand)]
pub enum CommentsCommand {
    /// List comments
    List {
        /// Maximum number of results
        #[arg(short, long, default_value = "20")]
        limit: u32,
        /// Pagination offset
        #[arg(short, long, default_value = "0")]
        offset: u32,
        /// Sort order
        #[arg(long, value_enum, default_value = "desc")]
        sort: SortOrder,
        /// Order by field
        #[arg(long)]
        order: Option<String>,
        /// Filter by parent entity type
        #[arg(long, value_enum)]
        parent_entity_type: Option<ParentEntityType>,
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
                sort,
                order,
                parent_entity_type,
                parent_entity_id,
                get_positions,
                holders_only,
            } => {
                let mut request = gamma
                    .comments()
                    .list()
                    .limit(limit)
                    .offset(offset)
                    .ascending(matches!(sort, SortOrder::Asc));

                if let Some(ord) = order {
                    request = request.order(ord);
                }
                if let Some(pet) = parent_entity_type {
                    request = request.parent_entity_type(pet.as_str());
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
