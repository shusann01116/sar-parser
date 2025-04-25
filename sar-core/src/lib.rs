mod core;
pub mod parser;
mod renderer;
pub use core::result::Result;
pub use renderer::default::draw;

#[cfg(test)]
mod test;
