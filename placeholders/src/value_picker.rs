//! Module containing the [`ValuePicker`] trait and some pickers provided by the
//! crate. In case that some custom picker is required, the trait can be implemented
//! and used in the [`Resolver`](crate::Resolver)
//! like those provided.

use std::{collections::HashMap, env};

/// Selects a value to resolve the placeholder.
pub trait ValuePicker {
    /// For a given placeholder generates a possible value to replace in the request
    fn pick_for(&self, placeholder: &str) -> Option<String>;
}

/// Picks a value from the request variables.
///
/// > WARNING_: using this picker with the [`Request::variables`](rede_schema::Request::variables)
/// will generate a compilation error in the `render` step. Drop the picker before rendering,
/// check the library root documentation for an example.
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use crate::rede_placeholders::{ValuePicker, value_picker::VariablesPicker};
/// #
/// let toml = r#"
/// http = { url = "http://localhost:8080", method = "GET" }
/// variables = { name = "variable" }
/// "#;
/// let request = rede_parser::parse_request(toml).unwrap();
/// let picker = VariablesPicker::from(&request.variables);
/// assert_eq!(picker.pick_for("name"), Some("variable".to_string()));
/// assert_eq!(picker.pick_for("missing"), None);
/// ```
pub struct VariablesPicker<'var> {
    vars: &'var HashMap<String, String>,
}

impl<'var> VariablesPicker<'var> {
    // Generates a picker surrounding the given dictionary of keys and values. Commonly used to
    // be instantiated with the [`Request`](rede_schema::Request)'s `variables` field.
    #[must_use]
    pub fn from(vars: &'var HashMap<String, String>) -> Self {
        Self { vars }
    }
}

impl<'r> ValuePicker for VariablesPicker<'r> {
    fn pick_for(&self, placeholder: &str) -> Option<String> {
        self.vars.get(placeholder).map(String::to_owned)
    }
}

/// Picks a value from the environment variables
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use crate::rede_placeholders::{ValuePicker, value_picker::EnvVarPicker};
/// #
/// std::env::set_var("envvar", "value");
/// assert_eq!(EnvVarPicker.pick_for("envvar"), Some("value".to_string()));
/// assert_eq!(EnvVarPicker.pick_for("missing"), None);
/// ```
pub struct EnvVarPicker;

impl ValuePicker for EnvVarPicker {
    fn pick_for(&self, placeholder: &str) -> Option<String> {
        env::var(placeholder).ok()
    }
}
