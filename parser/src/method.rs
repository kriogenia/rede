use serde::Deserialize;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Deserialize, PartialEq)]
/// Representation of the HTTP methods supported by Rede
pub enum Method {
    #[serde(alias = "get")]
    GET,
    #[serde(alias = "post")]
    POST,
    #[serde(alias = "put")]
    PUT,
    #[serde(alias = "patch")]
    PATCH,
    #[serde(alias = "delete")]
    DELETE,
    #[serde(alias = "head")]
    HEAD,
    #[serde(alias = "options")]
    OPTIONS,
    #[serde(alias = "connect")]
    CONNECT,
    #[serde(alias = "trace")]
    TRACE,
    /// RFC: https://httpwg.org/http-extensions/draft-ietf-httpbis-safe-method-w-body.html
    #[serde(alias = "query")]
    QUERY,
}
