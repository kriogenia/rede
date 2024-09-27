use rede_schema::Request;

use crate::error::Error;
use crate::schema::table::Transform;
use crate::schema::Schema;

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
            input_params: schema.input_params.into_map(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::schema;
    use crate::schema::table::Table;
    use crate::schema::types::{Primitive, PrimitiveArray};
    use crate::schema::{Http, Schema};
    use http::{HeaderMap, Method, Version};
    use rede_schema::body::Body;

    #[cfg(feature = "input_params")]
    use rede_schema::InputParam;

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
                schema::input_param::InputParam {
                    hint: Some("hint".to_string()),
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
            }
        );
    }
}
