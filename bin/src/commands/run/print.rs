use crate::{standard, verbose};
use console::{style, Style};
use http::{HeaderMap, Method, StatusCode};
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error};
use rede_parser::{Body, Request};
use reqwest::Response;
use serde_json::{from_str, to_string_pretty};
use std::time::Duration;

impl super::Command {
    pub(crate) fn print_request(&self, request: &Request) {
        debug!("{request:?}");

        verbose!(
            "{} Executing request {}\n",
            style(">").bold().cyan(),
            style(request.metadata.get("name").unwrap_or(&self.request)).yellow()
        );

        let output_arrows = style(">>>").bold().blue();
        verbose!(
            "{} {} {}\n",
            &output_arrows,
            style("HTTP Request").bold(),
            &output_arrows
        );

        let query = if request.query_params.is_empty() {
            String::new()
        } else {
            let query = request
                .query_params
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<String>>()
                .join("&");
            format!("?{query}")
        };

        let url = format!("{}{}", request.url, query);
        let method = method_style(&request.method).apply_to(request.method.as_str());

        verbose!("{method} {}", style(url).underlined().blue(),);
        verbose!("{:?}", request.http_version);

        print_headers(&request.headers);

        if let Some(mime) = request.body.mime() {
            verbose!("[{}]", style(mime).cyan());
        }
        match &request.body {
            Body::Raw { content, .. } => verbose!("{content}"),
            Body::Binary { path, .. } => verbose!("    @{path}"),
            Body::XFormUrlEncoded(map) => {
                let query = map
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<String>>()
                    .join(&style("&").blue().to_string());
                verbose!("{query}");
            }
            Body::FormData(form) => {
                for (k, v) in form {
                    verbose!("{}: {}", style(k).blue(), v);
                }
            }
            Body::None => {}
        }
    }

    pub(crate) async fn print_response(&self, response: Response) {
        let status_color = status_style(response.status());

        let output_arrows = status_color.apply_to("<<<");
        verbose!(
            "{} {} {}\n",
            &output_arrows,
            style("HTTP Response").bold(),
            &output_arrows
        );

        let status = status_color.apply_to(response.status());
        verbose!("{status} - {}", style(response.url()).underlined().blue());
        verbose!("{:?}", response.version());

        print_headers(response.headers());

        let body = response.text().await;
        if body.is_err() {
            error!("{}", body.unwrap_err());
            standard!(
                " {} The response body seems to not be printable",
                style("x").red().bold()
            );
            return;
        }
        let body = body.unwrap();

        if self.pretty_print {
            if let Ok(json) = from_str::<serde_json::Value>(&body) {
                standard!("{}", to_string_pretty(&json).unwrap());
                return;
            }
        }

        if body.is_empty() {
            standard!(below[Verbose] "{status}");
        }
        standard!("{body}");
    }
}

const SPINNER_TEMPLATE: &str = "{spinner:.cyan/blue} Waiting for the response: {elapsed}";
pub fn new_spinner() -> ProgressBar {
    let bar = ProgressBar::new_spinner()
        .with_prefix("this could be the request if you know...")
        .with_style(ProgressStyle::with_template(SPINNER_TEMPLATE).unwrap());
    bar.enable_steady_tick(Duration::from_millis(100));
    bar
}

fn print_headers(headers: &HeaderMap) {
    // TODO create if_verbose! to wrap this loop and omit it
    for (header_key, header_value) in headers {
        verbose!(
            "  - {} : {}",
            header_key,
            header_value.to_str().unwrap_or("<no ascii>")
        );
    }
    verbose!("");
}

fn method_style(method: &Method) -> Style {
    // postfix match would be quite nice here
    match *method {
        Method::GET => Style::new().green(),
        Method::POST => Style::new().blue(),
        Method::PUT => Style::new().cyan(),
        Method::PATCH => Style::new().magenta(),
        Method::DELETE => Style::new().red(),
        _ => Style::new().yellow(),
    }
    .bold()
}

fn status_style(status_code: StatusCode) -> Style {
    match status_code.as_u16() {
        100..=199 => Style::new().cyan(),
        200..=299 => Style::new().green(),
        300..=399 => Style::new().yellow(),
        400..=599 => Style::new().red(),
        _ => Style::new(),
    }
    .bold()
}
