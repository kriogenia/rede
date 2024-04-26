//! Library to provide functions and structures to work with `rede` placeholders.
//! It's based on an execution flow with three steps:
//! - Extract the request' [`Placeholders`].
//! - Use a [`Resolver`] with different [`ValuePicker`] to identify the values to replace each placeholder.
//! - Pass the placeholders and the resolved values to the [`Renderer`] to create a new request.
//!
//! # Example
//!
//! ```
//! # use std::error::Error;
//! # use crate::rede_placeholders::{Placeholders, Resolver, value_picker::{EnvVarPicker, VariablesPicker}};
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! # std::env::set_var("API_TOKEN", "token_from_env_var");
//! let toml = r#"
//!     http = { url = "http://localhost:8080/{{api_version}}/example/{{id}}", method = "GET" }
//!     query_params = { token = "{{API_TOKEN}}" }
//!     variables = { api_version = "v1" }
//! "#;
//! let request = rede_parser::parse_request(toml).unwrap();
//!
//! let placeholders = (&request).into();
//! let resolver = Resolver::new()
//!     .add_picker(Box::new(EnvVarPicker))
//!     .add_picker(Box::new(VariablesPicker::from(&request.variables)));
//! let ph_values = resolver.resolve(&placeholders);
//!
//! assert_eq!(ph_values.get_value("api_version"), Some(&"v1".to_string()));
//! assert_eq!(ph_values.get_value("API_TOKEN"), Some(&"token_from_env_var".to_string()));
//! assert!(ph_values.unresolved().all(|v| v == "id"));
//!
//! // TODO add renderer usage example
//! # Ok(())
//! # }
//! ```

#![warn(clippy::pedantic)]

mod placeholders;
mod renderer;
mod resolver;
pub mod value_picker;

pub use placeholders::Placeholders;
pub use renderer::Renderer;
pub use resolver::Resolver;
pub use value_picker::ValuePicker;
