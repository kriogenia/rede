//! This module holds a subset of TOML types to limit supported types of the scema

use std::vec::IntoIter;

use serde::Deserialize;

/// Subset with the four primitive types
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub(crate) enum Primitive {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
}

/// Subset with the four primitive types and the array of those
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub(crate) enum PrimitiveArray {
    Single(Primitive),
    Multiple(Vec<Primitive>),
}

impl From<Primitive> for String {
    fn from(value: Primitive) -> Self {
        match value {
            Primitive::Str(s) => s,
            Primitive::Bool(b) => b.to_string(),
            Primitive::Float(f) => f.to_string(),
            Primitive::Int(i) => i.to_string(),
        }
    }
}

impl From<PrimitiveArray> for String {
    fn from(value: PrimitiveArray) -> Self {
        match value {
            PrimitiveArray::Single(p) => p.into(),
            PrimitiveArray::Multiple(a) => a
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>()
                .join(","),
        }
    }
}

impl IntoIterator for PrimitiveArray {
    type Item = Primitive;
    type IntoIter = IntoIter<Primitive>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            PrimitiveArray::Single(v) => vec![v],
            PrimitiveArray::Multiple(vec) => vec,
        }
        .into_iter()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[derive(Deserialize)]
    struct Parent<T>(HashMap<String, T>);
    type Pri = Parent<Primitive>;
    type PriArr = Parent<PrimitiveArray>;

    #[test]
    fn primitive() {
        assert_eq!(parse_pri(r#"primitive="string""#), "string");
        assert_eq!(parse_pri("primitive=10"), "10");
        assert_eq!(parse_pri("primitive=5.1"), "5.1");
        assert_eq!(parse_pri("primitive=true"), "true");
        assert!(toml::from_str::<Pri>("a=1970-01-01").is_err());
    }

    #[test]
    fn primitive_array() {
        assert_eq!(parse_pri_arr(r#"primitive="string""#), "string");
        assert_eq!(parse_pri_arr("primitive=10"), "10");
        assert_eq!(parse_pri_arr("primitive=5.1"), "5.1");
        assert_eq!(parse_pri_arr("primitive=true"), "true");
        assert_eq!(parse_pri_arr(r#"primitive=[1,"two"]"#), "1,two");
        assert!(toml::from_str::<PriArr>("a=1970-01-01").is_err());
    }

    fn parse_pri(str: &str) -> String {
        toml::from_str::<Pri>(str).unwrap().0["primitive"]
            .clone()
            .into()
    }

    fn parse_pri_arr(str: &str) -> String {
        toml::from_str::<PriArr>(str).unwrap().0["primitive"]
            .clone()
            .into()
    }
}
