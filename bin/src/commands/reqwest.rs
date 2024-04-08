use crate::commands::run::RequestArgs;
use http::header::CONTENT_TYPE;
use http::HeaderMap;
use log::debug;
use mime::Mime;
use rede_parser::{Body, Request};
use reqwest::redirect::Policy;
use reqwest::{Client, ClientBuilder, Request as Reqwest, RequestBuilder, Url};

use crate::errors::RequestError;

pub async fn send(req: Request, args: RequestArgs) -> Result<String, RequestError<reqwest::Error>> {
    let url = Url::parse(&req.url).map_err(|e| RequestError::invalid_url(&req.url, e))?;

    let client = build_client(&args)?;
    let reqwest = Reqwest::new(req.method, url);

    let builder = RequestBuilder::from_parts(client, reqwest)
        .version(req.http_version)
        .query(&req.query_params);

    let mut headers = req.headers;

    let builder = match req.body {
        Body::Raw { mime, content } => {
            set_content_type(&mut headers, &mime);
            builder.body(content)
        }
        Body::None => builder,
        _ => unimplemented!(),
    }
    .headers(headers);

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

fn set_content_type(headers: &mut HeaderMap, mime: &Mime) {
    if !headers.contains_key(CONTENT_TYPE) {
        debug!("adding key from body: {mime}");
        headers.insert(CONTENT_TYPE, mime.to_string().parse().unwrap());
    }
}
