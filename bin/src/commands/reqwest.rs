use crate::commands::run::ClientProperties;
use http::header::CONTENT_TYPE;
use http::HeaderMap;
use log::debug;
use mime::Mime;
use rede_schema::body::FormDataValue;
use rede_schema::{Body, Request};
use reqwest::redirect::Policy;
use reqwest::{multipart, ClientBuilder, Request as Reqwest, RequestBuilder, Response, Url};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::errors::RequestError;

type Error = RequestError<reqwest::Error>;

pub struct Client {
    properties: ClientProperties,
}

impl Client {
    pub fn new(properties: ClientProperties) -> Self {
        Self { properties }
    }

    pub async fn send(self, req: Request) -> Result<Response, Error> {
        let url = Url::parse(&req.url).map_err(|e| RequestError::invalid_url(&req.url, e))?;

        let client = self.build_client()?;
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
            Body::Binary { mime, path } => {
                set_content_type(&mut headers, &mime);
                let body = file_to_body(&path).await?;
                builder.body(body)
            }
            Body::FormData(map) => {
                let mut form = multipart::Form::new();
                for (k, v) in map {
                    form = match v {
                        FormDataValue::Text(content) => form.text(k, content),
                        FormDataValue::File(path) => {
                            let body = file_to_body(&path).await?;
                            form.part(k, multipart::Part::stream(body))
                        }
                    }
                }
                builder.multipart(form)
            }
            Body::XFormUrlEncoded(form) => builder.form(&form),
            Body::None => builder,
        }
        .headers(headers);

        Ok(builder.send().await?)
    }

    fn build_client(&self) -> Result<reqwest::Client, reqwest::Error> {
        let mut client = ClientBuilder::new();
        if let Some(timeout) = self.properties.timeout {
            client = client.timeout(timeout);
        }
        client = match (self.properties.no_redirect, self.properties.max_redirects) {
            (true, _) => client.redirect(Policy::none()),
            (false, Some(val)) => client.redirect(Policy::limited(val)),
            _ => client,
        };
        client.build()
    }
}

fn set_content_type(headers: &mut HeaderMap, mime: &Mime) {
    if !headers.contains_key(CONTENT_TYPE) {
        debug!("adding key from body: {mime}");
        headers.insert(CONTENT_TYPE, mime.to_string().parse().unwrap());
    }
}

async fn file_to_body(path: &str) -> Result<reqwest::Body, Error> {
    let file = File::open(path)
        .await
        .map_err(|e| RequestError::io(path, e))?;
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = reqwest::Body::wrap_stream(stream);
    Ok(body)
}
