#![warn(clippy::pedantic)]

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
//! use std::error::Error;
//! use http::Method;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!  let toml = r#"
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
//!  println!("{}", request.body);
//!  Ok(())
//! }
//!
//! ```
//!

pub mod body;
pub mod error;
pub mod parser;
pub mod request;

pub(crate) mod schema;

pub use parser::parse_request;
