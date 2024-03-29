use serde::Deserialize;
use std::fmt::{Display, Formatter};
use toml::map::Map;
use toml::Value;

/// Body of the request, it contains all the currently supported options
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Body {
    /// The request does not have body (common for GET requests)
    #[default]
    None,
    /// The body of the request is in text format. This body can be bundled with a Content-Type
    /// like application/json to send JSONs with full meaning.
    #[serde(alias = "text")]
    Raw(String),
    /// The body of the request contains a file located at the given path.
    /// This body can be bundled with Content-Type headers like application/pdf.
    #[serde(alias = "file")]
    Binary(String),
    /// The body is an HTTP form. // TODO check this for better message
    #[serde(alias = "form-data", alias = "formdata")]
    FormData(Map<String, Value>),
    /// The body of the request is an HTTP form encoded in the URL.
    #[serde(
        alias = "x-www-form-urlencoded",
        alias = "form_urlencoded",
        alias = "form-urlencoded"
    )]
    XFormUrlEncoded(Map<String, Value>),
}

impl Display for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Body::{Binary, FormData, None, Raw, XFormUrlEncoded};
        match self {
            None => Ok(()),
            Raw(content) => f.write_str(content),
            Binary(path) => write!(f, "@{path}"),
            FormData(map) | XFormUrlEncoded(map) => writeln!(f, "{map}"),
        }
    }
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
