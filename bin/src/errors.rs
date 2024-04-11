use console::style;
use miette::{Diagnostic, SourceSpan};
use std::error::{Error as StdError, Error};
use std::io::Error as IOError;
use thiserror::Error;
use url::ParseError as UrlParseError;

#[derive(Debug, Diagnostic, Error)]
pub enum ParsingError {
    #[error("{message}")]
    #[diagnostic(
        code("spec violation"),
        url("https://toml.io/en/v1.0.0"),
        help("check the TOML specification and rede schema if you don't know what is wrong")
    )]
    Deserialization {
        message: String,
        #[source_code]
        code: String,
        #[label("here")]
        span: Option<SourceSpan>,
    },
    #[error("Failed to read {}", style(filename).yellow())]
    #[diagnostic(
        code("invalid [REQUEST]"),
        help("check if the file name is correct or you're in the correct path")
    )]
    IO { filename: String, source: IOError },
}

#[derive(Debug, Diagnostic, Error)]
pub enum RequestError<E: Error> {
    #[error(transparent)]
    #[diagnostic(code = "failed request building")]
    Building(E),
    #[error(transparent)]
    #[diagnostic(code = "failed connection")]
    FailedConnection(E),
    #[error("resulting url is not correct ({})", style(url).underlined().blue())]
    #[diagnostic(code("invalid url"))]
    InvalidUrl {
        url: String,
        source: url::ParseError,
    },
    #[error("a file defined on the request could not be loaded: {}", style(filename).yellow())]
    #[diagnostic(
        code("invalid file"),
        help(
            "request file paths must be relative to the directory where `rede` is being executed"
        )
    )]
    IO { filename: String, source: IOError },
    #[error(transparent)]
    #[diagnostic(code("redirect"))]
    Redirect(E),
    #[error(transparent)]
    #[diagnostic(code("timeout"))]
    Timeout(E),
    #[error(transparent)]
    #[diagnostic(
        code("wrong http version"),
        help("maybe that port or endpoint does not support this protocol version")
    )]
    WrongVersion(E),
    #[error(transparent)]
    #[diagnostic(
        code("unknown request error"),
        url("https://github.com/kriogenia/rede/issues"),
        help("if you contact with the development team with the error we could start tracking it")
    )]
    Unknown(E),
}

impl ParsingError {
    pub fn io<T: Into<String>>(filename: T, source: IOError) -> Self {
        Self::IO {
            filename: filename.into(),
            source,
        }
    }

    pub fn parsing<T: Into<String>>(code: T, source: rede_parser::Error) -> Self {
        match source {
            rede_parser::Error::ParsingToml(e) => ParsingError::Deserialization {
                message: e.message().to_owned(),
                code: code.into(),
                span: e.span().map(SourceSpan::from),
            },
        }
    }
}

impl<E: Error> RequestError<E> {
    pub fn invalid_url(url: &str, source: UrlParseError) -> Self {
        Self::InvalidUrl {
            url: url.to_string(),
            source,
        }
    }

    pub fn io<T: Into<String>>(filename: T, source: IOError) -> Self {
        Self::IO {
            filename: filename.into(),
            source,
        }
    }
}

impl From<reqwest::Error> for RequestError<reqwest::Error> {
    fn from(value: reqwest::Error) -> Self {
        if value.is_redirect() {
            RequestError::Redirect(value)
        } else if value.is_timeout() {
            RequestError::Timeout(value)
        } else if value.is_connect() {
            RequestError::FailedConnection(value)
        } else if value.is_builder() {
            RequestError::Building(value)
        } else if value.is_request()
            && value
                .source()
                .is_some_and(|s| s.to_string().contains("UserUnsupportedVersion"))
        {
            RequestError::WrongVersion(value)
        } else {
            RequestError::Unknown(value)
        }
    }
}
