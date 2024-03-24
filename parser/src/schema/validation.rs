use crate::error::Error;
use crate::schema::Schema;
use toml::Value;

macro_rules! has_invalid_type {
    ($key:literal, $item:expr, no $($type:ident),+) => {
        $(
            if let Some(value) = $item.has_value(|v| matches!(v, Value::$type(_))) {
                return Err(Error::invalid_type($key, value));
            }
        )+
    };
}

pub(super) fn validate_types(schema: &Schema) -> Result<(), Error> {
    if let Some(qp) = &schema.query_params {
        has_invalid_type!("params of [query_params]", qp, no Datetime, Table);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::schema::QueryParams;
    use toml::map::Map;

    #[test]
    fn valid_schema_types() {
        let mut query_params = Map::new();
        query_params.insert("string".to_string(), Value::String("valid".to_string()));
        query_params.insert("integer".to_string(), Value::Integer(0));
        query_params.insert("float".to_string(), Value::Float(0.1));
        query_params.insert("boolean".to_string(), Value::Boolean(true));
        query_params.insert("array".to_string(), Value::Array(vec![]));

        let mut schema = Schema::new();
        schema.query_params = Some(QueryParams(query_params));
        assert!(validate_types(&schema).is_ok())
    }

    #[test]
    fn invalid_schema_type() {
        let mut query_params = Map::new();
        query_params.insert("table".to_string(), Value::Table(Map::new()));

        let mut schema = Schema::new();
        schema.query_params = Some(QueryParams(query_params));
        assert_eq!(
            validate_types(&schema).err().unwrap(),
            Error::InvalidType {
                field: "params of [query_params]".to_string(),
                invalid_type: "table".to_string(),
            }
        )
    }
}
