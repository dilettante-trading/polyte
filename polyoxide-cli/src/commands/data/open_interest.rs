use clap::Args;
use color_eyre::eyre::Result;
use polyoxide_data::DataApi;

use crate::commands::common::parsing::parse_comma_separated;

#[derive(Args)]
pub struct OpenInterestCommand {
    /// Filter by market condition IDs (comma-separated, optional)
    #[arg(short, long, value_parser = parse_comma_separated)]
    pub market: Option<Vec<String>>,
}

impl OpenInterestCommand {
    pub async fn run(self, data: &DataApi) -> Result<()> {
        let mut request = data.open_interest().get();
        if let Some(ref ids) = self.market {
            let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
            request = request.market(ids);
        }
        let open_interest = request.send().await?;
        println!("{}", serde_json::to_string_pretty(&open_interest)?);
        Ok(())
    }
}
