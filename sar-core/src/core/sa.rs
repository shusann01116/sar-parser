use std::fmt::Debug;

use super::symbol::Symbol;

/// Represents a position in 2D space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    /// X coordinate
    pub x: u8,
    /// Y coordinate
    pub y: u8,
}

/// Represents a complete SymbolArt composition
///
/// A SymbolArt is a user-created artwork composed of multiple layers of symbols.
/// Each layer contains a symbol positioned and colored according to the layer's properties.
pub trait SymbolArt: Send + Sync {
    type Layer: SymbolArtLayer + Send + Sync;
    fn author_id(&self) -> u32;
    fn height(&self) -> u8;
    fn width(&self) -> u8;
    fn layers(&self) -> Vec<Self::Layer>;
    fn name(&self) -> String;
}

/// Represents a single layer in a SymbolArt composition
///
/// A SymbolArt is composed of multiple layers stacked on top of each other,
/// where each layer contains a symbol positioned and colored according to the
/// layer's properties. The order of layers determines their visual stacking
/// order in the final composition.
pub trait SymbolArtLayer {
    fn top_left(&self) -> Position;
    fn bottom_left(&self) -> Position;
    fn top_right(&self) -> Position;
    fn bottom_right(&self) -> Position;
    fn symbol(&self) -> Symbol;
    fn color(&self) -> Color;
    fn is_hidden(&self) -> bool;
}

/// Represents a color in RGBA format
///
/// Each component (red, green, blue, alpha) is represented as an 8-bit unsigned integer,
/// allowing for values between 0 and 255. The alpha channel controls transparency,
/// where 0 is fully transparent and 255 is fully opaque.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self { a, r, g, b }
    }
}

impl From<Color> for image::Rgba<u8> {
    fn from(value: Color) -> Self {
        image::Rgba([value.r, value.g, value.b, value.a])
    }
}
