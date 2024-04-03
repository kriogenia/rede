use clap::Args;

/// Executes the provided HTTP request
#[derive(Debug, Args)]
pub struct Command {
    /// Request file to execute
    request: String,
}

impl Command {
    pub fn run(self) {
        println!("Run request {}", self.request)
    }
}
