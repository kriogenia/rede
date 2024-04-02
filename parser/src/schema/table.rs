use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::ops::Index;

use crate::body::FormDataValue as PublicFDValue;
use crate::schema::body::FormDataValue;
use serde::Deserialize;
use toml::Value;

/// Newtype implementation to wrap TOML tables where the set of keys can be free
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Table<V>(pub(crate) HashMap<String, V>);

impl<V> Index<&str> for Table<V> {
    type Output = V;

    fn index(&self, index: &str) -> &Self::Output {
        &self.0[index]
    }
}

impl<V> IntoIterator for Table<V> {
    type Item = (String, V);
    type IntoIter = IntoIter<String, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<V> Default for Table<V> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

pub trait Transform<V, O>: IntoIterator<Item = (String, V)>
where
    Self: Sized,
{
    fn map_value(value: V) -> O;

    fn into_pairs(self) -> Vec<(String, O)> {
        self.into_iter()
            .map(|(k, v)| (k, Self::map_value(v)))
            .collect()
    }

    fn into_map(self) -> HashMap<String, O> {
        self.into_pairs().into_iter().collect()
    }
}

impl Transform<Value, String> for Table<Value> {
    fn map_value(value: Value) -> String {
        flatten_value(value)
    }
}

impl Transform<FormDataValue, PublicFDValue> for Table<FormDataValue> {
    fn map_value(value: FormDataValue) -> PublicFDValue {
        match value {
            FormDataValue::Text(value) => PublicFDValue::Text(flatten_value(value)),
            FormDataValue::File(path) => PublicFDValue::File(path),
        }
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
impl<V> Table<V> {
    pub fn new(table: HashMap<String, V>) -> Self {
        Self(table)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn into_pairs() {
        let pairs = new_test_table().into_pairs();

        assert_eq!(pairs.len(), 5);
        assert_pair(&pairs, "string", "value");
        assert_pair(&pairs, "integer", "10");
        assert_pair(&pairs, "float", "2.0");
        assert_pair(&pairs, "boolean", "false");
        assert_pair(&pairs, "array", "s,10");
    }

    #[test]
    fn into_map() {
        let map = new_test_table().into_map();

        assert_eq!(map.len(), 5);
        assert_eq!(map["string"], "value");
        assert_eq!(map["integer"], "10");
        assert_eq!(map["float"], "2.0");
        assert_eq!(map["boolean"], "false");
        assert_eq!(map["array"], "s,10");
    }

    fn new_test_table() -> Table<Value> {
        let string = r#"
        string = "value"
        integer = 10
        float = 2.0
        boolean = false
        array = [ "s", 10 ]
        "#;
        toml::from_str(string).unwrap()
    }

    fn assert_pair(pairs: &Vec<(String, String)>, key: &str, val: &str) {
        assert_eq!(pairs.iter().find(|(k, _)| k == key).unwrap().1, val);
    }
}
