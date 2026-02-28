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

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[derive(Parser)]
    struct TestLeaderboard {
        #[command(flatten)]
        cmd: LeaderboardCommand,
    }

    #[derive(Parser)]
    struct TestVolume {
        #[command(flatten)]
        cmd: VolumeCommand,
    }

    #[test]
    fn leaderboard_defaults() {
        let parsed = TestLeaderboard::try_parse_from(["test"]).unwrap();
        assert!(matches!(parsed.cmd.time_period, CliTimePeriod::Day));
        assert_eq!(parsed.cmd.limit, 25);
        assert_eq!(parsed.cmd.offset, 0);
    }

    #[test]
    fn leaderboard_custom_time_period_week() {
        let parsed = TestLeaderboard::try_parse_from(["test", "--time-period", "week"]).unwrap();
        assert!(matches!(parsed.cmd.time_period, CliTimePeriod::Week));
    }

    #[test]
    fn leaderboard_custom_time_period_month() {
        let parsed = TestLeaderboard::try_parse_from(["test", "--time-period", "month"]).unwrap();
        assert!(matches!(parsed.cmd.time_period, CliTimePeriod::Month));
    }

    #[test]
    fn leaderboard_custom_time_period_all() {
        let parsed = TestLeaderboard::try_parse_from(["test", "--time-period", "all"]).unwrap();
        assert!(matches!(parsed.cmd.time_period, CliTimePeriod::All));
    }

    #[test]
    fn leaderboard_invalid_time_period_errors() {
        let result = TestLeaderboard::try_parse_from(["test", "--time-period", "year"]);
        assert!(result.is_err());
    }

    #[test]
    fn leaderboard_custom_limit_and_offset() {
        let parsed = TestLeaderboard::try_parse_from(["test", "-l", "10", "-o", "50"]).unwrap();
        assert_eq!(parsed.cmd.limit, 10);
        assert_eq!(parsed.cmd.offset, 50);
    }

    #[test]
    fn volume_defaults() {
        let parsed = TestVolume::try_parse_from(["test"]).unwrap();
        assert!(matches!(parsed.cmd.time_period, CliTimePeriod::Day));
    }

    #[test]
    fn time_period_from_conversions() {
        assert!(matches!(
            TimePeriod::from(CliTimePeriod::Day),
            TimePeriod::Day
        ));
        assert!(matches!(
            TimePeriod::from(CliTimePeriod::Week),
            TimePeriod::Week
        ));
        assert!(matches!(
            TimePeriod::from(CliTimePeriod::Month),
            TimePeriod::Month
        ));
        assert!(matches!(
            TimePeriod::from(CliTimePeriod::All),
            TimePeriod::All
        ));
    }
}
