use image::DynamicImage;

use crate::core::result::Result;
use crate::core::sa::{SymbolArt, SymbolArtLayer};

pub mod resource;
mod transform;

pub fn render<T: SymbolArt<L>, L: SymbolArtLayer>(sa: T) -> Result<DynamicImage> {
    todo!()
}
