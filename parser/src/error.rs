use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    MissingField(String),
    #[error("{field} can't be of type {invalid_type}")]
    InvalidType { field: String, invalid_type: String },
    #[error("{0}")]
    InvalidValue(String),
    #[error("{0}")]
    Other(Box<dyn std::error::Error>),
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
        let msg = value.message().to_owned();
        if msg.starts_with("missing") {
            return Error::MissingField(msg);
        }
        if msg.starts_with("unknown") {
            return Error::InvalidValue(msg);
        }
        Error::Other(Box::new(value))
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
        assert_eq!(
            Error::InvalidType {
                field: "field".to_string(),
                invalid_type: "type".to_string()
            }
            .to_string(),
            "field can't be of type type"
        )
    }
}
