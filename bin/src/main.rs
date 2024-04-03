use clap::Parser;

use crate::commands::Cli;

mod commands;

fn main() {
    Cli::parse().run()
}
