use clap::Args;
use color_eyre::eyre::Result;
use polyoxide_data::DataApi;

use crate::commands::common::parsing::parse_comma_separated;

/// Get top holders for markets
#[derive(Args)]
pub struct HoldersCommand {
    /// Market condition IDs (comma-separated, required)
    #[arg(short, long, value_parser = parse_comma_separated)]
    market: Vec<String>,
    /// Maximum number of holders per market between 0 and 500
    #[arg(short, long, default_value = "100")]
    limit: u32,
    /// Minimum balance filter between 0 and 999999
    #[arg(long, default_value = "1")]
    min_balance: u32,
}

impl HoldersCommand {
    pub async fn run(self, data: &DataApi) -> Result<()> {
        let ids: Vec<&str> = self.market.iter().map(|s| s.as_str()).collect();
        let request = data
            .holders()
            .list(ids)
            .limit(self.limit)
            .min_balance(self.min_balance);

        let holders = request.send().await?;
        println!("{}", serde_json::to_string_pretty(&holders)?);
        Ok(())
    }
}
