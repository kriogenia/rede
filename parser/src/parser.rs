use crate::error::Error;
use crate::request::Request;
use crate::schema::Schema;
use std::str::FromStr;

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
/// assert_eq!(
///   result.err().unwrap().to_string(),
///   "params of [query_params] can't be of type datetime");
/// # }
/// ```
pub fn parse_request(content: &str) -> Result<Request, Error> {
    let schema = Schema::from_str(content)?;
    let request = Request::try_from(schema)?;
    Ok(request)
}
