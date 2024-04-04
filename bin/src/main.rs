use clap::Parser;

use crate::commands::Cli;

mod commands;
mod file;

fn main() -> miette::Result<()> {
    Cli::parse().run()
}
