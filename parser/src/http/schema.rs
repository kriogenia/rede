use crate::method::Method;
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Schema {
    pub http: Http,
}

#[derive(Deserialize)]
pub(super) struct Http {
    pub method: Method,
    pub url: String,
}

#[cfg(test)]
mod test {
    use super::*;

    const ALL: &str = r#"
    [http]
    method = "GET"
    url = "https://example.org/api"
    "#;

    #[test]
    fn deserialize_all() {
        let schema: Schema = toml::from_str(ALL).unwrap();
        assert_eq!(schema.http.url, "https://example.org/api");
        assert_eq!(schema.http.method, Method::GET);
    }

    #[test]
    fn missing_required() {
        let err = toml::from_str::<Schema>("").err().unwrap();
        assert!(err.message().contains("missing"));
    }
}
