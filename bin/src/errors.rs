use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum InnerError {
    #[error("Failed to read {filename}")]
    #[diagnostic(help("check if the file name is correct or you're in the correct path"))]
    IoError {
        filename: String,
        source: std::io::Error,
    },
}

impl InnerError {
    pub fn io<T: Into<String>>(filename: T, source: std::io::Error) -> Self {
        Self::IoError {
            filename: filename.into(),
            source,
        }
    }
}
