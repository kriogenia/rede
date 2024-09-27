use std::collections::HashMap;

use http::{HeaderMap, Method, Version};
use rede_schema::Body;

use crate::error::Error;
use crate::schema::table::Transform;
use crate::schema::Schema;

#[cfg(feature = "input_params")]
use crate::InputParam;

/// Representation of a rede HTTP request. Contains all the supported content by the current schema
/// to allow the creation and dispatching of the HTTP request with the command-line interface.
#[derive(Debug)]
pub struct Request {
    /// HTTP method of the request
    pub method: Method,
    /// URL of the request
    pub url: String,
    /// HTTP version of the request
    pub http_version: Version,
    /// Metadata of the request file
    pub metadata: HashMap<String, String>,
    /// Headers of the request
    pub headers: HeaderMap,
    /// Query parameters of the request
    pub query_params: Vec<(String, String)>,
    /// Body of the request
    pub body: Body,
    /// Variables to provide values for placeholders in the request
    pub variables: HashMap<String, String>,

    #[cfg(feature = "input_params")]
    /// Keys of placeholders to ask the user for input
    pub input_params: HashMap<String, InputParam>,
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
            query_params: schema.query_params.into_pairs(),
            variables: schema.variables.into_map(),
            body: schema.body.into(),

            #[cfg(feature = "input_params")]
            input_params: schema.input_params.0,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::schema;
    use crate::schema::table::Table;
    use crate::schema::types::{Primitive, PrimitiveArray};
    use crate::schema::{Http, Schema};
    use rede_schema::body::Body;

    use super::*;

    #[test]
    fn from_schema() {
        let mut metadata = HashMap::new();
        metadata.insert(
            "name".to_string(),
            PrimitiveArray::Single(Primitive::Str("test".to_string())),
        );

        let mut headers = HeaderMap::new();
        headers.insert("Header", "Value".parse().unwrap());

        let mut query_params = HashMap::new();
        query_params.insert(
            "qp".to_string(),
            PrimitiveArray::Multiple(vec![Primitive::Str("s".to_string()), Primitive::Int(1)]),
        );

        let mut variables = HashMap::new();
        variables.insert(
            "pp".to_string(),
            PrimitiveArray::Single(Primitive::Str("value".to_string())),
        );

        #[cfg(feature = "input_params")]
        let input_params = {
            let mut input_params = HashMap::new();
            input_params.insert(
                "ip".to_string(),
                InputParam {
                    hint: Some("hint".to_string()),
                    default: Some("127.0.0.1".to_string()),
                },
            );
            input_params
        };

        let body = schema::Body::Binary("path".to_string());

        let schema = Schema {
            http: Http {
                url: "url".to_string(),
                method: Method::GET,
                version: Version::HTTP_11,
            },
            headers,
            metadata: Table::new(metadata),
            query_params: Table::new(query_params),
            variables: Table::new(variables),
            body,

            #[cfg(feature = "input_params")]
            input_params: Table::new(input_params),
        };

        let request = Request::try_from(schema).unwrap();
        assert_eq!(request.url, "url");
        assert_eq!(request.method, Method::GET);
        assert_eq!(request.http_version, Version::HTTP_11);
        assert_eq!(request.metadata["name"], "test");
        assert_eq!(request.headers["Header"], "Value");
        assert_eq!(
            request.query_params,
            vec![
                ("qp".to_string(), "s".to_string()),
                ("qp".to_string(), "1".to_string()),
            ]
        );
        assert_eq!(request.variables["pp"], "value");
        assert_eq!(
            request.body,
            Body::Binary {
                path: "path".to_string(),
                mime: mime::APPLICATION_OCTET_STREAM,
            }
        );

        #[cfg(feature = "input_params")]
        assert_eq!(
            request.input_params["ip"],
            InputParam {
                hint: Some("hint".to_string()),
                default: Some("127.0.0.1".to_string()),
            }
        );
    }
}
