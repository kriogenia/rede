use std::collections::HashMap;

use http::HeaderName;
use regex::Regex;

use rede_parser::body::FormDataValue;
use rede_parser::{Body, Request};

/// TODO
#[derive(Debug, Default)]
pub struct Placeholders(HashMap<String, Vec<Location>>);
// todo possible improvement: store something like { url, times(3)} instead of three Location::Url
// todo possible improvement: support placeholders on Header, QueryParms and Form keys

impl From<&Request> for Placeholders {
    fn from(request: &Request) -> Self {
        let re = Regex::new(r"\{\{([A-z0-9-_.]*)}}").unwrap();

        let mut placeholder_map = Self::new();
        let set = find_placeholders(&re, &request.url);
        placeholder_map.add_all(&Location::Url, set);

        for (n, v) in &request.headers {
            let set = find_placeholders(&re, v.to_str().unwrap());
            placeholder_map.add_all(&Location::Headers(n.to_owned()), set);
        }

        for (k, v) in &request.query_params {
            let set = find_placeholders(&re, v.as_str());
            placeholder_map.add_all(&Location::QueryParams(k.clone()), set);
        }

        match &request.body {
            Body::Raw { content, .. } => {
                let set = find_placeholders(&re, content);
                placeholder_map.add_all(&Location::Body, set);
            }
            Body::Binary { path, .. } => {
                let set = find_placeholders(&re, path);
                placeholder_map.add_all(&Location::Body, set);
            }
            Body::XFormUrlEncoded(form) => {
                for (k, v) in form {
                    let set = find_placeholders(&re, v);
                    placeholder_map.add_all(&Location::Form(k.to_string()), set);
                }
            }
            Body::FormData(form) => {
                for (k, v) in form {
                    let content = match v {
                        FormDataValue::Text(v) | FormDataValue::File(v) => v,
                    };
                    let set = find_placeholders(&re, content);
                    placeholder_map.add_all(&Location::Form(k.to_string()), set);
                }
            }
            Body::None => {}
        }

        placeholder_map
    }
}

impl Placeholders {
    fn new() -> Self {
        Placeholders::default()
    }

    /// Returns an iterator with the keys of the request's placeholders.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(String::as_str)
    }

    fn insert(&mut self, key: &str, location: Location) {
        if let Some(locations) = self.0.get_mut(key) {
            locations.push(location);
        } else {
            let vec = vec![location];
            self.0.insert(key.to_string(), vec);
        }
    }

    fn add_all<'a>(&mut self, location: &Location, keys: impl IntoIterator<Item = &'a str>) {
        for key in keys {
            self.insert(key, location.clone());
        }
    }

    #[cfg(test)]
    pub(self) fn len(&self) -> usize {
        self.0.len()
    }
}

fn find_placeholders<'a>(regex: &Regex, haystack: &'a str) -> Vec<&'a str> {
    regex
        .find_iter(haystack)
        .map(|c| &c.as_str()[2..c.len() - 2])
        .collect()
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub(crate) enum Location {
    Url,
    Headers(HeaderName),
    QueryParams(String),
    Body,
    Form(String),
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use http::{HeaderMap, Method, Version};
    use rede_parser::Body;

    use super::*;

    #[test]
    fn insert() {
        let mut pm = Placeholders::new();
        pm.insert("host", Location::Url);
        pm.insert("id", Location::Url);
        pm.insert("id", Location::Body);
        pm.insert("id", Location::Body);

        assert_eq!(pm.len(), 2);
        assert_eq!(pm.0["host"].len(), 1);
        assert_eq!(pm.0["id"].len(), 3);
    }

    #[test]
    fn add_all() {
        let mut pm = Placeholders::new();
        pm.insert("two", Location::Url);
        let mut set = HashSet::new();
        set.insert("one");
        set.insert("two");
        pm.add_all(&Location::Url, set);

        assert_eq!(pm.len(), 2);
        assert_eq!(pm.0["one"].len(), 1);
        assert_eq!(pm.0["two"].len(), 2);
    }

    #[test]
    fn from_request() {
        let mut headers = HeaderMap::new();
        headers.insert("Host", "{{host}}".parse().unwrap());
        headers.insert("Location", "{{location}}".parse().unwrap());
        headers.insert("Header", "Value".parse().unwrap());

        let query_params = vec![
            ("genre".to_string(), "{{genre}}".to_string()),
            ("release".to_string(), "before:{{date}}".to_string()),
        ];

        let request = Request {
            method: Method::GET,
            url: "{{host}}/api/game".to_string(),
            http_version: Version::HTTP_11,
            metadata: HashMap::default(),
            headers,
            query_params,
            variables: HashMap::new(),
            body: Body::Raw {
                content: r#"
                {
                    "name": "{{name}}",
                    "genre": "{{genre}}",
                    "categories": [ 
                        "dreamcast",
                        "{{genre}}"
                    ]
                }"#
                .to_string(),
                mime: mime::APPLICATION_JSON,
            },
        };

        let placeholders = Placeholders::from(&request);
        assert_eq!(placeholders.len(), 5);
        assert_eq!(placeholders.0["host"].len(), 2);
        assert_eq!(placeholders.0["name"].len(), 1);
        assert_eq!(placeholders.0["genre"].len(), 3);
        assert_eq!(placeholders.0["location"].len(), 1);
        assert_eq!(placeholders.0["date"].len(), 1);

        assert_eq!(
            placeholders.0["location"][0],
            Location::Headers("Location".parse().unwrap())
        );
        assert_eq!(
            placeholders.0["date"][0],
            Location::QueryParams("release".to_string())
        );
    }
}
