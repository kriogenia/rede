use crate::errors::InnerError;
use crate::util::read_to_string;
use clap::Args;
use log::info;
use rede_parser::parse_request;

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
        let request = parse_request(&content).map_err(|e| InnerError::parsing(content, e))?;

        info!("{}", request.body);
        Ok(())
    }
}
