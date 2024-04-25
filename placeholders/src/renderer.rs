use crate::placeholders::Location;
use crate::Placeholders;
use http::{HeaderMap, HeaderName};
use miette::{miette, Result};
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
    ///
    /// # Errors
    ///
    /// todo
    pub fn render(&self, request: Request) -> Result<Request> {
        let mut url = request.url;
        let mut headers = request.headers;

        for (key, locations) in self.placeholders.iter() {
            let val = self.values_map.get(key); // todo maybe this could be changed into a map
            if let Some(val) = val {
                let key = format!("{{{{{key}}}}}");
                for location in locations {
                    match location {
                        Location::Url => url = url.replace(&key, val),
                        Location::Headers(name) => render_headers(&mut headers, name, &key, val)?,
                        _ => { /* todo */ }
                    }
                }
            }
        }

        Ok(Request {
            method: request.method,
            url,
            http_version: request.http_version,
            metadata: request.metadata,
            headers,
            query_params: request.query_params,
            variables: request.variables,
            body: request.body,
        })
    }
}

fn render_headers(
    header_map: &mut HeaderMap,
    header: &HeaderName,
    key: &str,
    val: &str,
) -> Result<()> {
    if let Some(header_value) = header_map.get_mut(header) {
        let new_value = header_value.to_str().map_err(|_| {
            miette!("failed to convert header to string: {header} {header_value:?}")
        })?;
        let new_value = new_value.to_string().replace(key, val);
        *header_value = new_value
            .parse()
            .map_err(|_| miette!("rendered header value is invalid: {header} {new_value}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use http::{HeaderMap, Method, Version};

    #[test]
    fn render() {
        // todo replace by generated placeholders
        let mut placeholders = Placeholders::default();
        placeholders.add_all(&Location::Url, vec!["id", "name"]);
        placeholders.add_all(
            &Location::Headers("Authorization".parse().unwrap()),
            vec!["token"],
        );
        placeholders.add_all(&Location::QueryParams("query".to_string()), vec!["page"]);
        placeholders.add_all(&Location::Body, vec!["id", "name"]);

        let values = vec![
            ("id".to_string(), "1".to_string()),
            ("name".to_string(), "test".to_string()),
            ("token".to_string(), "abc".to_string()),
            ("page".to_string(), "1".to_string()),
        ];

        let renderer = Renderer::new(placeholders, &values);

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Authorization", "Bearer {{token}}".parse().unwrap());

        let request = Request {
            method: Method::GET,
            url: "https://example.com/{{id}}/{{name}}/{{id}}".to_string(),
            http_version: Version::HTTP_11,
            metadata: HashMap::new(),
            headers,
            query_params: Vec::new(),
            variables: HashMap::new(),
            body: rede_schema::Body::None,
        };

        let rendered = renderer.render(request).unwrap();

        assert_eq!(rendered.url, "https://example.com/1/test/1");
        assert_eq!(
            rendered
                .headers
                .get("Authorization")
                .unwrap()
                .to_str()
                .unwrap(),
            "Bearer abc"
        );
    }
}
