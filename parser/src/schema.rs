mod query_params;

pub(crate) use query_params::QueryParams;

use crate::error::Error;
use serde::Deserialize;
use std::str::FromStr;
use toml::Value;

#[derive(Deserialize)]
pub(crate) struct Schema {
    pub http: Http,
    #[serde(alias = "queryparams", alias = "query-params")]
    pub query_params: Option<QueryParams>,
}

#[derive(Deserialize)]
pub(crate) struct Http {
    pub url: String,
    #[serde(default = "default_method")]
    pub method: String,
}

impl FromStr for Schema {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let schema: Schema = toml::from_str(s)?;

        if let Some(qp) = &schema.query_params {
            if qp.has_value(|v| matches!(v, Value::Datetime(_))) {
                return Err(Error::invalid_type("queryparams", "datetime"));
            }
            if qp.has_value(|v| matches!(v, Value::Table(_))) {
                return Err(Error::invalid_type("queryparams", "table"));
            }
        }

        Ok(schema)
    }
}

#[inline]
fn default_method() -> String {
    "GET".to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::request::Request;

    const ALL: &str = r#"
    [http]
    method = "GET"
    url = "https://example.org/api"

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
        assert_eq!(schema.http.method, "GET");
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
        assert!(matches!(
            Schema::from_str("").err().unwrap(),
            Error::MissingField(str) if str == "missing field `http`"
        ));
        assert!(matches!(
            Request::from_str("[http]").err().unwrap(),
            Error::MissingField(str) if str == "missing field `url`"
        ));
    }

    #[test]
    fn default_values() {
        let toml = r#"http.url = "url""#;
        let schema = Schema::from_str(toml).unwrap();
        assert_eq!(schema.http.method, "GET");
    }

    #[test]
    fn invalid_types() {
        let toml = r#"
        [http]
        method = "GET"
        url = "url"

        [queryparams]
        date = 1970-01-01
        "#;
        assert!(matches!(
            Schema::from_str(toml).err().unwrap(),
            Error::InvalidType { field, invalid_type } if field == "queryparams" && invalid_type == "datetime"
        ))
    }
}
