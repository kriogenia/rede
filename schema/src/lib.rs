//! Library containing the structs used in the `rede` crate.
//!
//! # Features
//!
//! - `input_params`

#![warn(clippy::pedantic)]

/// Contains all the specific types used in the body
pub mod body;

#[cfg(feature = "input_params")]
mod input_param;

#[doc(inline)]
pub use body::Body;

#[cfg(feature = "input_params")]
#[doc(inline)]
pub use input_param::InputParam;

use std::collections::HashMap;

use http::{HeaderMap, Method, Version};

/// Representation of a rede HTTP request. Contains all the supported content by the current schema
/// to allow the creation and dispatching of the HTTP request with the command-line interface.
#[derive(Debug)]
pub struct Request {
    /// HTTP method of the request
    pub method: Method,
    /// URL of the request
    pub url: String,
    /// HTTP version of the request
    pub http_version: Version,
    /// Metadata of the request file
    pub metadata: HashMap<String, String>,
    /// Headers of the request
    pub headers: HeaderMap,
    /// Query parameters of the request
    pub query_params: Vec<(String, String)>,
    /// Body of the request
    pub body: Body,
    /// Variables to provide values for placeholders in the request
    pub variables: HashMap<String, String>,

    #[cfg(feature = "input_params")]
    /// Keys of placeholders to ask the user for input
    pub input_params: HashMap<String, InputParam>,
}
