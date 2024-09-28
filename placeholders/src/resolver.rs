use std::collections::HashMap;

use rede_schema::Request;

use crate::{Placeholders, ValuePicker};

// TODO remove the request from the ValuePicker trait

#[derive(Default)]
pub struct Resolver {
    pickers: Vec<Box<dyn ValuePicker>>,
}

impl Resolver {
    /// Creates a new empty resolver
    #[must_use]
    pub fn new() -> Self {
        Resolver::default()
    }

    /// Adds a new [`ValuePicker`] to the resolver. The resolver will use the provided
    /// pickers in the order they were adding to resolve the placholder values.
    pub fn add_picker(&mut self, picker: Box<dyn ValuePicker>) -> &mut Resolver {
        self.pickers.push(picker);
        self
    }

    /// The method to use a [`Resolver`]. It takes the placeholders and iterates over them
    /// using its value pickers to find values for each one.
    ///
    /// # Example
    ///
    /// The resolver only requires to be instantiated with the value pickers to use in the
    /// desired order, then calling `resolve` will output the generated values.
    ///
    /// ```
    /// # use crate::rede_placeholders::Resolver;
    /// # use crate::rede_placeholders::value_picker::{VariablesPicker, EnvVarPicker};
    /// #
    /// let toml = r#"
    /// http = { url = "http://localhost:8080/{{name}}/{{unresolved}}", method = "GET" }
    /// variables = { name = "variable" }
    /// "#;
    /// let request = rede_parser::parse_request(toml).unwrap();
    /// let placeholders = (&request).into();
    ///
    /// let mut resolver = Resolver::new();
    /// &resolver.add_picker(Box::new(EnvVarPicker)).add_picker(Box::new(VariablesPicker));
    ///
    /// let ph_values = resolver.resolve(&placeholders, &request);
    /// assert_eq!(ph_values.values["name"], Some("variable".to_string()));
    /// assert_eq!(ph_values.values["unresolved"], None);
    ///
    /// std::env::set_var("name", "env_var");
    /// std::env::set_var("unresolved", "fixed");
    /// let ph_values = resolver.resolve(&placeholders, &request);
    /// assert_ne!(ph_values.values["name"], Some("variable".to_string()));
    /// assert_eq!(ph_values.values["name"], Some("env_var".to_string()));
    /// assert_eq!(ph_values.values["unresolved"], Some("fixed".to_string()));
    /// ```
    #[must_use]
    pub fn resolve<'ph>(
        &self,
        placeholders: &'ph Placeholders,
        req: &Request,
    ) -> PlaceholderValues<'ph> {
        let values: HashMap<&str, Option<String>> = placeholders
            .keys()
            .map(|k| (k, self.pickers.iter().find_map(|p| p.pick_for(req, k))))
            .collect();
        PlaceholderValues { values }
    }
}

/// Wrapper over the resolved values for the placeholders. You can directly access its internal
/// `values` or use the convinient methods to ease access to its content.
#[derive(Debug)]
pub struct PlaceholderValues<'ph> {
    // Contains the collection placeholders and their values after from the `resolve` call.
    pub values: HashMap<&'ph str, Option<String>>,
}

impl<'ph> PlaceholderValues<'ph> {
    // Returns an iterator with the pairs of placeholder keys and their resolved values found.
    // Every placeholder unresolved won't be returned in this iterator, but in the `unresolved` one.
    pub fn resolved(&self) -> impl Iterator<Item = (&str, String)> {
        self.values
            .iter()
            .filter(|(_, v)| v.is_some())
            .map(|(k, v)| (*k, v.clone().unwrap()))
    }

    // Returns an iterator with the placeholder keys that ended up unresolved.
    // Every placeholder resolved won't be returned in this iterator, but in the `resolved` one.
    pub fn unresolved(&self) -> impl Iterator<Item = &str> {
        self.values
            .iter()
            .filter(|(_, v)| v.is_none())
            .map(|(k, _)| *k)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::PlaceholderValues;

    #[test]
    fn placeholder_values_iters() {
        let mut values = HashMap::new();
        values.insert("resolved", Some("value".to_string()));
        values.insert("unresolved", None);
        values.insert("resolved_too", Some("different_value".to_string()));

        let ph_values = PlaceholderValues { values };
        assert_eq!(ph_values.resolved().count(), 2);
        assert!(ph_values.unresolved().all(|k| k == "unresolved"));
    }
}
