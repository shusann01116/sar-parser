use image::{ImageBuffer, Rgba};

use crate::core::sa::SymbolArt;

struct Canvas {}

// trait Drawer {
//     type Item = SymbolArt;

//     fn draw(sa: Item) -> ImageBuffer<Rgba<u8>, u8>;
// }

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use image::{imageops, GenericImage, RgbaImage};
    use rayon::prelude::*;

    use crate::{
        core::sa::{SymbolArt, SymbolArtLayer},
        parser,
        test::RAW_FILE,
    };

    #[test]
    fn test_projection_from_layer() {
        let now = Instant::now();
        println!("Started at {:?}", now);

        let bytes = Vec::from(RAW_FILE);
        let sa = parser::parse(bytes.into()).unwrap();
        println!("Parsed in {}ms", now.elapsed().as_millis());

        let layers = sa.layers();
        let mut result_image = RgbaImage::new(256, 256);

        let resource = crate::renderer::resource::Resource::new().unwrap();
        let images = layers
            .par_chunks(10)
            .map(|chunk| {
                let mut images = chunk
                    .par_iter()
                    .rev()
                    .enumerate()
                    .filter_map(|(i, layer)| {
                        if layer.is_hidden() {
                            return None;
                        }

                        let image = match resource.get_image(layer.symbol().id()) {
                            Some(image) => image,
                            None => {
                                println!("Symbol {} not found", layer.symbol().id());
                                return None;
                            }
                        };

                        let mut new_image = RgbaImage::new(256, 256);
                        imageproc::geometric_transformations::warp_into(
                            &image.to_image(),
                            &layer.try_into().unwrap(),
                            imageproc::geometric_transformations::Interpolation::Bilinear,
                            image::Rgba([0; 4]),
                            &mut new_image,
                        );

                        Some((i, new_image))
                    })
                    .collect::<Vec<_>>();
                images.par_sort_by_key(|(i, _)| *i);

                let mut canvas = RgbaImage::new(256, 256);
                for (_, img) in images {
                    imageops::overlay(&mut canvas, &img, 0, 0);
                }

                return canvas;
            })
            .collect::<Vec<_>>();
        println!("Sorted in {}ms", now.elapsed().as_millis());

        for overlay in images {
            imageops::overlay(&mut result_image, &overlay, 0, 0);
        }
        println!("Overlayed in {}ms", now.elapsed().as_millis());

        result_image
            .sub_image(128 - 96, 128 - 48, 193, 96)
            .to_image()
            .save(format!("test.png"))
            .unwrap();

        println!("Saved in {}ms", now.elapsed().as_millis());
    }
}
