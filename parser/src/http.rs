mod schema;

use crate::error::Error;
use crate::http::schema::{QueryParams, Schema};
use crate::method::Method;
use std::str::FromStr;

pub struct Request {
    pub method: Method,
    pub url: String,
    pub query_params: Vec<(String, String)>,
}

impl FromStr for Request {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let schema: Schema = Schema::from_str(s)?;
        let query_params = schema
            .query_params
            .map(QueryParams::into_pairs)
            .unwrap_or_default();
        Ok(Self {
            method: schema.http.method,
            url: schema.http.url,
            query_params,
        })
    }
}
