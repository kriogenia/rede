/// Contains the different properties that can be defined for an input parameter.
#[derive(Debug, Default, PartialEq)]
pub struct InputParam {
    /// Hint to provide to the user when asking for the input
    pub hint: Option<String>,
}
