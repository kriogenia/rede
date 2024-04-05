use clap::{Parser, Subcommand};

mod run;

#[derive(Debug, Parser)]
#[command(name = "rede", term_width = 100, about, version)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn run(self) -> miette::Result<()> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async { self.command.run().await })
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    Run(run::Command),
}

impl Command {
    pub async fn run(self) -> miette::Result<()> {
        match self {
            Command::Run(c) => c.run(),
        }
        .await
    }
}
