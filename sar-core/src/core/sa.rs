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

pub trait SymbolArt: Send + Sync {
    type Layer: SymbolArtLayer + Send + Sync;
    fn author_id(&self) -> u32;
    fn layers(&self) -> Vec<Self::Layer>;
}

pub trait SymbolArtLayer {
    fn top_left(&self) -> Position;
    fn bottom_left(&self) -> Position;
    fn top_right(&self) -> Position;
    fn bottom_right(&self) -> Position;
    fn symbol(&self) -> Symbol;
    fn color(&self) -> Color;
    fn is_hidden(&self) -> bool;
}

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
