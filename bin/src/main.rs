#![warn(clippy::pedantic)]

use clap::Parser;

use crate::commands::Cli;

mod commands;
mod errors;
mod terminal;
mod util;

fn main() -> miette::Result<()> {
    env_logger::init();
    Cli::parse().run()
}
