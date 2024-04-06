use crate::commands::reqwest::send;
use crate::errors::ParsingError;
use crate::util::read_to_string;
use clap::Args;
use colored::Colorize;
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
    pub async fn run(self) -> miette::Result<()> {
        info!("Run request {}", self.request);

        let content = read_to_string(&self.request)?;
        let request = parse_request(&content).map_err(|e| ParsingError::parsing(content, e))?;

        let response = send(request).await?;

        println!("{}", response.bold());
        Ok(())
    }
}
