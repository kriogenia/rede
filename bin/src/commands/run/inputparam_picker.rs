use std::collections::BTreeMap;

use console::style;
use dialoguer::Input;
use rede_placeholders::ValuePicker;
use rede_schema::InputParam;

use crate::verbose;

/// [`ValuePicker`](rede_placeholders::value_picker::ValuePicker) implementation to obtain
/// the values from the user input.
pub(super) struct InputParamPicker<'req> {
    input_params: &'req BTreeMap<String, InputParam>,
}

impl<'req> ValuePicker for InputParamPicker<'req> {
    fn pick_for(&self, placeholder: &str) -> Option<String> {
        if let Some(ip) = self.input_params.get(placeholder) {
            let hint = if let Some(hint) = &ip.hint {
                format!(" ({hint})")
            } else {
                String::new()
            };
            let prompt = format!(
                "{} {}{}",
                style(">").bold().cyan(),
                style(placeholder).bold(),
                hint
            );
            let value: String = Input::new()
                .with_prompt(&prompt)
                .allow_empty(true)
                .interact()
                .unwrap();

            if value.is_empty() {
                verbose!("{}", style("  Using default if any"));
            } else {
                return Some(value);
            }
        }
        None
    }
}

impl<'req> InputParamPicker<'req> {
    pub fn new(input_params: &'req BTreeMap<String, InputParam>) -> Self {
        Self { input_params }
    }
}
