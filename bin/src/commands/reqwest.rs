use crate::commands::run::RequestArgs;
use rede_parser::Request;
use reqwest::{ClientBuilder, Request as Reqwest, RequestBuilder, Url};

use crate::errors::RequestError;

pub async fn send(req: Request, args: RequestArgs) -> Result<String, RequestError<reqwest::Error>> {
    let url = Url::parse(&req.url).map_err(|e| RequestError::invalid_url(&req.url, e))?;

    let mut client = ClientBuilder::new();
    if let Some(timeout) = args.timeout {
        client = client.timeout(timeout);
    }

    let reqwest = Reqwest::new(req.method, url);
    // todo handle build errors
    let builder = RequestBuilder::from_parts(client.build()?, reqwest).version(req.http_version);
    // todo handle send errors
    // todo handle text errors
    Ok(builder.send().await?.text().await?)
}
