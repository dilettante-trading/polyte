use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use color_eyre::eyre::Result;

#[derive(Parser)]
pub struct CompletionsCommand {
    /// Shell to generate completions for
    #[arg(value_enum)]
    shell: Shell,
}

impl CompletionsCommand {
    pub fn run<C: CommandFactory>(&self) -> Result<()> {
        let mut cmd = C::command();
        generate(self.shell, &mut cmd, "polyoxide", &mut std::io::stdout());
        Ok(())
    }
}
