use std::collections::{HashMap, HashSet};

use regex::Regex;

use rede_parser::Request;

/// TODO
#[derive(Debug, Default)]
pub struct Placeholders(HashMap<String, HashSet<Location>>);

impl Placeholders {
    /// Creates an empty placeholders object
    pub(crate) fn new() -> Self {
        Placeholders::default()
    }

    /// Returns an iterator with the placeholder's keys
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(String::as_str)
    }

    fn insert(&mut self, key: &str, location: Location) {
        if let Some(locations) = self.0.get_mut(key) {
            locations.insert(location);
        } else {
            let mut set = HashSet::new();
            set.insert(location);
            self.0.insert(key.to_string(), set);
        }
    }

    pub(crate) fn add_all<'a>(
        &mut self,
        location: Location,
        keys: impl IntoIterator<Item = &'a str>,
    ) {
        for key in keys {
            self.insert(key, location);
        }
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<&Request> for Placeholders {
    fn from(request: &Request) -> Self {
        let re = Regex::new(r"\{\{([A-z0-9-_.]*)}}").unwrap();

        let mut placeholder_map = Self::new();
        let set = find_placeholders(&re, &request.url);
        placeholder_map.add_all(Location::Url, set);

        for (_, v) in &request.headers {
            let set = find_placeholders(&re, v.to_str().unwrap());
            placeholder_map.add_all(Location::Headers, set); // todo store header key
        }

        for (_, v) in &request.query_params {
            let set = find_placeholders(&re, v.as_str());
            placeholder_map.add_all(Location::QueryParams, set); // todo store qp key
        }

        // todo body

        placeholder_map
    }
}

fn find_placeholders<'a>(regex: &Regex, haystack: &'a str) -> HashSet<&'a str> {
    regex
        .find_iter(haystack)
        .map(|c| &c.as_str()[2..c.len() - 2])
        .collect()
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub(crate) enum Location {
    Url,
    Headers,
    QueryParams,
    #[cfg(test)]
    Body,
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
        assert_eq!(pm.0["id"].len(), 2);
    }

    #[test]
    fn add_all() {
        let mut pm = Placeholders::new();
        pm.insert("two", Location::Url);
        let mut set = HashSet::new();
        set.insert("one");
        set.insert("two");
        pm.add_all(Location::Headers, set);

        assert_eq!(pm.len(), 2);
        assert_eq!(pm.0["one"].len(), 1);
        assert_eq!(pm.0["two"].len(), 2);
    }

    #[test]
    fn from_request() {
        let mut headers = HeaderMap::new();
        headers.insert("Host", "{{host}}".parse().unwrap());
        headers.insert("Header", "Value".parse().unwrap());

        let mut query_params = Vec::new();
        query_params.push(("genre".to_string(), "{{genre}}".to_string()));

        let request = Request {
            method: Method::GET,
            url: "{{host}}/api/game".to_string(),
            http_version: Version::HTTP_11,
            metadata: Default::default(),
            headers,
            query_params,
            variables: HashMap::new(),
            body: Body::Raw {
                content: r#"{"name":"{{name}}","genre":"{{genre}}"}"#.to_string(),
                mime: mime::APPLICATION_JSON,
            },
        };

        let placeholders = Placeholders::from(&request);
        //assert_eq!(placeholders.len(), 3);
        assert_eq!(placeholders.len(), 2);
        assert_eq!(placeholders.0["host"].len(), 2);
        //assert_eq!(placeholders.0["name"].len(), 1);
        //assert_eq!(placeholders.0["genre"].len(), 2);
        assert_eq!(placeholders.0["genre"].len(), 1);
    }
}
