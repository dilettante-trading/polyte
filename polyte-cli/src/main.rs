use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;

mod commands;

#[derive(Parser)]
#[command(name = "polyte")]
#[command(about = "CLI tool for querying Polymarket APIs", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Query Gamma API (market data)
    Gamma {
        #[command(subcommand)]
        command: commands::GammaCommand,
    },
    /// Generate shell completions
    Completions(commands::CompletionsCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Gamma { command } => command.run().await?,
        Commands::Completions(cmd) => cmd.run::<Cli>(),
    }

    Ok(())
}
