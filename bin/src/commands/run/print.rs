use crate::{standard, verbose};
use console::style;
use log::debug;
use rede_parser::{Body, Request};

impl super::Command {
    pub(crate) fn print_request(&self, request: &Request) {
        debug!("{request:?}");

        standard!(
            "{} Executing request {}\n",
            style(">").bold().cyan(),
            style(request.metadata.get("name").unwrap_or(&self.request)).yellow()
        );

        let output_arrows = style(">>>").bold().cyan();
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
            style(url).underlined().cyan()
        );

        // TODO use if_verbose! to omit this loop
        for (header_key, header_value) in &request.headers {
            verbose!(
                "  - {} : {}",
                header_key,
                header_value.to_str().unwrap_or("<no ascii>")
            );
        }

        verbose!("");

        if let Some(mime) = request.body.mime() {
            verbose!("[{}]", style(mime).blue());
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
}
