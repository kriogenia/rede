use rede_schema::Request;
use std::env;

/// Selects a value to replace the placeholder from the request or based on it.
pub trait ValuePicker {
    /// For a given `placeholder` generates a possible value to replace
    /// in the request
    fn pick_for(&self, req: &Request, placeholder: &str) -> Option<String>;
}

/// Picks a value from the request variables
///
/// # Example
///
/// ```
/// # use http::Method;
/// # use rede_schema::body::Body;
/// # use std::error::Error;
/// # use crate::rede_placeholders::{ValuePicker, value_picker::VariablesPicker};
/// #
/// # let toml = r#"
/// # http = { url = "http://localhost:8080", method = "GET" }
/// # variables = { name = "variable "}
/// # "#;
/// # let request = rede_parser::parse_request(toml).unwrap();
/// assert_eq!(VariablesPicker.pick_for(&request, "name").unwrap(), request.variables["name"]);
/// assert_eq!(VariablesPicker.pick_for(&request, "missing"), None);
/// ```
pub struct VariablesPicker;

impl ValuePicker for VariablesPicker {
    fn pick_for(&self, req: &Request, placeholder: &str) -> Option<String> {
        req.variables.get(placeholder).map(String::to_owned)
    }
}

/// Picks a value from the environment variables
///
/// # Example
///
/// ```
/// # use http::Method;
/// # use rede_schema::body::Body;
/// # use std::error::Error;
/// # use crate::rede_placeholders::{ValuePicker, value_picker::EnvVarPicker};
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// # let toml = r#"
/// # http = { url = "http://localhost:8080", method = "GET" }
/// # "#;
/// # let request = rede_parser::parse_request(toml)?;
/// std::env::set_var("envvar", "value");
/// assert_eq!(EnvVarPicker.pick_for(&request, "envvar"), Some("value".to_string()));
/// assert_eq!(EnvVarPicker.pick_for(&request, "missing"), None);
/// # Ok(())
/// # }
/// ```
pub struct EnvVarPicker;

impl ValuePicker for EnvVarPicker {
    fn pick_for(&self, _request: &Request, placeholder: &str) -> Option<String> {
        env::var(placeholder).ok()
    }
}
