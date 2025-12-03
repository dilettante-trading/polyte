mod market;
mod user;

use clap::Subcommand;
use color_eyre::eyre::Result;

#[derive(Subcommand)]
pub enum WsCommand {
    /// Subscribe to market channel (order book, price changes)
    Market {
        #[command(flatten)]
        args: market::MarketArgs,
    },
    /// Subscribe to user channel (orders, trades) - requires authentication
    User {
        #[command(flatten)]
        args: user::UserArgs,
    },
}

impl WsCommand {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::Market { args } => market::run(args).await,
            Self::User { args } => user::run(args).await,
        }
    }
}
