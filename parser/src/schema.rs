use std::str::FromStr;

use http::{HeaderMap, Method, Version};
use serde::Deserialize;
use toml::Value;

pub(crate) use body::Body;

use crate::error::Error;
use crate::schema::table::Table;
use crate::schema::validation::validate_types;

mod body;
pub(crate) mod table;
mod validation;

/// Model of the supported request schema contents.
#[derive(Deserialize)]
#[cfg_attr(test, derive(Default))]
pub(crate) struct Schema {
    pub http: Http,
    #[serde(default)]
    pub metadata: Table<Value>,
    #[serde(with = "http_serde::header_map", default)]
    pub headers: HeaderMap,
    #[serde(alias = "queryparams", alias = "query-params", default)]
    pub query_params: Table<Value>,
    #[serde(alias = "pathparams", alias = "path-params", default)]
    pub path_params: Table<Value>,
    #[serde(default)]
    pub body: Body,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(Default))]
pub(crate) struct Http {
    pub url: String,
    #[serde(with = "http_serde::method", default)]
    pub method: Method,
    #[serde(with = "http_serde::version", default)]
    pub version: Version,
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

    [headers]
    Content-Type = "application/toml"
    Api-Version = "v2"

    [queryparams]
    string = "string"
    integer = 10
    float = 0.1
    array = [ "first", "second" ]
    boolean = true

    [path-params]
    string = "string"
    integer = 5
    float = 1.2
    boolean = false

    [body]
    raw = """
    {
        "key": "value"
    }
    """
    "#;

    #[test]
    fn deserialize_all() {
        let schema: Schema = toml::from_str(ALL).unwrap();
        assert_eq!(schema.http.url, "https://example.org/api");
        assert_eq!(schema.http.method, Method::GET);
        assert_eq!(schema.http.version, Version::HTTP_11);
        assert_eq!(schema.metadata.0.len(), 2);
        assert_eq!(
            schema.metadata.0["name"],
            Value::String("Test request".to_string())
        );
        assert_eq!(
            schema.metadata.0["description"],
            Value::String("Request with all supported options".to_string())
        );
        assert_eq!(schema.headers.len(), 2);
        assert_eq!(schema.headers["Content-Type"], "application/toml");
        assert_eq!(schema.headers["Api-Version"], "v2");
        assert_eq!(schema.query_params.0.len(), 5);
        assert_eq!(
            schema.query_params.0["string"],
            Value::String("string".into())
        );
        assert_eq!(schema.query_params.0["integer"], Value::Integer(10));
        assert_eq!(schema.query_params.0["float"], Value::Float(0.1));
        assert_eq!(schema.query_params.0["boolean"], Value::Boolean(true));
        assert_eq!(
            schema.query_params.0["array"],
            Value::Array(vec![
                Value::String("first".into()),
                Value::String("second".into()),
            ])
        );
        assert_eq!(schema.path_params.0.len(), 4);
        assert_eq!(
            schema.path_params.0["string"],
            Value::String("string".to_string())
        );
        assert_eq!(schema.path_params.0["integer"], Value::Integer(5));
        assert_eq!(schema.path_params.0["float"], Value::Float(1.2));
        assert_eq!(schema.path_params.0["boolean"], Value::Boolean(false));
        let body: Body = schema.body.into();
        assert!(matches!(body, Body::Raw(content) if content.contains(r#""key": "value""#)));
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
        assert!(schema.metadata.0.is_empty());
        assert!(schema.headers.is_empty());
        assert!(schema.query_params.0.is_empty());
        assert!(schema.path_params.0.is_empty());
        assert_eq!(schema.body, Body::None);
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
