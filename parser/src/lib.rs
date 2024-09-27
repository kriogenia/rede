//! Library to handle the parsing of requests in TOML format used by the crate `rede`.
//!
//! The library offers the function [`rede_parser::parse_request`](parse_request)
//! to convert a given string into a valid [`rede_parser::Request`](Request).
//!
//! # Example
//!
//! Pass the correct request TOML to the parser functions to get the parsed request:
//!
//! ```
//! # use http::Method;
//! # use rede_schema::body::Body;
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
//! ```

#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod error;
mod request;
mod schema;

#[cfg(feature = "input_params")]
mod input_param;

use crate::schema::Schema;
use std::str::FromStr;

#[doc(inline)]
pub use error::Error;
#[doc(inline)]
pub use request::Request;

#[cfg(feature = "input_params")]
#[doc(inline)]
pub use input_param::InputParam;

/// Attempts to parse the given string into an HTTP request.
///
/// # Example
///
/// Passing the contents of a valid request TOML will provide a [Request]
///
/// ```
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let toml = r#"
///  http = { url = "http://localhost:8080", method = "GET" }
/// "#;
/// let request = rede_parser::parse_request(toml)?;
/// assert_eq!(request.url, "http://localhost:8080");
/// assert_eq!(request.method, "GET");
/// #    Ok(())
/// # }
///```
/// # Errors
///
/// Some possible errors are:
/// - The contents are not a valid TOML file
/// - A required key is missing
/// - At least one is the wrong type
///
/// ```
/// # use std::error::Error;
/// # fn main() {
/// let toml = r#"
///  http = { url = "http://localhost:8080", method = "GET" }
///  query_params = { since = 1970-01-01 }
/// "#;
/// let result = rede_parser::parse_request(toml);
/// assert!(result.is_err());
/// # }
/// ```
pub fn parse_request(content: &str) -> Result<Request, Error> {
    let schema = Schema::from_str(content)?;
    let request = Request::try_from(schema)?;
    Ok(request)
}
