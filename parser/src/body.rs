use mime::Mime;
use std::collections::HashMap;

/// Body of the request, it contains all the currently supported options
#[derive(Debug, Default, PartialEq)]
pub enum Body {
    /// The request does not have body (common for GET requests)
    #[default]
    None,
    /// The body of the request is in text format. This body can be bundled with a Content-Type
    /// like application/json to send JSONs with full meaning.
    Raw { content: String, mime: Mime },
    /// The body of the request contains a file located at the given path.
    /// This body can be bundled with Content-Type headers like application/pdf.
    Binary { path: String, mime: Mime },
    /// The body is an HTTP form.
    FormData(HashMap<String, FormDataValue>),
    /// The body of the request is an HTTP form encoded in the URL.
    XFormUrlEncoded(HashMap<String, String>),
}

#[derive(Debug, PartialEq)]
pub enum FormDataValue {
    Text(String),
    File(String),
}

impl Body {
    /// Returns the MIME type associated with the body
    ///
    /// ```
    /// # fn main() {
    /// # use thiserror::__private::AsDisplay;
    /// # use rede_parser::body::Body;
    /// # let body = Body::Binary { path: "path".to_string(), mime: mime::APPLICATION_OCTET_STREAM};
    /// assert_eq!(body.mime().map(|m| m.to_string()), Some("application/octet-stream".to_string()));
    /// # }
    /// ```
    ///
    #[must_use]
    pub fn mime(&self) -> Option<&Mime> {
        match self {
            Body::None => None,
            Body::Raw { mime, .. } | Body::Binary { mime, .. } => Some(mime),
            Body::FormData(_) => Some(&mime::MULTIPART_FORM_DATA),
            Body::XFormUrlEncoded(_) => Some(&mime::APPLICATION_WWW_FORM_URLENCODED),
        }
    }
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
