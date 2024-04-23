use serde::Deserialize;

// todo: doc
#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct InputParam {
    pub hint: Option<String>,
    pub default: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize() {
        let toml = r#"
            hint = "hint"
            default = "default"
        "#;
        let input_param: InputParam = toml::from_str(toml).unwrap();
        assert_eq!(
            input_param,
            InputParam {
                hint: Some("hint".to_string()),
                default: Some("default".to_string()),
            }
        );
    }

    #[test]
    fn deserialize_empty() {
        let input_param: InputParam = toml::from_str("").unwrap();
        assert_eq!(input_param, InputParam::default());
    }
}
