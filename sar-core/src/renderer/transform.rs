#[cfg(test)]
mod tests {
    use std::{sync::mpsc, time::Instant};

    use image::{imageops, DynamicImage, GenericImage, RgbaImage};
    use rayon::prelude::*;

    use crate::{
        core::sa::{SymbolArt, SymbolArtLayer},
        parser,
        test::RAW_FILE,
    };

    #[test]
    fn test_image_proc_projection() {
        let bytes = include_bytes!("../../assets/symbols_b.png");
        let image = image::load_from_memory(bytes).unwrap();

        let projection = imageproc::geometric_transformations::Projection::from_control_points(
            [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)],
            [(0.0, 0.0), (0.2, 1.0), (0.8, 0.0), (1.0, 1.0)],
        )
        .unwrap();
        let transformed = imageproc::geometric_transformations::warp(
            &image.to_rgba8(),
            &projection,
            imageproc::geometric_transformations::Interpolation::Bilinear,
            image::Rgba([0; 4]),
        );

        transformed.save("test.png").unwrap();
    }

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
        let mut images = layers
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
        println!("Sorted in {}ms", now.elapsed().as_millis());

        let overlays = images
            .par_chunks(10)
            .map(|chunk| {
                let mut image = RgbaImage::new(256, 256);
                for (_, i) in chunk {
                    imageops::overlay(&mut image, i, 0, 0);
                }
                image
            })
            .collect::<Vec<_>>();
        for overlay in overlays {
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
