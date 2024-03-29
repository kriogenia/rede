use thiserror::Error;
use toml::Value;

/// Errors that can happen during the parsing
#[derive(Error, Debug, PartialEq)]
pub enum Error {
    /// Error triggered by submitting a key with a non-allowed TOML type.
    #[error("{field} can't be of type {invalid_type}")]
    InvalidType { field: String, invalid_type: String },
    /// Error triggered while parsing the TOML file. Some common errors bundled on this one are:
    /// - Missing required keys
    /// - Duplicated keys
    /// - Bad formatting...
    #[error("{0}")]
    InvalidFile(#[from] toml::de::Error),
}

impl Error {
    pub(crate) fn invalid_type<T: Into<String>>(field: T, invalid_type: &Value) -> Self {
        let value_type = match invalid_type {
            Value::String(_) => "string",
            Value::Integer(_) => "integer",
            Value::Float(_) => "float",
            Value::Boolean(..) => "boolean",
            Value::Datetime(_) => "datetime",
            Value::Array(_) => "array",
            Value::Table(_) => "table",
        };
        Self::InvalidType {
            field: field.into(),
            invalid_type: value_type.into(),
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
                field: "`field`".to_string(),
                invalid_type: "type".to_string()
            }
            .to_string(),
            "`field` can't be of type type"
        )
    }
}
