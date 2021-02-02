#![allow(non_snake_case)]

mod color_types;
mod functions;
mod convert;

pub use color_types::*;
pub use functions::*;
pub use convert::Convert;

// Testing
mod convert_test;