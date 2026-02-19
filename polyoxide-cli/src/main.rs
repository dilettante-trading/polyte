use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;

mod commands;

#[derive(Parser)]
#[command(name = "polyoxide")]
#[command(version, about = "CLI tool for querying Polymarket APIs", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Query Data API (user positions)
    Data {
        #[command(subcommand)]
        command: commands::DataCommand,
    },
    /// Query Gamma API (market data)
    Gamma {
        #[command(subcommand)]
        command: commands::GammaCommand,
    },
    /// Subscribe to WebSocket channels (real-time updates)
    Ws {
        #[command(subcommand)]
        command: commands::WsCommand,
    },
    /// Generate shell completions
    Completions(commands::CompletionsCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Data { command } => command.run().await?,
        Commands::Gamma { command } => command.run().await?,
        Commands::Ws { command } => command.run().await?,
        Commands::Completions(cmd) => cmd.run::<Cli>(),
    }

    Ok(())
}
