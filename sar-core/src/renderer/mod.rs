pub mod draw;
pub mod resource;

pub(crate) mod default {
    use super::draw::{Drawer, DrawerImpl};
    use crate::core::sa::{SymbolArt, SymbolArtLayer};
    use crate::Result;
    use image::{ImageBuffer, Rgba};

    pub fn drawer<S, L>() -> impl Drawer<S, L>
    where
        S: SymbolArt<Layer = L>,
        L: SymbolArtLayer + Sync,
    {
        DrawerImpl::default()
    }

    /// A handy function to draw a SymbolArt
    /// Should avoid using this function recursively
    pub fn draw<S, L>(sa: &S) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>>
    where
        S: SymbolArt<Layer = L>,
        L: SymbolArtLayer + Sync,
    {
        let drawer = DrawerImpl::default();
        drawer.draw_with_scale(sa, 1.0)
    }
}
