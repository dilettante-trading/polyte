use clap::Args;
use color_eyre::eyre::Result;
use polyoxide_data::DataApi;

#[derive(Args)]
pub struct LiveVolumeCommand {
    /// Event ID (must be >= 1)
    #[arg(short, long)]
    pub event_id: u64,
}

impl LiveVolumeCommand {
    pub async fn run(self, data: &DataApi) -> Result<()> {
        let volume = data.live_volume().get(self.event_id).await?;
        println!("{}", serde_json::to_string_pretty(&volume)?);
        Ok(())
    }
}
