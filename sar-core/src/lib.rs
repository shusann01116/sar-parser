mod core;
pub mod parser;
pub mod renderer;
pub use core::result::Result;
pub use parser::payload::parse;
pub use renderer::default::{draw, drawer};

#[cfg(test)]
mod test;
