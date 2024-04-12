use crate::{standard, verbose};
use console::{style, Style};
use http::{HeaderMap, StatusCode};
use log::{debug, error};
use rede_parser::{Body, Request};
use reqwest::Response;
use serde_json::{from_str, to_string_pretty};

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

        // TODO print each method in a different color
        verbose!(
            "{} {}",
            style(request.method.as_str()).bold().yellow(),
            style(url).underlined().blue(),
        );
        verbose!("{:?}", request.http_version);

        print_headers(&request.headers);

        if let Some(mime) = request.body.mime() {
            verbose!("[{}]", style(mime).cyan());
        }
        match &request.body {
            Body::Raw { content, .. } => verbose!("{content}\n"),
            Body::Binary { path, .. } => verbose!("    @{path}\n"),
            Body::XFormUrlEncoded(map) => {
                let query = map
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<String>>()
                    .join(&style("&").blue().to_string());
                verbose!("{query}\n");
            }
            Body::FormData(form) => {
                for (k, v) in form {
                    verbose!("{}: {}", style(k).blue(), v);
                }
                verbose!("");
            }
            Body::None => {}
        }
    }

    pub(crate) async fn print_response(&self, response: Response) {
        let status_color = status_color(response.status());

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

fn print_headers(headers: &HeaderMap) {
    // TODO use if_verbose! to omit this loop
    for (header_key, header_value) in headers {
        verbose!(
            "  - {} : {}",
            header_key,
            header_value.to_str().unwrap_or("<no ascii>")
        );
    }
    verbose!("");
}

fn status_color(status_code: StatusCode) -> Style {
    match status_code.as_u16() {
        100..=199 => Style::new().cyan(),
        200..=299 => Style::new().green(),
        300..=399 => Style::new().yellow(),
        400..=599 => Style::new().red(),
        _ => Style::new(),
    }
    .bold()
}
