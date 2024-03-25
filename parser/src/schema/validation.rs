use crate::body::Body;
use crate::error::Error;
use crate::schema::Schema;
use toml::Value;

macro_rules! validate_type {
    ($item:expr, $key:literal are not: $($type:ident),+) => {
        $(
        if let Some(value) = $item.values().find(|v| matches!(v, Value::$type(_))) {
            return Err(Error::invalid_type($key, value));
        }
        )+
    };
}

pub(super) fn validate_types(schema: &Schema) -> Result<(), Error> {
    validate_type!(&schema.metadata.0, "values of [metadata]" are not: Datetime, Array, Table);
    validate_type!(&schema.query_params.0, "values of [query_params]" are not: Datetime, Table);
    validate_type!(&schema.path_params.0, "values of [path_params]" are not: Datetime, Array, Table);
    if let Body::FormData(map) = &schema.body {
        validate_type!(map, "values of [form_data]" are not: Datetime, Table);
    }
    if let Body::XFormUrlEncoded(map) = &schema.body {
        validate_type!(map, "valus of [x-www-form-urlencoded" are not: Datetime, Table);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::schema::{QueryParams, StrStrTable};
    use toml::map::Map;

    #[test]
    fn valid_schema_types() {
        let schema = Schema {
            metadata: StrStrTable::new(map_with_base_types()),
            query_params: QueryParams::new(map_with_base_types_and_array()),
            path_params: StrStrTable::new(map_with_base_types()),
            body: Body::FormData(map_with_base_types_and_array()),
            ..Default::default()
        };
        assert!(validate_types(&schema).is_ok())
    }

    #[test]
    fn invalid_metadata_type() {
        let schema = Schema {
            metadata: StrStrTable::new(singleton_map(Value::Array(vec![]))),
            ..Default::default()
        };
        assert_eq!(
            validate_types(&schema).err().unwrap(),
            Error::InvalidType {
                field: "values of [metadata]".to_string(),
                invalid_type: "array".to_string(),
            }
        )
    }

    #[test]
    fn invalid_query_param_type() {
        let schema = Schema {
            query_params: QueryParams::new(singleton_map(Value::Table(Map::new()))),
            ..Default::default()
        };
        assert_eq!(
            validate_types(&schema).err().unwrap(),
            Error::InvalidType {
                field: "values of [query_params]".to_string(),
                invalid_type: "table".to_string(),
            }
        )
    }

    #[test]
    fn invalid_path_param_type() {
        let schema = Schema {
            path_params: StrStrTable::new(singleton_map(Value::Array(vec![]))),
            ..Default::default()
        };
        assert_eq!(
            validate_types(&schema).err().unwrap(),
            Error::InvalidType {
                field: "values of [path_params]".to_string(),
                invalid_type: "array".to_string(),
            }
        )
    }

    #[test]
    fn invalid_form_data_body() {
        let schema = Schema {
            body: Body::FormData(singleton_map(Value::Table(Map::new()))),
            ..Default::default()
        };
        assert_eq!(
            validate_types(&schema).err().unwrap(),
            Error::InvalidType {
                field: "values of [form_data]".to_string(),
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

    fn map_with_base_types_and_array() -> Map<String, Value> {
        let mut map = map_with_base_types();
        map.insert("array".to_string(), Value::Array(vec![]));
        map
    }

    fn singleton_map(value: Value) -> Map<String, Value> {
        let mut map = Map::new();
        map.insert("key".to_string(), value);
        map
    }
}
