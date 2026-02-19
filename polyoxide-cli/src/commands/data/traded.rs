use clap::Args;
use color_eyre::eyre::Result;
use polyoxide_data::DataApi;

#[derive(Args)]
pub struct TradedCommand {
    /// User address (0x-prefixed, 40 hex chars)
    #[arg(short, long)]
    user: String,
}

impl TradedCommand {
    pub async fn run(self, data: &DataApi) -> Result<()> {
        let result = data.traded(&self.user).get().await?;
        println!("{}", serde_json::to_string_pretty(&result)?);
        Ok(())
    }
}
