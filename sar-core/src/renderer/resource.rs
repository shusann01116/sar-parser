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

    pub(crate) fn get_image(&self, id: SymbolId) -> Option<Image> {
        let index = ImageIndex::get(id)?;
        let sheet = self.sheets.get(&index.sheet)?;
        let (x, y) = Self::get_coordinates(&index);
        let image = imageops::crop_imm(sheet, x, y, SYMBOL_PIXELS, SYMBOL_PIXELS);
        Some(Image::new_with_sheet(image, index))
    }

    fn get_coordinates(index: &ImageIndex) -> (u32, u32) {
        let x = index.index % SYMBOL_WIDTH_NUM * SYMBOL_PIXELS;
        let y = index.index / SYMBOL_WIDTH_NUM * SYMBOL_PIXELS;
        (x, y)
    }
}

pub(crate) enum Image<'a> {
    R(SubImage<&'a DynamicImage>),
    G(SubImage<&'a DynamicImage>),
    B(SubImage<&'a DynamicImage>),
    Color(SubImage<&'a DynamicImage>),
}

impl<'a> Image<'a> {
    pub(crate) fn inner(&self) -> &SubImage<&'a DynamicImage> {
        match self {
            Image::R(image) => image,
            Image::G(image) => image,
            Image::B(image) => image,
            Image::Color(image) => image,
        }
    }
}

impl<'a> std::fmt::Debug for Image<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Image::R(_) => write!(f, "R"),
            Image::G(_) => write!(f, "G"),
            Image::B(_) => write!(f, "B"),
            Image::Color(_) => write!(f, "Color"),
        }
    }
}

impl<'a> Image<'a> {
    fn new_with_sheet(image: SubImage<&'a DynamicImage>, index: ImageIndex) -> Self {
        match index.sheet {
            ImageSheet::R => Self::R(image),
            ImageSheet::G => Self::G(image),
            ImageSheet::B => Self::B(image),
            ImageSheet::Color => Self::Color(image),
        }
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
                index: id,
            }),
            id @ 240..=480 => {
                let offset = match id {
                    305..400 => 240 + 16,
                    _ if id > 400 => 240 + 16 * 2,
                    _ => 240,
                };
                Some(Self {
                    sheet: ImageSheet::G,
                    index: id - offset,
                })
            }
            id @ 481..=720 => {
                let offset = match id {
                    561..641 => 480 + 16,
                    _ if id > 641 => 480 + 16 * 2,
                    _ => 480,
                };
                Some(Self {
                    sheet: ImageSheet::B,
                    index: id - offset,
                })
            }
            id @ 721..=768 => Some(Self {
                sheet: ImageSheet::Color,
                index: id - 720,
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
        if let Image::R(image) = image {
            assert_eq!(image.width(), SYMBOL_PIXELS);
        } else {
            panic!("image is not R");
        }
        if let Image::R(image) = image2 {
            assert_eq!(image.width(), SYMBOL_PIXELS);
        } else {
            panic!("image2 is not R");
        }
    }
}
