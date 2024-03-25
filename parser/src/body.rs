use serde::Deserialize;
use toml::map::Map;
use toml::Value;

#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Body {
    #[default]
    None,
    #[serde(alias = "text")]
    Raw(String),
    #[serde(alias = "file")]
    Binary(String),
    #[serde(alias = "form-data", alias = "formdata")]
    FormData(Map<String, Value>),
    #[serde(
        alias = "x-www-form-urlencoded",
        alias = "form_urlencoded",
        alias = "form-urlencoded"
    )]
    XFormUrlEncoded(Map<String, Value>),
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct Parent {
        body: Body,
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            toml::from_str::<Parent>(r#"body.raw = "content""#)
                .unwrap()
                .body,
            Body::Raw("content".to_string())
        );
        let toml = r#"body.form-data = { type = "integer", value = 1 }"#;
        let body = toml::from_str::<Parent>(toml).unwrap().body;
        assert!(matches!(&body, Body::FormData(map) if map.len() == 2));
        if let Body::FormData(map) = &body {
            assert_eq!(map["type"], Value::String("integer".to_string()));
            assert_eq!(map["value"], Value::Integer(1));
        }
    }

    #[test]
    fn only_one_type() {
        let toml = r#"
        [body]
        raw = "raw"
        binary = "file"
        "#;
        let err = toml::from_str::<Parent>(toml).err().unwrap();
        assert!(err.to_string().contains("wanted exactly 1 element"));
    }

    #[test]
    fn deserializae_empty() {
        let err = toml::from_str::<Parent>(r#"[body]"#).err().unwrap();
        assert!(err.to_string().contains("wanted exactly 1 element"));
    }
}
