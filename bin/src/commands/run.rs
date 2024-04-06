use clap::Args;
use colored::Colorize;
use log::info;
use rede_parser::{parse_request, Request};
use reqwest::{Client, Request as Reqwest, RequestBuilder, Url};

use crate::errors::{ParsingError, RequestError};
use crate::util::read_to_string;

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

async fn send(request: Request) -> Result<String, RequestError<reqwest::Error>> {
    let url = Url::parse(&request.url).expect("valid url");
    let client = Client::new();
    let reqwest = Reqwest::new(request.method, url);
    let builder = RequestBuilder::from_parts(client, reqwest).version(request.http_version);
    Ok(builder.send().await?.text().await?)
}
