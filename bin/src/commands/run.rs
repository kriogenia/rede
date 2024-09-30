mod print;

use crate::commands::reqwest::Client;
use crate::commands::RedeCommand;
use crate::errors::ParsingError;
use crate::util::input_to_string;
use clap::{ArgAction, Args};
use console::style;
use log::{info, trace};
use miette::{miette, LabeledSpan, Report};
use rede_parser::parse_request;
use std::time::Duration;

use super::GlobalArgs;

/// Executes the provided HTTP request
#[derive(Debug, Args)]
#[command(
    after_help = "Documentation: https://rede.sotoestevez.dev/reference_guide/command_line_interface/run.html"
)]
pub struct Command {
    /// Request file to execute
    #[arg(default_value = "-")]
    request: String,
    /// Specifies if formatting should be applied to response body, by default is true
    #[arg(
        long,
        value_name = "true|false",
        default_missing_value ="true",
        default_value = "true",
        num_args(0..=1),
        require_equals = true,
        action = ArgAction::Set,
    )]
    pretty_print: bool,
    /// Timeout, in a string like [0-9]+(ns|us|ms|\[smhdwy\], for example "3m"
    #[arg(
        long,
        help = r#"Timeout, in a string like [0-9]+(ns|us|ms|[smhdwy], for example "3m"#
    )]
    timeout: Option<String>,
    /// Disallows auto-redirection
    #[arg(long)]
    no_redirect: bool,
    /// Maximum number of redirects allowed, by default 10.
    #[arg(long)]
    max_redirects: Option<usize>,
}

impl RedeCommand for Command {
    async fn run(self, gargs: GlobalArgs) -> miette::Result<()> {
        info!("Launched rede run with {}", self.request);

        let content = input_to_string(&self.request)?;
        trace!("Content: {content}");

        let request = parse_request(&content).map_err(|e| ParsingError::parsing(content, e))?;
        self.print_request(&request);
        if gargs.dry_run {
            return Ok(());
        }

        let client = Client::new((&self).try_into()?);

        let response = client.send(request);
        let bar = self.new_spinner();
        let response = response.await?;
        bar.finish_and_clear();
        self.print_response(response).await;

        Ok(())
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
                    style("--timeout").italic().yellow()
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
