use clap::{Parser, Subcommand};

mod run;

#[derive(Debug, Parser)]
#[command(name = "rede", term_width = 100, about, version)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn run(self) {
        self.command.run()
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    Run(run::Command),
}

impl Command {
    pub fn run(self) {
        match self {
            Command::Run(c) => c.run(),
        }
    }
}
