use super::symbol::Symbol;

/// Represents a position in 2D space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    /// X coordinate
    pub x: u8,
    /// Y coordinate
    pub y: u8,
}

pub trait SymbolArt<L>
where
    L: SymbolArtLayer,
{
    fn author_id(&self) -> u32;
    fn layers(&self) -> Vec<L>;
}

pub trait SymbolArtLayer {
    fn top_left(&self) -> Position;
    fn bottom_left(&self) -> Position;
    fn top_right(&self) -> Position;
    fn bottom_right(&self) -> Position;
    fn symbol(&self) -> Symbol;
    fn color(&self) -> Color;
}

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
