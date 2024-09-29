use std::collections::HashMap;

use http::{HeaderMap, HeaderName};
use miette::{miette, Result};
use rede_schema::body::FormDataValue;
use rede_schema::{Body, Request};

use crate::placeholders::Location;
use crate::resolver::PlaceholderValues;
use crate::Placeholders;

/// A renderer is responsible for rendering a request using the placeholders and values provided.
/// It should be used with [`render`](Renderer::render) to replace the placeholders with the values in the
/// request.
pub struct Renderer<'ph> {
    placeholders: &'ph Placeholders,
    values: PlaceholderValues<'ph>,
}

macro_rules! replace_pointer {
    ($pointer:expr, $placeholder:expr, $value:expr) => {
        let new_value = $pointer.replace($placeholder, $value);
        *$pointer = new_value;
    };
}

impl<'ph> Renderer<'ph> {
    /// Creates a new instance of a `Renderer` that will be able to render request using the given
    /// placeholders and values.
    #[must_use]
    pub fn new(placeholders: &'ph Placeholders, values: PlaceholderValues<'ph>) -> Self {
        Self {
            placeholders,
            values,
        }
    }

    /// Renders the given request using the placeholders and values of the `Renderer`. The renderer
    /// will iterate through all the placeholders in the request and use them to search and replace
    /// the request part with the map of values.
    ///
    /// In case that any of the placeholder keys is unresolved the render operation won't fail.
    /// It will continue without replacing the placeholder. This is by design to allow the option
    /// of chaining multiple renderings to make multistep replacements that could help to, for
    /// example, enable in the future the option for nested placeholders.
    ///
    /// # Errors
    ///
    /// If the renderer fails to render the request, it will return an error. This can happen, for
    /// example if the generated header value breaks the HTTP specification.
    pub fn render(&self, request: Request) -> Result<Request> {
        let mut url = request.url;
        let mut headers = request.headers;
        let mut query_params = request.query_params;
        let mut body = request.body;

        for (key, locations) in self.placeholders.iter() {
            let val = self.values.get_value(key); // todo this could be changed into a map operation
            if let Some(val) = val {
                let placeholder = format!("{{{{{key}}}}}");
                for location in locations {
                    match location {
                        Location::Url => url = url.replace(&placeholder, val),
                        Location::Headers(name) => {
                            render_headers(&mut headers, name, &placeholder, val)?;
                        }
                        Location::QueryParams(key) => {
                            if let Some((_, v)) = query_params.iter_mut().find(|(k, _)| k == key) {
                                replace_pointer!(v, &placeholder, val);
                            }
                        }
                        Location::BodyForm(k) => match &mut body {
                            Body::FormData(form) => {
                                render_form_data(form, k, &placeholder, val);
                            }
                            Body::XFormUrlEncoded(form) => {
                                render_form_urlencoded(form, k, &placeholder, val);
                            }
                            _ => {}
                        },
                        Location::Body => {
                            if let Body::Raw { content, .. } | Body::Binary { path: content, .. } =
                                &mut body
                            {
                                replace_pointer!(content, &placeholder, val);
                            }
                        }
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
            query_params,
            variables: request.variables,
            body,
        })
    }
}

fn render_headers(
    header_map: &mut HeaderMap,
    header: &HeaderName,
    placeholder: &str,
    val: &str,
) -> Result<()> {
    if let Some(header_value) = header_map.get_mut(header) {
        let new_value = header_value.to_str().map_err(|_| {
            miette!("failed to convert header to string: {header} {header_value:?}")
        })?;
        let new_value = new_value.to_string().replace(placeholder, val);
        *header_value = new_value
            .parse()
            .map_err(|_| miette!("rendered header value is invalid: {header} {new_value}"))?;
    }
    Ok(())
}

fn render_form_data(
    form: &mut HashMap<String, FormDataValue>,
    key: &str,
    placeholder: &str,
    val: &str,
) {
    if let Some(FormDataValue::Text(v) | FormDataValue::File(v)) = form.get_mut(key) {
        replace_pointer!(v, placeholder, val);
    }
}

fn render_form_urlencoded(
    form: &mut HashMap<String, String>,
    key: &str,
    placeholder: &str,
    val: &str,
) {
    if let Some(v) = form.get_mut(key) {
        replace_pointer!(v, placeholder, val);
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use super::*;

    #[test]
    fn render() -> std::result::Result<(), Box<dyn Error>> {
        let request = r#"
        [http]
        url = "https://example.com/{{id}}/{{name}}/{{id}}"

        [headers]
        Content-Type = "application/json"
        Authorization = "Bearer {{token}}"

        [query_params]
        page = "{{page}}"
        size = "{{size}}"

        [body]
        raw = """
        {
            "id": {{id}},
            "name": "{{name}} {{last_name}}",
            "tag": "{{NOT_REPLACED}}"
        }
        """
        "#;

        let request = rede_parser::parse_request(request).unwrap();
        let placeholders = (&request).into();

        let values = vec![
            ("id", "1".to_string()),
            ("name", "test".to_string()),
            ("token", "abc".to_string()),
            ("page", "1".to_string()),
            ("size", "10".to_string()),
            ("last_name", "renderer".to_string()),
        ]
        .into_iter()
        .map(|(k, v)| (k, Some(v)))
        .collect();
        let values = PlaceholderValues { values };

        let renderer = Renderer::new(&placeholders, values);
        let rendered = renderer.render(request).unwrap();

        assert_eq!(rendered.url, "https://example.com/1/test/1");
        assert_eq!(rendered.headers["Authorization"].to_str()?, "Bearer abc");
        assert_eq!(
            rendered
                .query_params
                .iter()
                .find(|(k, _)| k == "page")
                .unwrap()
                .1,
            "1"
        );
        assert_eq!(
            rendered
                .query_params
                .iter()
                .find(|(k, _)| k == "size")
                .unwrap()
                .1,
            "10"
        );
        if let Body::Raw { content, .. } = rendered.body {
            println!("{}", content);
            assert!(content.contains(r#""id": 1"#));
            assert!(content.contains(r#""name": "test renderer""#));
            assert!(content.contains("{{NOT_REPLACED}}"));
        } else {
            panic!("body is not raw")
        }
        Ok(())
    }

    #[test]
    fn render_form_data() {
        let mut form = HashMap::new();
        form.insert(
            "name".to_string(),
            FormDataValue::Text("{{name}}".to_string()),
        );
        form.insert(
            "file".to_string(),
            FormDataValue::File("{{path}}/file".to_string()),
        );

        super::render_form_data(&mut form, "name", "{{name}}", "temp_file");
        super::render_form_data(&mut form, "file", "{{path}}", "/tmp");

        assert_eq!(form["name"], FormDataValue::Text("temp_file".to_string()));
        assert_eq!(form["file"], FormDataValue::File("/tmp/file".to_string()));
    }

    #[test]
    fn render_form_urlencoded() {
        let mut form = HashMap::new();
        form.insert("page".to_string(), "{{page}}".to_string());
        form.insert("order".to_string(), "{{field}}:asc".to_string());

        super::render_form_urlencoded(&mut form, "page", "{{page}}", "10");
        super::render_form_urlencoded(&mut form, "order", "{{field}}", "id");

        assert_eq!(form["page"], "10".to_string());
        assert_eq!(form["order"], "id:asc".to_string());
    }
}
