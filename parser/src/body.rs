use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Body {
    #[default]
    None,
    #[serde(alias = "text")]
    Raw(String),
    #[serde(alias = "file")]
    Binary(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Deserialize)]
    struct Parent {
        body: Body,
    }

    #[test]
    fn deserialize() {
        let toml = r#"body.raw = "content""#;
        let parent: Parent = toml::from_str(toml).unwrap();
        assert_eq!(parent.body, Body::Raw("content".to_string()));
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
}
