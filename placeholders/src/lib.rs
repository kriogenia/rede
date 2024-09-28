//! TODO

#![warn(clippy::pedantic)]

mod placeholders;
mod resolver;
pub mod value_picker;

pub use placeholders::Placeholders;
pub use resolver::Resolver;
pub use value_picker::ValuePicker;
