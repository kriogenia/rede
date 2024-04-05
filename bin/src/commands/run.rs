use crate::util::read_to_string;
use clap::Args;
use log::info;

/// Executes the provided HTTP request
#[derive(Debug, Args)]
pub struct Command {
    /// Request file to execute
    #[arg(default_value = "-")]
    request: String,
}

impl Command {
    pub fn run(self) -> miette::Result<()> {
        info!("Run request {}", self.request);

        let content = read_to_string(&self.request)?;

        info!("{content}");
        Ok(())
    }
}
