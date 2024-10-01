use std::collections::HashMap;

use crate::{Placeholders, ValuePicker};

/// Uses a list of [`ValuePicker`] to resolve the value of [`Placeholders`].
#[derive(Default)]
pub struct Resolver<'req> {
    pickers: Vec<Box<dyn ValuePicker + 'req>>,
}

impl<'req> Resolver<'req> {
    /// Creates a new empty resolver
    #[must_use]
    pub fn new() -> Self {
        Resolver::default()
    }

    /// Adds a new [`ValuePicker`] to the resolver. The resolver will use the provided
    /// pickers in the order they were adding to resolve the placholder values.
    #[must_use]
    pub fn add_picker(mut self, picker: Box<dyn ValuePicker + 'req>) -> Self {
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
    /// let mut resolver = Resolver::new()
    ///     .add_picker(Box::new(EnvVarPicker))
    ///     .add_picker(Box::new(VariablesPicker::new(&request.variables)));
    ///
    /// let ph_values = resolver.resolve(&placeholders);
    /// assert_eq!(ph_values.get_value("name"), Some(&"variable".to_string()));
    /// assert_eq!(ph_values.get_value("unresolved"), None);
    ///
    /// std::env::set_var("name", "env_var");
    /// std::env::set_var("unresolved", "fixed");
    /// let ph_values = resolver.resolve(&placeholders);
    /// assert_ne!(ph_values.get_value("name"), Some(&"variable".to_string()));
    /// assert_eq!(ph_values.get_value("name"), Some(&"env_var".to_string()));
    /// assert_eq!(ph_values.get_value("unresolved"), Some(&"fixed".to_string()));
    /// ```
    #[must_use]
    pub fn resolve<'ph>(&self, placeholders: &'ph Placeholders) -> PlaceholderValues<'ph> {
        let values: HashMap<&str, Option<String>> = placeholders
            .keys()
            .map(|k| (k, self.pickers.iter().find_map(|p| p.pick_for(k))))
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
    /// Returns the value for the given key if any. This method doesn't differentiate into keys
    /// of inexistent placeholders and unresolved keys derived from a value not found.
    ///
    /// In case of wanting to query a key taking into account this distinction, you can access
    /// the internal [`HashMap`] `values`, using query against this map would return a nested option
    /// where `None` are inexistent placholders and `Some(None)` would be unresolved placeholders.
    #[must_use]
    pub fn get_value(&self, k: &str) -> Option<&String> {
        // TODO a get returning Solved(val), Unresolved and Unknown?
        match self.values.get(k) {
            Some(val) => val.as_ref(),
            None => None,
        }
    }

    // Returns an iterator with the pairs of placeholder keys and their resolved values found.
    // Every placeholder unresolved won't be returned in this iterator, but in the `unresolved` one.
    #[allow(clippy::missing_panics_doc)] // clippy is missing the filter ensuring the safe unwrap
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
    fn get_value() {
        let mut values = HashMap::new();
        values.insert("resolved", Some("value".to_string()));
        values.insert("unresolved", None);

        let ph_values = PlaceholderValues { values };
        assert_eq!(ph_values.get_value("resolved"), Some(&"value".to_string()));
        assert_eq!(ph_values.get_value("unresolved"), None);
        assert_eq!(ph_values.get_value("unknown"), None);
    }

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
