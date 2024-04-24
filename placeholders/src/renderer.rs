use crate::placeholders::Location;
use crate::Placeholders;
use rede_schema::Request;
use std::collections::HashMap;

pub struct Renderer {
    placeholders: Placeholders,
    values_map: HashMap<String, String>,
}

impl Renderer {
    /// todo doc
    #[must_use]
    pub fn new(placeholders: Placeholders, values: &[(String, String)]) -> Self {
        let values_map = values
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

        Self {
            placeholders,
            values_map,
        }
    }

    /// todo doc
    #[must_use]
    pub fn render(&self, request: Request) -> Request {
        let mut url = request.url;

        for (key, locations) in self.placeholders.iter() {
            let val = self.values_map.get(key); // maybe this could be changed into a map
            if let Some(val) = val {
                let key = format!("{{{{{key}}}}}");
                for location in locations {
                    match location {
                        Location::Url => url = url.replace(&key, val),
                        Location::Headers(_) => unimplemented!("Headers"),
                        Location::QueryParams(_) => unimplemented!("QueryParams"),
                        Location::Body => unimplemented!("Body"),
                        Location::Form(_) => unimplemented!("Form"),
                    }
                }
            }
        }

        Request {
            method: request.method,
            url,
            http_version: request.http_version,
            metadata: request.metadata,
            headers: request.headers,
            query_params: request.query_params,
            variables: request.variables,
            body: request.body,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use http::{HeaderMap, Method, Version};

    #[test]
    fn render() {
        let mut placeholders = Placeholders::default();
        placeholders.add_all(&Location::Url, vec!["id", "name"]);
        // placeholders.add_all(&Location::Headers("header".parse().unwrap()), vec!["token"]);
        // placeholders.add_all(&Location::QueryParams("query".to_string()), vec!["page"]);
        // placeholders.add_all(&Location::Body, vec!["id", "name"]);

        let values = vec![
            ("id".to_string(), "1".to_string()),
            ("name".to_string(), "test".to_string()),
            ("token".to_string(), "Bearer abc".to_string()),
            ("page".to_string(), "1".to_string()),
        ];

        let renderer = Renderer::new(placeholders, &values);

        let request = Request {
            method: Method::GET,
            url: "https://example.com/{{id}}/{{name}}/{{id}}".to_string(),
            http_version: Version::HTTP_11,
            metadata: HashMap::new(),
            headers: HeaderMap::new(),
            query_params: Vec::new(),
            variables: HashMap::new(),
            body: rede_schema::Body::None,
        };

        let rendered = renderer.render(request);

        assert_eq!(rendered.url, "https://example.com/1/test/1");
    }
}
