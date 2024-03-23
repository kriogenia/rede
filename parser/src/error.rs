use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("{field} can't be of type {invalid_type}")]
    InvalidType { field: String, invalid_type: String },
    #[error("{0}")]
    InvalidFile(#[from] toml::de::Error),
}

impl Error {
    pub(crate) fn invalid_type<T: Into<String>>(field: T, invalid_type: T) -> Self {
        Self::InvalidType {
            field: field.into(),
            invalid_type: invalid_type.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display() {
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
