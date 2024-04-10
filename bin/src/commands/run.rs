use crate::commands::reqwest::Client;
use crate::errors::ParsingError;
use crate::util::input_to_string;
use crate::{standard, verbose};
use clap::Args;
use colored::Colorize;
use log::{debug, info, trace};
use miette::{miette, LabeledSpan, Report};
use rede_parser::{parse_request, Request};
use std::time::Duration;

/// Executes the provided HTTP request
#[derive(Debug, Args)]
pub struct Command {
    /// Request file to execute
    #[arg(default_value = "-")]
    request: String,
    /// Timeout, in a string like [0-9]+(ns|us|ms|[smhdwy], for example "3m"
    #[arg(long)]
    timeout: Option<String>,
    /// Disallows auto-redirection
    #[arg(long)]
    no_redirect: bool,
    /// Maximum number of redirects allowed, by default 10.
    #[arg(long)]
    max_redirects: Option<usize>,
}

impl Command {
    pub async fn run(self) -> miette::Result<()> {
        info!("Launched rede run with {}", self.request);

        let content = input_to_string(&self.request)?;
        trace!("Content: {content}");

        let request = parse_request(&content).map_err(|e| ParsingError::parsing(content, e))?;
        self.print_request(&request);

        let client = Client::new((&self).try_into()?);
        let response = client.send(request).await?;

        standard!("{}", response.italic());
        Ok(())
    }

    fn print_request(&self, request: &Request) {
        debug!("{request:?}");

        standard!(
            "{} Executing request {}\n",
            ">".bold().cyan(),
            request
                .metadata
                .get("name")
                .unwrap_or(&self.request)
                .yellow()
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
            request.method.as_str().yellow(),
            url.underline().cyan()
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
    }
}

pub struct ClientProperties {
    pub timeout: Option<Duration>,
    pub no_redirect: bool,
    pub max_redirects: Option<usize>,
}

impl TryFrom<&Command> for ClientProperties {
    type Error = Report;

    fn try_from(value: &Command) -> Result<Self, Self::Error> {
        let timeout = value
            .timeout
            .as_ref()
            .map(|t| duration_str::parse(t).map_err(|e| (e, t)))
            .transpose()
            .map_err(|(_, t)| {
                miette!(
                    code = "invalid argument: timeout",
                    url = "https://docs.rs/duration-str",
                    help = "duration is usually represented like: [0-9]+(ns|us|ms|[smhdwy])",
                    labels = vec![LabeledSpan::at(0..t.len(), "wrong value")],
                    "Failed to convert the {} into a valid duration",
                    "--timeout".italic().yellow()
                )
                .with_source_code(t.to_owned())
            })?;

        Ok(ClientProperties {
            timeout,
            no_redirect: value.no_redirect,
            max_redirects: value.max_redirects,
        })
    }
}
