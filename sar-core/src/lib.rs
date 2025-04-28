//! # SAR Parser
//!
//! A Rust library for parsing and rendering SymbolArt (SAR) files from Phantasy Star Online 2.
//!
//! ## Overview
//!
//! This library provides functionality to:
//! - Parse SAR files into a structured format
//! - Render SymbolArt compositions into images
//! - Manipulate and inspect SymbolArt properties
//!
//! ## Basic Usage
//!
//! ```no_run
//! use sar_core::{parse, SymbolArtDrawer, SymbolArt};
//! use sar_core::renderer::draw::Drawer;
//!
//! // Parse a SAR file
//! let bytes = std::fs::read("example.sar").unwrap();
//! let symbol_art = parse(bytes).unwrap();
//!
//! // Create a drawer and render the SymbolArt
//! let drawer = SymbolArtDrawer::new();
//! let image = drawer.draw(&symbol_art).unwrap();
//!
//! // Save the rendered image
//! image.save("output.png").unwrap();
//! ```
//!
//! ## Core Concepts
//!
//! ### SymbolArt
//!
//! A SymbolArt is a user-created artwork composed of multiple layers of symbols. Each layer contains:
//! - A symbol (from a predefined set)
//! - Position information (four corner points)
//! - Color and transparency settings
//! - Visibility state
//!
//! ### Layers
//!
//! Each layer in a SymbolArt has the following properties:
//! - Position: Defined by four corner points (top-left, bottom-left, top-right, bottom-right)
//! - Symbol: A unique identifier for the symbol used in the layer
//! - Color: RGBA color values
//! - Visibility: Whether the layer is hidden or visible
//!
//! ## Advanced Usage
//!
//! ### Customizing the Renderer
//!
//! ```no_run
//! use sar_core::{parse, SymbolArtDrawer, SymbolArt};
//! use sar_core::renderer::draw::Drawer;
//!
//! let bytes = std::fs::read("example.sar").unwrap();
//! let symbol_art = parse(bytes).unwrap();
//!
//! // Create a customized drawer
//! let drawer = SymbolArtDrawer::new()
//!     .with_raise_error(true)  // Make errors fatal
//!     .with_chunk_size(5);     // Adjust parallel processing
//!
//! // Render with custom scale
//! let image = drawer.draw_with_scale(&symbol_art, 2.0).unwrap();
//! ```
//!
//! ### Inspecting SymbolArt Properties
//!
//! ```no_run
//! use sar_core::{parse, SymbolArt, SymbolArtLayer};
//!
//! let bytes = std::fs::read("example.sar").unwrap();
//! let symbol_art = parse(bytes).unwrap();
//!
//! // Get basic information
//! println!("Name: {}", symbol_art.name());
//! println!("Author ID: {}", symbol_art.author_id());
//! println!("Dimensions: {}x{}", symbol_art.width(), symbol_art.height());
//!
//! // Inspect layers
//! for (i, layer) in symbol_art.layers().iter().enumerate() {
//!     println!("Layer {}:", i);
//!     println!("  Symbol ID: {}", layer.symbol().id());
//!     println!("  Color: {:?}", layer.color());
//!     println!("  Hidden: {}", layer.is_hidden());
//! }
//! ```
//!
//! ## File Format
//!
//! SAR files have the following structure:
//! - Header (8 bytes)
//!   - Author ID (4 bytes, big-endian)
//!   - Number of layers (1 byte)
//!   - Height (1 byte)
//!   - Width (1 byte)
//!   - Sound effect (1 byte)
//! - Layer data (variable length)
//!   - Position data (8 bytes per layer)
//!   - Layer properties (4 bytes per layer)
//! - Name data (UTF-16LE, up to 13 characters)
//!
//! ## Error Handling
//!
//! The library uses a custom `Result` type for error handling. Common errors include:
//! - Invalid file format
//! - I/O errors
//! - Symbol not found
//! - Projection errors during rendering
//!
//! ```
//! use sar_core::{parse, Result};
//!
//! fn process_sar_file(bytes: &[u8]) -> Result<()> {
//!     let symbol_art = parse(Vec::from(bytes))?;
//!     // Process the SymbolArt...
//!     Ok(())
//! }
//! ```
//!
//! ## Performance Considerations
//!
//! - The renderer uses parallel processing for layer rendering
//! - Symbol resources are cached for better performance
//! - The chunk size can be adjusted to balance parallelization overhead
//!
//! ## Dependencies
//!
//! - `image`: For image processing and rendering
//! - `imageproc`: For geometric transformations
//! - `rayon`: For parallel processing
//! - `blowfish`: For file decryption
//! - `ages_prs`: For file decompression
//!
//! ## License
//!
//! This project is licensed under the MIT License - see the LICENSE file for details.

mod core;
mod parser;
pub mod renderer;
pub use core::result::Result;
pub use core::sa::{Color, SymbolArt, SymbolArtLayer};
pub use parser::payload::parse;
pub use renderer::SymbolArtDrawer;
pub use renderer::default::draw;

#[cfg(test)]
mod test;
