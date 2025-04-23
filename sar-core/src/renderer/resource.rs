use std::collections::HashMap;

use crate::core::result::Result;
use image::{imageops, DynamicImage, SubImage};

use crate::core::symbol::SymbolId;

const SYMBOLS_R: &[u8] = include_bytes!("../../assets/symbols_r.png");
const SYMBOLS_G: &[u8] = include_bytes!("../../assets/symbols_g.png");
const SYMBOLS_B: &[u8] = include_bytes!("../../assets/symbols_b.png");
const SYMBOLS_COLOR: &[u8] = include_bytes!("../../assets/symbols_color.png");

const SYMBOL_PIXELS: u32 = 64;
const SYMBOL_WIDTH_NUM: u32 = 16;

#[derive(Eq, Hash, PartialEq)]
enum ImageSheet {
    R,
    G,
    B,
    Color,
}

pub struct Resource {
    sheets: HashMap<ImageSheet, DynamicImage>,
}

impl Resource {
    pub fn new() -> Result<Self> {
        let r = image::load_from_memory(SYMBOLS_R)?;
        let g = image::load_from_memory(SYMBOLS_G)?;
        let b = image::load_from_memory(SYMBOLS_B)?;
        let color = image::load_from_memory(SYMBOLS_COLOR)?;

        let mut sheets = HashMap::new();
        sheets.insert(ImageSheet::R, r);
        sheets.insert(ImageSheet::G, g);
        sheets.insert(ImageSheet::B, b);
        sheets.insert(ImageSheet::Color, color);

        Ok(Self { sheets })
    }

    pub fn get_image(&self, id: SymbolId) -> Option<SubImage<&DynamicImage>> {
        let index = ImageIndex::get(id)?;
        let sheet = self.sheets.get(&index.sheet)?;
        let (x, y, width, height) = Self::get_coordinates(&index);
        let image = imageops::crop_imm(sheet, x, y, width, height);
        Some(image)
    }

    fn get_coordinates(index: &ImageIndex) -> (u32, u32, u32, u32) {
        let x = index.index % SYMBOL_WIDTH_NUM * SYMBOL_PIXELS;
        let y = index.index / SYMBOL_WIDTH_NUM * SYMBOL_PIXELS;
        (x, y, SYMBOL_PIXELS, SYMBOL_PIXELS)
    }
}

struct ImageIndex {
    sheet: ImageSheet,
    index: u32,
}

impl ImageIndex {
    fn get(id: SymbolId) -> Option<Self> {
        match id.id() {
            id @ 1..=80 => Some(Self {
                sheet: ImageSheet::R,
                index: id - 1,
            }),
            id @ 241..=480 => {
                let offset = match id {
                    305..400 => 241 + 16,
                    _ if id > 400 => 241 + 16 * 2,
                    _ => 241,
                };
                Some(Self {
                    sheet: ImageSheet::G,
                    index: id - offset,
                })
            }
            id @ 481..=720 => {
                let offset = match id {
                    561..641 => 481 + 16,
                    _ if id > 641 => 481 + 16 * 2,
                    _ => 481,
                };
                Some(Self {
                    sheet: ImageSheet::B,
                    index: id - offset,
                })
            }
            id @ 721..=768 => Some(Self {
                sheet: ImageSheet::Color,
                index: id - 721,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use image::GenericImageView;

    use super::*;

    #[test]
    fn test_get_image_index() {
        let index = ImageIndex::get(SymbolId::new(1));
        assert!(index.is_some());
    }

    #[test]
    fn test_get_image() {
        let resource = Resource::new().unwrap();
        let image = resource.get_image(SymbolId::new(40)).unwrap();
        let image2 = resource.get_image(SymbolId::new(41)).unwrap();
        assert_eq!(image.width(), SYMBOL_PIXELS);
        assert_eq!(image.height(), SYMBOL_PIXELS);
        assert_eq!(image2.width(), SYMBOL_PIXELS);
        assert_eq!(image2.height(), SYMBOL_PIXELS);
    }
}
