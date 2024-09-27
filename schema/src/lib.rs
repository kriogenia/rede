//! TODO

#![warn(clippy::pedantic)]

/// Contains all the specific types used in the body
pub mod body;

#[cfg(feature = "input_params")]
mod input_param;

#[doc(inline)]
pub use body::Body;

#[cfg(feature = "input_params")]
#[doc(inline)]
pub use input_param::InputParam;
