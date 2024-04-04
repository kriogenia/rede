use crate::file::open_file_or_stdin;
use clap::Args;
use std::io::Read;

/// Executes the provided HTTP request
#[derive(Debug, Args)]
pub struct Command {
    /// Request file to execute
    #[arg(default_value = "-")]
    request: String,
}

impl Command {
    pub fn run(self) -> miette::Result<()> {
        println!("Run request {}", self.request);
        let mut buffer = open_file_or_stdin(&self.request)?;
        let mut content = String::new();
        buffer
            .read_to_string(&mut content)
            .expect("TODO: map to miette error");
        println!("{content}");
        Ok(())
    }
}
