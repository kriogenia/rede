use crate::commands::run::RequestArgs;
use rede_parser::Request;
use reqwest::redirect::Policy;
use reqwest::{Client, ClientBuilder, Request as Reqwest, RequestBuilder, Url};

use crate::errors::RequestError;

pub async fn send(req: Request, args: RequestArgs) -> Result<String, RequestError<reqwest::Error>> {
    let url = Url::parse(&req.url).map_err(|e| RequestError::invalid_url(&req.url, e))?;

    let client = build_client(&args)?;
    let reqwest = Reqwest::new(req.method, url);
    let builder = RequestBuilder::from_parts(client, reqwest)
        .version(req.http_version)
        .headers(req.headers)
        .query(&req.query_params);
    // todo handle send errors
    // todo handle text errors
    Ok(builder.send().await?.text().await?)
}

fn build_client(args: &RequestArgs) -> Result<Client, reqwest::Error> {
    let mut client = ClientBuilder::new();
    if let Some(timeout) = args.timeout {
        client = client.timeout(timeout);
    }
    client = match (args.no_redirect, args.max_redirects) {
        (true, _) => client.redirect(Policy::none()),
        (false, Some(val)) => client.redirect(Policy::limited(val)),
        _ => client,
    };
    client.build()
}
