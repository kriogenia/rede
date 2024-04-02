use std::collections::HashMap;

use toml::map::Map;
use toml::Value;

/// Body of the request, it contains all the currently supported options
#[derive(Debug, Default, PartialEq)]
pub enum Body {
    /// The request does not have body (common for GET requests)
    #[default]
    None,
    /// The body of the request is in text format. This body can be bundled with a Content-Type
    /// like application/json to send JSONs with full meaning.
    Raw(String),
    /// The body of the request contains a file located at the given path.
    /// This body can be bundled with Content-Type headers like application/pdf.
    Binary(String),
    /// The body is an HTTP form.
    FormData(HashMap<String, FormDataValue>),
    /// The body of the request is an HTTP form encoded in the URL.
    XFormUrlEncoded(Map<String, Value>),
}

#[derive(Debug, PartialEq)]
pub enum FormDataValue {
    Text(String),
    File(String),
}

// todo implement display for Body

/*
impl Display for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Body::{Binary, FormData, None, Raw, XFormUrlEncoded};
        match self {
            None => Ok(()),
            Raw(content) => f.write_str(content),
            Binary(path) => write!(f, "@{path}"),
            FormData(map) => writeln!(f, "{map}"),
            XFormUrlEncoded(map) => writeln!(f, "{map}"),
        }
    }
}

impl Display for FormDataValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FormDataValue::Text(value) => f.write_str(value),
            FormDataValue::File(path) => writeln!(f, "@{path}"),
        }
    }
}
*/
