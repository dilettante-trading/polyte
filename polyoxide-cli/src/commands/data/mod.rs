mod activity;
mod builders;
mod holders;
mod live_volume;
mod open_interest;
mod positions;
mod traded;
mod trades;

use clap::{Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use polyoxide_data::DataApi;

use crate::commands::data::{
    activity::UserActivityCommand, holders::HoldersCommand, live_volume::LiveVolumeCommand,
    open_interest::OpenInterestCommand, positions::PositionsCommand, traded::TradedCommand,
};

#[derive(Subcommand)]
pub enum DataCommand {
    /// Check API health status
    Health,
    /// Query user activity
    Activity(UserActivityCommand),
    /// Query builder leaderboard and volume
    Builders {
        #[command(subcommand)]
        command: builders::BuildersCommand,
    },
    /// Query top holders for markets
    Holders(HoldersCommand),
    /// Query trades
    Trades {
        #[command(subcommand)]
        command: trades::TradesCommand,
    },
    /// Get traded markets by user
    Traded(TradedCommand),
    /// Query user-specific data (positions, traded count)
    Positions(PositionsCommand),
    /// Get open interest for markets
    OpenInterest(OpenInterestCommand),
    /// Get live volume for an event
    LiveVolume(LiveVolumeCommand),
}

impl DataCommand {
    pub async fn run(self) -> Result<()> {
        let data = DataApi::new()?;

        match self {
            Self::Health => {
                let health = data.health().check().await?;
                println!("{}", serde_json::to_string_pretty(&health)?);
                Ok(())
            }
            Self::Activity(cmd) => cmd.run(&data).await,
            Self::Builders { command } => command.run(&data).await,
            Self::Holders(cmd) => cmd.run(&data).await,
            Self::Trades { command } => command.run(&data).await,
            Self::Traded(cmd) => cmd.run(&data).await,
            Self::Positions(cmd) => cmd.run(&data).await,
            Self::OpenInterest(cmd) => cmd.run(&data).await,
            Self::LiveVolume(cmd) => cmd.run(&data).await,
        }
    }
}

/// Sort order
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum SortOrder {
    /// Ascending order
    Asc,
    /// Descending order
    #[default]
    Desc,
}

impl From<SortOrder> for polyoxide_data::types::SortDirection {
    fn from(order: SortOrder) -> Self {
        match order {
            SortOrder::Asc => Self::Asc,
            SortOrder::Desc => Self::Desc,
        }
    }
}
