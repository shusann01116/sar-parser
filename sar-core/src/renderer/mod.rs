mod draw;
pub mod resource;

/// A handy function to draw a SymbolArt
pub(crate) mod default {
    use super::draw::{Drawer, DrawerImpl};
    use crate::core::sa::{SymbolArt, SymbolArtLayer};
    use crate::Result;
    use image::{ImageBuffer, Rgba};

    pub fn draw<S, L>(sa: &S) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>>
    where
        S: SymbolArt<Layer = L>,
        L: SymbolArtLayer + Sync,
    {
        let drawer = DrawerImpl::default();
        drawer.draw(sa)
    }
}
