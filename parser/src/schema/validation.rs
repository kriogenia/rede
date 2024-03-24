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
}

pub(crate) type TypeFilterFn = fn(&&Value) -> bool;

pub(super) fn validate_types(schema: &Schema) -> Result<(), Error> {
    validate_type!(&schema.metadata, "values of [metadata]" are not: Datetime, Array, Table);
    validate_type!(&schema.query_params, "values of [query_params]" are not: Datetime, Table);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::schema::{QueryParams, StrStrTable};
    use toml::map::Map;

    #[test]
    fn valid_schema_types() {
        let metadata = map_with_base_types();

        let mut query_params = map_with_base_types();
        query_params.insert("array".to_string(), Value::Array(vec![]));

        let mut schema = Schema::default();
        schema.metadata = Some(StrStrTable::new(metadata));
        schema.query_params = Some(QueryParams::new(query_params));
        assert!(validate_types(&schema).is_ok())
    }

    #[test]
    fn invalid_metadata_type() {
        let mut metadata = Map::new();
        metadata.insert("array".to_string(), Value::Array(vec![]));

        let mut schema = Schema::default();
        schema.metadata = Some(StrStrTable::new(metadata));
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

        let mut schema = Schema::default();
        schema.query_params = Some(QueryParams::new(query_params));
        assert_eq!(
            validate_types(&schema).err().unwrap(),
            Error::InvalidType {
                field: "values of [query_params]".to_string(),
                invalid_type: "table".to_string(),
            }
        )
    }

    fn map_with_base_types() -> Map<String, Value> {
        let mut map = Map::new();
        map.insert("string".to_string(), Value::String("valid".to_string()));
        map.insert("integer".to_string(), Value::Integer(0));
        map.insert("float".to_string(), Value::Float(0.1));
        map.insert("boolean".to_string(), Value::Boolean(true));
        map
    }
}
