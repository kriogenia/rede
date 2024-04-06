use crate::errors::RequestError;
use rede_parser::Request;
use reqwest::{Client, Request as Reqwest, RequestBuilder, Url};

// Wrapping all the reqwest logic into this signature should allow to easily change clients in the
// future or enable/disable them via featuers
pub async fn send(request: Request) -> Result<String, RequestError<reqwest::Error>> {
    let url = Url::parse(&request.url).expect("valid url");
    let client = Client::new();
    let reqwest = Reqwest::new(request.method, url);
    let builder = RequestBuilder::from_parts(client, reqwest).version(request.http_version);
    Ok(builder.send().await?.text().await?)
}
