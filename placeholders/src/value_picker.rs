use rede_schema::Request;

/// Selects a value to replace the placeholder from the request or based on it.
pub trait ValuePicker {
    /// For a given `placeholder` generates a possible value to replace
    /// in the request
    fn pick_for(placeholder: &str, req: &Request) -> Option<String>;
}
