use thiserror::Error;

/// Errors that can happen during the parsing
#[derive(Error, Debug, PartialEq)]
pub enum Error {
    /// Error triggered while parsing the TOML file. Some common errors bundled on this one are:
    /// - Missing required keys
    /// - Duplicated keys
    /// - Bad formatting...
    #[error("{0}")]
    ParsingToml(#[from] toml::de::Error),
}
