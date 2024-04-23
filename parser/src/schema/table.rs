use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::ops::Index;

use crate::schema::body::FormDataValue;
use crate::schema::types::PrimitiveArray;
use serde::Deserialize;

#[cfg(feature = "input_params")]
use crate::InputParam;

/// Newtype implementation to wrap TOML tables where the set of keys can be free
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Table<V>(pub(crate) HashMap<String, V>);

pub type PrimitiveTable = Table<PrimitiveArray>;
pub type FormDataTable = Table<FormDataValue>;

#[cfg(feature = "input_params")]
pub type InputParamsTable = Table<InputParam>;

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

    fn into_map(self) -> HashMap<String, O> {
        self.into_iter()
            .map(|(k, v)| (k, Self::map_value(v)))
            .collect()
    }
}

impl<V, O> Transform<V, O> for Table<V>
where
    V: Into<O>,
{
    fn map_value(value: V) -> O {
        value.into()
    }
}

impl PrimitiveTable {
    pub(crate) fn into_pairs(self) -> Vec<(String, String)> {
        let mut vec = Vec::new();
        for (key, val) in self {
            val.into_iter()
                .map(String::from)
                .for_each(|v| vec.push((key.clone(), v)));
        }
        vec
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

        assert_eq!(pairs.len(), 6);
        assert_pair(&pairs, "string", "value");
        assert_pair(&pairs, "integer", "10");
        assert_pair(&pairs, "float", "2.1");
        assert_pair(&pairs, "boolean", "false");
    }

    #[test]
    fn into_map() {
        let map: HashMap<String, String> = new_test_table().into_map();

        assert_eq!(map.len(), 5);
        assert_eq!(map["string"], "value");
        assert_eq!(map["integer"], "10");
        assert_eq!(map["float"], "2.1");
        assert_eq!(map["boolean"], "false");
        assert_eq!(map["array"], "one,2");
    }

    fn new_test_table() -> PrimitiveTable {
        let string = r#"
        string = "value"
        integer = 10
        float = 2.1
        boolean = false
        array = [ "one", 2 ]
        "#;
        toml::from_str(string).unwrap()
    }

    fn assert_pair(pairs: &Vec<(String, String)>, key: &str, val: &str) {
        assert_eq!(pairs.iter().find(|(k, _)| k == key).unwrap().1, val);
    }
}
