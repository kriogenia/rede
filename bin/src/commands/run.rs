use crate::errors::InnerError;
use crate::util::read_to_string;
use clap::Args;
use log::info;
use rede_parser::{parse_request, Request};
use reqwest::{Client, Request as Reqwest, RequestBuilder, Response, Url};

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
        let request = parse_request(&content).map_err(|e| InnerError::parsing(content, e))?;

        let response = send(request).await.unwrap().text().await.unwrap();

        info!("{response:?}");
        Ok(())
    }
}

async fn send(request: Request) -> reqwest::Result<Response> {
    let url = Url::parse(&request.url).expect("valid url");
    let reqwest = Reqwest::new(request.method, url);
    let builder = RequestBuilder::from_parts(Client::new(), reqwest);
    builder.send().await
}
