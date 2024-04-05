use crate::util::read_to_string;
use clap::Args;

/// Executes the provided HTTP request
#[derive(Debug, Args)]
pub struct Command {
    /// Request file to execute
    #[arg(default_value = "-")]
    request: String,
}

impl Command {
    pub fn run(self) -> miette::Result<()> {
        println!("Run request {}", self.request);

        let content = read_to_string(&self.request)?;

        println!("{content}");
        Ok(())
    }
}
