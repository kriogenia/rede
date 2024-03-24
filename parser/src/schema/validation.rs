use crate::error::Error;
use crate::schema::Schema;
use toml::Value;

macro_rules! validate_type {
    ($item:expr, $key:literal are not: $($type:ident),+) => {
        $(
            if let Some(table) = $item {
                if let Some(value) = table.has_value(|v| matches!(v, Value::$type(_))) {
                    return Err(Error::invalid_type($key, value));
                }
            }
        )+
    };
    ($item:expr, $key:literal are: $($type:ident),+) => {
        $(
            if let Some(table) = $item {
                if let Some(value) = table.has_value(|v| !matches!(v, Value::$type(_))) {
                    return Err(Error::invalid_type($key, value));
                }
            }
        )+
    };
}

pub(crate) type TypeFilterFn = fn(&&Value) -> bool;

pub(super) fn validate_types(schema: &Schema) -> Result<(), Error> {
    validate_type!(&schema.metadata, "values of [metadata]" are: String);
    validate_type!(&schema.query_params, "values of [query_params]" are not: Datetime, Table);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::schema::{Metadata, QueryParams};
    use toml::map::Map;

    #[test]
    fn valid_schema_types() {
        let mut metadata = Map::new();
        metadata.insert("key".to_string(), Value::String("value".to_string()));
        metadata.insert("other".to_string(), Value::String("string".to_string()));

        let mut query_params = Map::new();
        query_params.insert("string".to_string(), Value::String("valid".to_string()));
        query_params.insert("integer".to_string(), Value::Integer(0));
        query_params.insert("float".to_string(), Value::Float(0.1));
        query_params.insert("boolean".to_string(), Value::Boolean(true));
        query_params.insert("array".to_string(), Value::Array(vec![]));

        let mut schema = Schema::new();
        schema.metadata = Some(Metadata::new(metadata));
        schema.query_params = Some(QueryParams::new(query_params));
        assert!(validate_types(&schema).is_ok())
    }

    #[test]
    fn invalid_metadata_type() {
        let mut metadata = Map::new();
        metadata.insert("array".to_string(), Value::Array(vec![]));

        let mut schema = Schema::new();
        schema.metadata = Some(Metadata::new(metadata));
        assert_eq!(
            validate_types(&schema).err().unwrap(),
            Error::InvalidType {
                field: "values of [metadata]".to_string(),
                invalid_type: "array".to_string(),
            }
        )
    }

    #[test]
    fn invalid_schema_type() {
        let mut query_params = Map::new();
        query_params.insert("table".to_string(), Value::Table(Map::new()));

        let mut schema = Schema::new();
        schema.query_params = Some(QueryParams::new(query_params));
        assert_eq!(
            validate_types(&schema).err().unwrap(),
            Error::InvalidType {
                field: "values of [query_params]".to_string(),
                invalid_type: "table".to_string(),
            }
        )
    }
}
