use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};

#[derive(Parser)]
pub struct CompletionsCommand {
    /// Shell to generate completions for
    #[arg(value_enum)]
    shell: Shell,
}

impl CompletionsCommand {
    pub fn run<C: CommandFactory>(&self) {
        let mut cmd = C::command();
        generate(self.shell, &mut cmd, "polyte", &mut std::io::stdout());
    }
}
