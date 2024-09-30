use crate::{standard, verbose};
use clap::Args;
use console::style;
use miette::{miette, Report};
use std::io::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use super::GlobalArgs;

/// Generate an example request file to run
#[derive(Debug, Args)]
#[command(
    after_help = "Documentation: https://rede.sotoestevez.dev/reference_guide/command_line_interface/example.html"
)]
pub struct Command;

const EXAMPLE: &str = include_str!("../static/example.toml");

impl Command {
    pub async fn run(self, gargs: GlobalArgs) -> miette::Result<()> {
        let rede = style("rede").bold().cyan();

        standard!("Welcome to {rede}\n");
        verbose!(
            "{}",
            style("! Did you use --verbose with this command? I like you\n").yellow()
        );
        standard!(
            "The following code snippet is a valid {rede} request that you can \
                    use to test the run command. Don't sweat it, there's no need for \
                    you to copy it, we have already created a {}\n",
            style("example.toml").yellow().bold()
        );
        standard!("```\n{EXAMPLE}\n```\n");
        standard!("Now just run: {}", style("rede run example").cyan());

        if !gargs.dry_run {
            // feels kinda bad to not run this concurrently ith the printing but dry-run fucks it a bit
            File::create("example.toml")
                .await
                .map_err(map_err)?
                .write_all(EXAMPLE.as_ref())
                .await
                .map_err(map_err)?;
        }

        Ok(())
    }
}

#[allow(clippy::needless_pass_by_value)]
fn map_err(e: Error) -> Report {
    miette!("There was a problem creating the example file: {}", e)
}
