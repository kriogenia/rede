use std::collections::{BTreeMap, HashSet};

use http::HeaderName;
use regex::Regex;

use rede_schema::body::FormDataValue;
use rede_schema::{Body, Request};

type PlaceholdersMap = BTreeMap<String, HashSet<Location>>;

/// The `Placeholders` struct analyzes a request and extracts all the placeholders from its parts.
#[derive(Debug, Default)]
pub struct Placeholders(PlaceholdersMap);
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
                    placeholder_map.add_all(&Location::BodyForm(k.clone()), set);
                }
            }
            Body::FormData(form) => {
                for (k, v) in form {
                    let content = match v {
                        FormDataValue::Text(v) | FormDataValue::File(v) => v,
                    };
                    let set = find_placeholders(&re, content);
                    placeholder_map.add_all(&Location::BodyForm(k.clone()), set);
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

    /// Returns an iterator over the placeholders and their locations in the request
    #[must_use]
    pub fn iter(&self) -> <&PlaceholdersMap as IntoIterator>::IntoIter {
        <&Self as IntoIterator>::into_iter(self)
    }

    fn insert(&mut self, key: &str, location: Location) {
        if let Some(locations) = self.0.get_mut(key) {
            locations.insert(location);
        } else {
            #[allow(clippy::mutable_key_type)]
            let mut set = HashSet::new();
            set.insert(location);
            self.0.insert(key.to_string(), set);
        }
    }

    pub(crate) fn add_all<'a>(
        &mut self,
        location: &Location,
        keys: impl IntoIterator<Item = &'a str>,
    ) {
        for key in keys {
            self.insert(key, location.clone());
        }
    }

    #[cfg(test)]
    pub(self) fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'p> IntoIterator for &'p Placeholders {
    type Item = <&'p PlaceholdersMap as IntoIterator>::Item;
    type IntoIter = <&'p PlaceholdersMap as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

fn find_placeholders<'a>(regex: &Regex, haystack: &'a str) -> Vec<&'a str> {
    regex
        .find_iter(haystack)
        .map(|c| &c.as_str()[2..c.len() - 2])
        .collect()
}

/// Represents the part of the request where a placeholder can be present
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Location {
    Url,
    Headers(HeaderName),
    QueryParams(String),
    Body,
    BodyForm(String),
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use http::{HeaderMap, Method, Version};
    use rede_schema::Body;

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
        assert_eq!(pm.0["id"].len(), 2);
    }

    #[test]
    fn add_all() {
        let mut pm = Placeholders::new();
        pm.insert("two", Location::Url);
        let mut set = HashSet::new();
        set.insert("one");
        set.insert("two");
        pm.add_all(&Location::Headers("Header".parse().unwrap()), set);

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
            metadata: BTreeMap::default(),
            headers,
            query_params,
            variables: BTreeMap::new(),
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

            #[cfg(feature = "input_params")]
            input_params: BTreeMap::new(),
        };

        let placeholders = Placeholders::from(&request);
        assert_eq!(placeholders.len(), 5);
        assert_eq!(placeholders.0["host"].len(), 2);
        assert_eq!(placeholders.0["name"].len(), 1);
        assert_eq!(placeholders.0["genre"].len(), 2);
        assert_eq!(placeholders.0["location"].len(), 1);
        assert_eq!(placeholders.0["date"].len(), 1);

        assert_eq!(
            placeholders.0["location"].iter().next().unwrap(),
            &Location::Headers("Location".parse().unwrap())
        );
        assert_eq!(
            placeholders.0["date"].iter().next().unwrap(),
            &Location::QueryParams("release".to_string())
        );
    }
}
