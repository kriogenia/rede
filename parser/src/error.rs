use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("{0}")]
    MissingField(String),
    #[error("{field} can't be of type {invalid_type}")]
    InvalidType { field: String, invalid_type: String },
}

impl Error {
    pub(crate) fn invalid_type<T: Into<String>>(field: T, invalid_type: T) -> Self {
        Self::InvalidType {
            field: field.into(),
            invalid_type: invalid_type.into(),
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Error::MissingField(value.message().to_owned())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            Error::MissingField("missing field `http`".to_string()).to_string(),
            "missing field `http`"
        );
    }
}
