pub mod draw;
pub mod resource;
pub use draw::SymbolArtDrawer;

pub(crate) mod default {
    use super::draw::{Drawer, SymbolArtDrawer};
    use crate::core::sa::{SymbolArt, SymbolArtLayer};
    use crate::Result;
    use image::{ImageBuffer, Rgba};

    /// A handy function to draw a SymbolArt
    /// Should avoid using this function recursively
    pub fn draw<S, L>(sa: &S) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>>
    where
        S: SymbolArt<Layer = L>,
        L: SymbolArtLayer + Sync,
    {
        let drawer = SymbolArtDrawer::default();
        drawer.draw_with_scale(sa, 1.0)
    }
}
