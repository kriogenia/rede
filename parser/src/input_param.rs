use serde::Deserialize;

/// Contains the different properties that can be defined for an input parameter.
#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct InputParam {
    /// Hint to provide to the user when asking for the input
    pub hint: Option<String>,
    /// Default value to use if the user does not provide any input
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
