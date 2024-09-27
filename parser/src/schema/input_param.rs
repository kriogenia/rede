use rede_schema::InputParam as SchemaIP;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq)]
pub(crate) struct InputParam {
    pub(crate) hint: Option<String>,
}

impl From<InputParam> for SchemaIP {
    fn from(value: InputParam) -> Self {
        Self { hint: value.hint }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize() {
        let toml = r#"
            hint = "hint"
        "#;
        let input_param: InputParam = toml::from_str(toml).unwrap();
        assert_eq!(
            input_param,
            InputParam {
                hint: Some("hint".to_string()),
            }
        );
    }

    #[test]
    fn deserialize_empty() {
        let input_param: InputParam = toml::from_str("").unwrap();
        assert_eq!(input_param, InputParam::default());
    }
}
