use crate::error::Error;
use crate::schema::{Metadata, QueryParams, Schema};
use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub url: String,
    pub http_version: String,
    pub metadata: HashMap<String, String>,
    pub query_params: Vec<(String, String)>,
}

impl TryFrom<Schema> for Request {
    type Error = Error;

    fn try_from(schema: Schema) -> Result<Self, Self::Error> {
        let metadata = schema.metadata.map(Metadata::into_map).unwrap_or_default();
        let query_params = schema
            .query_params
            .map(QueryParams::into_param_pairs)
            .unwrap_or_default();
        Ok(Self {
            method: schema.http.method.to_uppercase(),
            url: schema.http.url,
            http_version: schema.http.version.to_string(),
            metadata,
            query_params,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::schema::{Http, HttpVersion, Schema};
    use toml::map::Map;
    use toml::Value;

    #[test]
    fn from_schema() {
        let mut metadata = Map::new();
        metadata.insert("name".to_string(), Value::String("test".to_string()));

        let mut query_params = Map::new();
        query_params.insert(
            "qp".to_string(),
            Value::Array(vec![Value::String("s".to_string()), Value::Integer(1)]),
        );

        let schema = Schema {
            http: Http {
                url: "url".to_string(),
                method: "get".to_string(),
                version: HttpVersion::OnePointOne,
            },
            metadata: Some(Metadata::new(metadata)),
            query_params: Some(QueryParams::new(query_params)),
        };
        let request = Request::try_from(schema).unwrap();
        assert_eq!(request.url, "url");
        assert_eq!(request.method, "GET");
        assert_eq!(request.http_version, "HTTP/1.1");
        assert_eq!(request.metadata["name"], "test");
        assert_eq!(
            request.query_params,
            vec![("qp".to_string(), "s,1".to_string())]
        );
    }
}
