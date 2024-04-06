use colored::Colorize;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum InnerError {
    #[error("Failed to read {}", filename.bold())]
    #[diagnostic(
        code("invalid request"),
        help("check if the file name is correct or you're in the correct path")
    )]
    IO {
        filename: String,
        source: std::io::Error,
    },
    #[error("{message}")]
    #[diagnostic(
        code("spec violation"),
        url("https://toml.io/en/v1.0.0"),
        help("check the TOML specification and rede schema if you don't know what is wrong")
    )]
    Parsing {
        message: String,
        #[source_code]
        code: String,
        #[label("here")]
        span: Option<SourceSpan>,
    },
    #[error(transparent)]
    #[diagnostic(
        code("spec violation: types"),
        help("usually keys accept only strings or everything except datetimes and tables")
    )] // todo: url to schema specification
    WrongType { source: rede_parser::Error },
}

impl InnerError {
    pub fn io<T: Into<String>>(filename: T, source: std::io::Error) -> Self {
        Self::IO {
            filename: filename.into(),
            source,
        }
    }

    pub fn parsing<T: Into<String>>(code: T, source: rede_parser::Error) -> Self {
        match source {
            rede_parser::Error::InvalidFile(e) => InnerError::Parsing {
                message: e.message().to_owned(),
                code: code.into(),
                span: e.span().map(SourceSpan::from),
            },
            rede_parser::Error::InvalidType { .. } => InnerError::WrongType { source },
        }
    }
}
