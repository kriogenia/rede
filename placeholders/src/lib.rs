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
//! # use crate::rede_placeholders::{Placeholders, Renderer, Resolver, value_picker::{EnvVarPicker, VariablesPicker}};
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
//! // find placeholders
//! let placeholders = (&request).into();
//! // identify values, as the resolver uses the variables picker, it should be dropped before the render step
//! let ph_values = {
//!     let resolver = Resolver::new()
//!         .add_picker(Box::new(EnvVarPicker))
//!         .add_picker(Box::new(VariablesPicker::new(&request.variables)));
//!     resolver.resolve(&placeholders)
//! };
//! // render new request
//! let renderer = Renderer::new(&placeholders, ph_values);
//! let rendered = renderer.render(request)?;
//!
//! assert_eq!(rendered.url, "http://localhost:8080/v1/example/{{id}}");
//! assert_eq!(rendered.query_params[0].1, "token_from_env_var".to_string());
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
pub use resolver::PlaceholderValues;
pub use resolver::Resolver;
pub use value_picker::ValuePicker;
