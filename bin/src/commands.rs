use std::ops::Not;
use std::sync::OnceLock;

use clap::{Parser, Subcommand};

use crate::terminal::{Terminal, TERM_LOCK};

mod example;
mod reqwest;
mod run;

#[derive(Debug, Parser)]
#[command(
    name = "rede",
    about,
    after_help = "Check the full documentation: https://rede.sotoestevez.dev",
    next_help_heading = "Global options",
    infer_long_args = true,
    infer_subcommands = true,
    term_width = 100,
    version
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Command,
    /// Enables all printing messages
    #[arg(long, global = true, conflicts_with = "quiet")]
    verbose: bool,
    /// Disables all printing messages
    #[arg(long, global = true)]
    quiet: bool,
    /// Disables output coloring
    #[arg(long, global = true)]
    no_color: bool,
}

static COLOR: OnceLock<bool> = OnceLock::new();

impl Cli {
    pub fn run(self) -> miette::Result<()> {
        TERM_LOCK
            .set(Terminal::new(self.quiet, self.verbose))
            .expect("terminal to be created");

        console::set_colors_enabled(self.no_color.not());
        COLOR.set(self.no_color.not()).unwrap();
        miette::set_hook(Box::new(|_| {
            Box::new(
                miette::MietteHandlerOpts::new()
                    .color(*COLOR.get().unwrap())
                    .tab_width(4)
                    .build(),
            )
        }))
        .unwrap();

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
    Example(example::Command),
}

trait RedeCommand {
    async fn run(self) -> miette::Result<()>;
}

impl RedeCommand for Command {
    async fn run(self) -> miette::Result<()> {
        match self {
            Command::Run(c) => c.run().await,
            Command::Example(c) => c.run().await,
        }
    }
}
