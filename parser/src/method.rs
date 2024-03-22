use strum::EnumString;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, EnumString, PartialEq)]
#[strum(ascii_case_insensitive)]
/// Representation of the HTTP methods supported by Rede
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    /// RFC: https://httpwg.org/http-extensions/draft-ietf-httpbis-safe-method-w-body.html
    QUERY,
}

#[cfg(test)]
mod test {
    use super::*;
    use strum::ParseError;

    #[test]
    fn from_str() {
        assert_eq!(Method::try_from("GET").unwrap(), Method::GET);
        assert_eq!(Method::try_from("post").unwrap(), Method::POST);
        assert_eq!(Method::try_from("pUt").unwrap(), Method::PUT);
        assert_eq!(
            Method::try_from("unknown"),
            Err(ParseError::VariantNotFound)
        )
    }
}
