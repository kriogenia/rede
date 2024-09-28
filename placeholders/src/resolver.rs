use std::collections::HashMap;

use rede_schema::Request;

use crate::{Placeholders, ValuePicker};

// TODO remove the request from the ValuePicker trait
// TODO return an structure with the values instead of the Result nomad (or map with options?)

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
    /// http = { url = "http://localhost:8080/{{name}}", method = "GET" }
    /// variables = { name = "variable "}
    /// "#;
    /// let request = rede_parser::parse_request(toml).unwrap();
    /// let placeholders = (&request).into();
    ///
    /// let mut resolver = Resolver::new();
    /// &resolver.add_picker(Box::new(EnvVarPicker)).add_picker(Box::new(VariablesPicker));
    ///
    /// let values = resolver.resolve(&placeholders, &request).expect("All placeholders resolved");
    /// assert_eq!(values["name"], request.variables["name"]);
    ///
    /// std::env::set_var("name", "value");
    /// let values = resolver.resolve(&placeholders, &request).expect("All placeholders resolved");
    /// assert_ne!(values["name"], request.variables["name"]);
    /// assert_eq!(values["name"], "value");
    /// ```
    ///
    /// # Errors
    ///
    /// // TODO to be removed
    pub fn resolve<'a>(
        &self,
        placeholders: &'a Placeholders,
        req: &Request,
    ) -> Result<HashMap<&'a str, String>, Vec<&'a str>> {
        let mut values = HashMap::new();
        let mut missing = Vec::new();
        for k in placeholders.keys() {
            if let Some(value) = self.pickers.iter().find_map(|p| p.pick_for(req, k)) {
                values.insert(k, value);
            } else {
                missing.push(k);
            }
        }
        if missing.is_empty() {
            Ok(values)
        } else {
            Err(missing)
        }
    }
}
