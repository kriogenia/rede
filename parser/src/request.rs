use crate::error::Error;
use crate::schema::{QueryParams, Schema};

pub struct Request {
    pub method: String,
    pub url: String,
    pub query_params: Vec<(String, String)>,
}

impl TryFrom<Schema> for Request {
    type Error = Error;

    fn try_from(schema: Schema) -> Result<Self, Self::Error> {
        let query_params = schema
            .query_params
            .map(QueryParams::into_pairs)
            .unwrap_or_default();
        Ok(Self {
            method: schema.http.method.to_uppercase(),
            url: schema.http.url,
            query_params,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::schema::{Http, Schema};
    use toml::map::Map;
    use toml::Value;

    #[test]
    fn from_schema() {
        let mut query_params = Map::new();
        query_params.insert(
            "qp".to_string(),
            Value::Array(vec![Value::String("s".to_string()), Value::Integer(1)]),
        );

        let schema = Schema {
            http: Http {
                url: "url".to_string(),
                method: "get".to_string(),
            },
            query_params: Some(QueryParams(query_params)),
        };
        let request = Request::try_from(schema).unwrap();
        assert_eq!(request.url, "url");
        assert_eq!(request.method, "GET");
        assert_eq!(
            request.query_params,
            vec![("qp".to_string(), "s,1".to_string())]
        );
    }
}
