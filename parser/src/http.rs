mod schema;

use crate::http::schema::Schema;
use crate::method::Method;
use std::str::FromStr;

pub struct Request {
    pub method: Method,
    pub url: String,
}

impl FromStr for Request {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let schema: Schema = toml::from_str(s).unwrap(); // todo add error management
        Ok(Self {
            method: schema.http.method,
            url: schema.http.url,
        })
    }
}
