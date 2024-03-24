use crate::schema::validation::TypeFilterFn;
use serde::Deserialize;
use std::collections::HashMap;
use toml::map::Map;
use toml::Value;

mod implementors {
    pub(crate) const METADATA: u8 = 0x00;
    pub(crate) const QUERY_PARAMS: u8 = 0x01;
}

/// Newtype implementation to wrap TOML tables where the set of keys can be free
#[derive(Deserialize)]
pub(crate) struct Table<const T: u8>(pub(crate) Map<String, Value>);

/// `metadata` table
pub(crate) type Metadata = Table<{ implementors::METADATA }>;

/// `query_params` table
pub(crate) type QueryParams = Table<{ implementors::QUERY_PARAMS }>;

impl<const T: u8> Table<T> {
    pub fn has_value(&self, filter: TypeFilterFn) -> Option<&Value> {
        self.0.values().find(filter)
    }

    fn into_pairs<O>(self, map: fn(Value) -> O) -> Vec<(String, O)> {
        self.0
            .into_iter()
            .map(|(key, val)| (key, map(val)))
            .collect()
    }
}

impl Metadata {
    pub fn into_map(self) -> HashMap<String, String> {
        self.into_pairs(flatten_value).into_iter().collect()
    }
}

impl QueryParams {
    pub fn into_param_pairs(self) -> Vec<(String, String)> {
        self.into_pairs(flatten_value)
    }
}

fn flatten_value(val: Value) -> String {
    match val {
        Value::String(s) => s,
        Value::Array(a) => a
            .into_iter()
            .map(flatten_value)
            .collect::<Vec<String>>()
            .join(","),
        Value::Datetime(_) | Value::Table(_) => {
            unreachable!("these types are rejected in from_str")
        }
        _ => val.to_string(),
    }
}

#[cfg(test)]
mod constructors {
    use super::*;
    use toml::map::Map;

    macro_rules! implement_new {
        ($alias:ident) => {
            impl $alias {
                pub fn new(table: Map<String, Value>) -> Self {
                    Self(table)
                }
            }
        };
    }

    implement_new!(Metadata);
    implement_new!(QueryParams);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn query_params_into_pairs() {
        let string = r#"
        string = "value"
        integer = 10
        float = 2.0
        boolean = false
        array = [ "s", 10 ]
        "#;
        let pairs = toml::from_str::<QueryParams>(string)
            .unwrap()
            .into_param_pairs();

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
