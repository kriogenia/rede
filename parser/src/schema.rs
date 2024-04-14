use std::str::FromStr;

use http::{HeaderMap, Method, Version};
use serde::Deserialize;

pub(crate) use body::Body;

use crate::error::Error;
use crate::schema::table::PrimitiveArrTable;

mod body;
pub(crate) mod table;
pub(crate) mod types;

/// Model of the supported request schema contents.
#[derive(Deserialize)]
#[cfg_attr(test, derive(Default))]
pub(crate) struct Schema {
    pub http: Http,
    #[serde(default)]
    pub metadata: PrimitiveArrTable,
    #[serde(with = "http_serde::header_map", default)]
    pub headers: HeaderMap,
    #[serde(alias = "queryparams", alias = "query-params", default)]
    pub query_params: PrimitiveArrTable,
    #[serde(default)]
    pub body: Body,
    #[serde(default)]
    pub variables: PrimitiveArrTable,
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
        Ok(schema)
    }
}

#[cfg(test)]
mod test {
    use crate::schema::types::{Primitive, PrimitiveArray};

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

    [variables]
    string = "string"
    integer = 5
    float = 1.2
    boolean = false
    array = [ 1, "2" ]

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
            PrimitiveArray::Single(Primitive::Str("Test request".to_string()))
        );
        assert_eq!(
            schema.metadata.0["description"],
            PrimitiveArray::Single(Primitive::Str(
                "Request with all supported options".to_string()
            ))
        );
        assert_eq!(schema.headers.len(), 2);
        assert_eq!(schema.headers["Content-Type"], "application/toml");
        assert_eq!(schema.headers["Api-Version"], "v2");
        assert_eq!(schema.query_params.0.len(), 5);
        assert_eq!(
            schema.query_params.0["string"],
            PrimitiveArray::Single(Primitive::Str("string".into()))
        );
        assert_eq!(
            schema.query_params.0["integer"],
            PrimitiveArray::Single(Primitive::Int(10))
        );
        assert_eq!(
            schema.query_params.0["float"],
            PrimitiveArray::Single(Primitive::Float(0.1))
        );
        assert_eq!(
            schema.query_params.0["boolean"],
            PrimitiveArray::Single(Primitive::Bool(true))
        );
        assert_eq!(
            schema.query_params.0["array"],
            PrimitiveArray::Multiple(vec![
                Primitive::Str("first".into()),
                Primitive::Str("second".into()),
            ])
        );
        assert_eq!(schema.variables.0.len(), 5);
        assert_eq!(
            schema.variables.0["string"],
            PrimitiveArray::Single(Primitive::Str("string".to_string()))
        );
        assert_eq!(
            schema.variables.0["integer"],
            PrimitiveArray::Single(Primitive::Int(5))
        );
        assert_eq!(
            schema.variables.0["float"],
            PrimitiveArray::Single(Primitive::Float(1.2))
        );
        assert_eq!(
            schema.variables.0["boolean"],
            PrimitiveArray::Single(Primitive::Bool(false))
        );
        assert_eq!(
            schema.variables.0["array"],
            PrimitiveArray::Multiple(vec![Primitive::Int(1), Primitive::Str("2".into()),])
        );
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
        assert!(schema.variables.0.is_empty());
        assert_eq!(schema.body, Body::None);
    }
}
