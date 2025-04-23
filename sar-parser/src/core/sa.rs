use crate::parser::payload::Payload;

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
}

#[cfg(test)]
mod tests {
    use crate::test::RAW_FILE;

    use super::*;

    #[test]
    fn test_symbol_art_format() {}
}
