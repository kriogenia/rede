//! This module holds a subset of TOML types to limit supported types of the scema

use serde::Deserialize;

/// Subset with the four primitive types
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub(crate) enum Primitive {
    Bool(bool),
    Float(f64),
    Int(i64),
    Str(String),
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

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashMap;

    #[derive(Deserialize)]
    struct Parent<T>(HashMap<String, T>);
    type PParent = Parent<Primitive>;

    #[test]
    fn primitive() {
        assert_eq!(parse(r#"primitive="string""#), "string");
        assert_eq!(parse(r#"primitive=10"#), "10");
        assert_eq!(parse(r#"primitive=5.1"#), "5.1");
        assert_eq!(parse(r#"primitive=true"#), "true");
        assert!(toml::from_str::<PParent>(r#"a=1970-01-01"#).is_err());
    }

    fn parse(str: &str) -> String {
        toml::from_str::<PParent>(str).unwrap().0["primitive"]
            .clone()
            .into()
    }
}
