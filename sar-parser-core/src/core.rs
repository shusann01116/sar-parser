use crate::parser::Position;

pub trait SymbolArt<L>
where
    L: SymbolArtLayer,
{
    fn author_id(&self) -> u32;
    fn layers(&self) -> Vec<L>;
    fn name(&self) -> String;
}

pub trait SymbolArtLayer {
    fn top_left(&self) -> Position;
    fn bottom_left(&self) -> Position;
    fn top_right(&self) -> Position;
    fn bottom_right(&self) -> Position;
}

enum Symbol {}

#[cfg(test)]
mod tests {
    use crate::test::RAW_FILE;

    use super::*;

    #[test]
    fn test_symbol_art_format() {}
}
