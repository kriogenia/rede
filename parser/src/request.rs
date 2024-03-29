use crate::body::Body;
use crate::error::Error;
use crate::schema::Schema;
use http::{HeaderMap, Method, Version};
use std::collections::HashMap;

/// Representation of a rede HTTP request. Contains all the supported content by the current schema
/// to allow the creation and dispatching of the HTTP request with the command-line interface.
pub struct Request {
    pub method: Method,
    pub url: String,
    pub http_version: Version,
    pub metadata: HashMap<String, String>,
    pub headers: HeaderMap,
    pub query_params: Vec<(String, String)>,
    pub path_params: HashMap<String, String>,
    pub body: Body,
}

impl TryFrom<Schema> for Request {
    type Error = Error;

    fn try_from(schema: Schema) -> Result<Self, Self::Error> {
        Ok(Self {
            method: schema.http.method,
            url: schema.http.url,
            http_version: schema.http.version,
            metadata: schema.metadata.into_map(),
            headers: schema.headers,
            query_params: schema.query_params.into_param_pairs(),
            path_params: schema.path_params.into_map(),
            body: schema.body,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::body::Body;
    use crate::schema::{Http, QueryParams, Schema, StrStrTable};
    use toml::map::Map;
    use toml::Value;

    #[test]
    fn from_schema() {
        let mut metadata = Map::new();
        metadata.insert("name".to_string(), Value::String("test".to_string()));

        let mut headers = HeaderMap::new();
        headers.insert("Header", "Value".parse().unwrap());

        let mut query_params = Map::new();
        query_params.insert(
            "qp".to_string(),
            Value::Array(vec![Value::String("s".to_string()), Value::Integer(1)]),
        );

        let mut path_params = Map::new();
        path_params.insert("pp".to_string(), Value::String("value".to_string()));

        let body = Body::Binary("path".to_string());

        let schema = Schema {
            http: Http {
                url: "url".to_string(),
                method: Method::GET,
                version: Version::HTTP_11,
            },
            headers,
            metadata: StrStrTable::new(metadata),
            query_params: QueryParams::new(query_params),
            path_params: StrStrTable::new(path_params),
            body,
        };

        let request = Request::try_from(schema).unwrap();
        assert_eq!(request.url, "url");
        assert_eq!(request.method, Method::GET);
        assert_eq!(request.http_version, Version::HTTP_11);
        assert_eq!(request.metadata["name"], "test");
        assert_eq!(request.headers["Header"], "Value");
        assert_eq!(
            request.query_params,
            vec![("qp".to_string(), "s,1".to_string())]
        );
        assert_eq!(request.path_params["pp"], "value");
        assert_eq!(request.body, Body::Binary("path".to_string()));
    }
}
