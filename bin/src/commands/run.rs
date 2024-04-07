use crate::commands::reqwest::send;
use crate::errors::ParsingError;
use crate::util::input_to_string;
use clap::Args;
use colored::Colorize;
use log::info;
use miette::{miette, LabeledSpan, Report};
use rede_parser::parse_request;
use std::time::Duration;

/// Executes the provided HTTP request
#[derive(Debug, Args)]
pub struct Command {
    /// Request file to execute
    #[arg(default_value = "-")]
    request: String,
    /// Timeout, in a string like \[0-9]+(ns|us|ms|\[smhdwy], for example "3m"
    #[arg(short, long)]
    timeout: Option<String>,
}

impl Command {
    pub async fn run(self) -> miette::Result<()> {
        info!("Run request {}", self.request);

        let content = input_to_string(&self.request)?;
        let request = parse_request(&content).map_err(|e| ParsingError::parsing(content, e))?;

        let args = RequestArgs::try_from(&self)?;
        let response = send(request, args).await?;

        println!("{}", response.bold());
        Ok(())
    }
}

pub struct RequestArgs {
    pub timeout: Option<Duration>,
}

impl TryFrom<&Command> for RequestArgs {
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

        Ok(RequestArgs { timeout })
    }
}
