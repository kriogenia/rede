use core::str;

use rede_schema::Request;

/// Selects a value to replace the placeholder from the request or based on it.
pub trait ValuePicker {
    /// For a given `placeholder` generates a possible value to replace
    /// in the request
    fn pick_for<'a>(req: &'a Request, placeholder: &str) -> Option<&'a String>;
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
/// # fn main() -> Result<(), Box<dyn Error>> {
/// # let toml = r#"
/// # http = { url = "http://localhost:8080", method = "GET" }
/// # variables = { name = "variable "}
/// # "#;
/// # let request = rede_parser::parse_request(toml)?;
/// assert_eq!(VariablesPicker::pick_for(&request, "name"), Some(&request.variables["name"]));
/// assert_eq!(VariablesPicker::pick_for(&request, "missing"), None);
/// # Ok(())
/// # }
/// ```
pub struct VariablesPicker;

impl ValuePicker for VariablesPicker {
    fn pick_for<'a>(req: &'a Request, placeholder: &str) -> Option<&'a String> {
        req.variables.get(placeholder)
    }
}
