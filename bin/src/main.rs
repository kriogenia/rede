use clap::Parser;

use crate::commands::Cli;

mod commands;
mod errors;
mod util;

fn main() -> miette::Result<()> {
    Cli::parse().run()
}
