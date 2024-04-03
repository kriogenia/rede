//! Library to handle the parsing of requests in TOML format used by the crate `rede`.
//!
//! The library offers the function [`rede_parser::parse_request`] to convert a given string into
//! a valid [`rede_parser::request::Request`].
//!
//! # Example
//!
//! Pass the correct request TOML to the parser functions to get the parsed request:
//!
//! ```
//! # use http::Method;
//! # use rede_parser::body::Body;
//! # use std::error::Error;
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let toml = r#"
//!     [http]
//!     method = "POST"
//!     url = "http://localhost:8080/note"
//!
//!     [headers]
//!     Content-Type = "application/json"
//!
//!     [body]
//!     raw = """
//!     {
//!         "title": "Implement rede_parser" ,
//!         "description": "Implement it following the example
//!     }
//!     """
//!  "#;
//!  let request = rede_parser::parse_request(toml)?;
//!  assert_eq!(request.method, Method::POST);
//!  assert_eq!(request.url, "http://localhost:8080/note");
//!  assert_eq!(request.headers["Content-Type"], "application/json");
//!  if let Body::Raw { content, mime } = &request.body {
//!     assert_eq!(mime, &"text/plain; charset=utf-8");
//!     println!("{}", &request.body);
//!  }
//!  # Ok(())
//! # }
//!
//! ```

#![warn(clippy::pedantic)]

pub mod body;
pub mod error;
pub mod parser;
pub mod request;

pub(crate) mod schema;

pub use parser::parse_request;
