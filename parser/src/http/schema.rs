use crate::method::Method;
use serde::Deserialize;
use toml::map::Map;
use toml::Value;

#[derive(Deserialize)]
pub(super) struct Schema {
    pub http: Http,
    #[serde(alias = "queryparams", alias = "query-params")]
    pub query_params: Option<QueryParams>,
}

#[derive(Deserialize)]
pub(super) struct Http {
    pub method: Method,
    pub url: String,
}

#[derive(Deserialize)]
pub(super) struct QueryParams(Map<String, Value>);

impl QueryParams {
    pub fn into_pairs(self) -> Vec<(String, String)> {
        self.0
            .into_iter()
            .map(|(key, val)| (key, flatten_value(val)))
            .collect()
    }
}

fn flatten_value(val: Value) -> String {
    match val {
        Value::String(s) => s,
        Value::Datetime(_) | Value::Table(_) => todo!("return error"),
        Value::Array(a) => a
            .into_iter()
            .map(flatten_value)
            .collect::<Vec<String>>()
            .join(","),
        _ => val.to_string(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        assert_eq!(schema.http.method, Method::GET);
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
    fn missing_required() {
        let err = toml::from_str::<Schema>("").err().unwrap();
        assert!(err.message().contains("missing"));
    }

    #[test]
    fn query_params_as_pairs() {
        let string = r#"
        string = "value"
        integer = 10
        float = 2.0
        boolean = false
        array = [ "s", 10 ]
        "#;
        let pairs = toml::from_str::<QueryParams>(string).unwrap().into_pairs();

        assert_eq!(pairs.len(), 5);
        assert_pair(&pairs, "string", "value");
        assert_pair(&pairs, "integer", "10");
        assert_pair(&pairs, "float", "2.0");
        assert_pair(&pairs, "boolean", "false");
        assert_pair(&pairs, "array", "s,10");
    }

    fn assert_pair(pairs: &Vec<(String, String)>, key: &str, val: &str) {
        assert_eq!(pairs.iter().find(|(k, _)| k == key).unwrap().1, val);
    }
}
