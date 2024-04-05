use clap::Parser;

use crate::commands::Cli;

mod commands;
mod errors;
mod util;

fn main() -> miette::Result<()> {
    env_logger::init();
    Cli::parse().run()
}
