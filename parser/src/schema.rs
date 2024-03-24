use std::str::FromStr;

use http::{Method, Version};
use serde::Deserialize;

pub(crate) use table::Metadata;
pub(crate) use table::QueryParams;

use crate::error::Error;
use crate::schema::validation::validate_types;

mod table;
mod validation;

#[derive(Deserialize)]
pub(crate) struct Schema {
    pub http: Http,
    pub metadata: Option<Metadata>,
    #[serde(alias = "queryparams", alias = "query-params")]
    pub query_params: Option<QueryParams>,
}

#[derive(Deserialize)]
pub(crate) struct Http {
    pub url: String,
    #[serde(with = "http_serde::method", default)]
    pub method: Method,
    #[serde(with = "http_serde::version", default)]
    pub version: Version,
}

#[cfg(test)]
impl Schema {
    pub fn new() -> Self {
        Self {
            http: Http {
                url: "url".to_string(),
                method: Method::GET,
                version: Version::HTTP_11,
            },
            metadata: None,
            query_params: None,
        }
    }
}

impl FromStr for Schema {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let schema: Schema = toml::from_str(s)?;
        validate_types(&schema)?;
        Ok(schema)
    }
}

#[cfg(test)]
mod test {
    use toml::Value;

    use super::*;

    const ALL: &str = r#"
    [http]
    method = "GET"
    url = "https://example.org/api"
    version = "HTTP/1.1"

    [metadata]
    name = "Test request"
    description = "Request with all supported options"

    [queryparams]
    string = "string"
    integer = 10
    float = 0.1
    array = [ "first", "second" ]
    boolean = true
    "#;

    #[test]
    fn deserialize_all() {
        let mut schema: Schema = toml::from_str(ALL).unwrap();
        assert_eq!(schema.http.url, "https://example.org/api");
        assert_eq!(schema.http.method, Method::GET);
        assert_eq!(schema.http.version, Version::HTTP_11);
        let metadata = schema.metadata.take().unwrap();
        assert_eq!(metadata.0.len(), 2);
        assert_eq!(
            metadata.0["name"],
            Value::String("Test request".to_string())
        );
        assert_eq!(
            metadata.0["description"],
            Value::String("Request with all supported options".to_string())
        );
        let query_params = schema.query_params.take().unwrap();
        assert_eq!(query_params.0.len(), 5);
        assert_eq!(
            *query_params.0.get("string").unwrap(),
            Value::String("string".into())
        );
        assert_eq!(*query_params.0.get("integer").unwrap(), Value::Integer(10));
        assert_eq!(*query_params.0.get("float").unwrap(), Value::Float(0.1));
        assert_eq!(
            *query_params.0.get("boolean").unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            *query_params.0.get("array").unwrap(),
            Value::Array(vec![
                Value::String("first".into()),
                Value::String("second".into()),
            ])
        );
    }

    #[test]
    fn missing_fields() {
        assert!(Schema::from_str("")
            .err()
            .unwrap()
            .to_string()
            .contains("missing field `http"));
        assert!(Schema::from_str("[http]")
            .err()
            .unwrap()
            .to_string()
            .contains("missing field `url`"));
    }

    #[test]
    fn default_values() {
        let toml = r#"http.url = "url""#;
        let schema = Schema::from_str(toml).unwrap();
        assert_eq!(schema.http.method, Method::GET);
        assert_eq!(schema.http.version, Version::HTTP_11);
    }

    #[test]
    fn invalid_type() {
        let toml = r#"
        [http]
        method = "GET"
        url = "url"

        [queryparams]
        date = 1970-01-01
        "#;
        assert_eq!(
            Schema::from_str(toml).err().unwrap(),
            Error::InvalidType {
                field: "values of [query_params]".to_string(),
                invalid_type: "datetime".to_string()
            }
        )
    }
}
