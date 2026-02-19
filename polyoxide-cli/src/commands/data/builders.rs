use clap::{Args, Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use polyoxide_data::{api::builders::TimePeriod, DataApi};

#[derive(Subcommand)]
pub enum BuildersCommand {
    /// Get aggregated builder leaderboard
    Leaderboard(LeaderboardCommand),
    /// Get daily builder volume time series
    Volume(VolumeCommand),
}

impl BuildersCommand {
    pub async fn run(self, data: &DataApi) -> Result<()> {
        match self {
            Self::Leaderboard(cmd) => cmd.run(data).await,
            Self::Volume(cmd) => cmd.run(data).await,
        }
    }
}

/// Get aggregated builder leaderboard
#[derive(Args)]
pub struct LeaderboardCommand {
    /// Time period for aggregation
    #[arg(short, long, default_value = "day")]
    pub time_period: CliTimePeriod,
    /// Maximum number of results (0-50)
    #[arg(short, long, default_value = "25")]
    pub limit: u32,
    /// Pagination offset (0-1000)
    #[arg(short, long, default_value = "0")]
    pub offset: u32,
}

impl LeaderboardCommand {
    pub async fn run(self, data: &DataApi) -> Result<()> {
        let rankings = data
            .builders()
            .leaderboard()
            .time_period(self.time_period.into())
            .limit(self.limit)
            .offset(self.offset)
            .send()
            .await?;
        println!("{}", serde_json::to_string_pretty(&rankings)?);
        Ok(())
    }
}

/// Get daily builder volume time series
#[derive(Args)]
pub struct VolumeCommand {
    /// Time period filter
    #[arg(short, long, default_value = "day")]
    pub time_period: CliTimePeriod,
}

impl VolumeCommand {
    pub async fn run(self, data: &DataApi) -> Result<()> {
        let volumes = data
            .builders()
            .volume()
            .time_period(self.time_period.into())
            .send()
            .await?;
        println!("{}", serde_json::to_string_pretty(&volumes)?);
        Ok(())
    }
}

/// Time period for aggregation
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum CliTimePeriod {
    /// Daily aggregation
    #[default]
    Day,
    /// Weekly aggregation
    Week,
    /// Monthly aggregation
    Month,
    /// All time aggregation
    All,
}

impl From<CliTimePeriod> for TimePeriod {
    fn from(period: CliTimePeriod) -> Self {
        match period {
            CliTimePeriod::Day => TimePeriod::Day,
            CliTimePeriod::Week => TimePeriod::Week,
            CliTimePeriod::Month => TimePeriod::Month,
            CliTimePeriod::All => TimePeriod::All,
        }
    }
}
