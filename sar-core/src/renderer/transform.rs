#[cfg(test)]
mod tests {
    use std::result;

    use image::{imageops, DynamicImage};

    use crate::{
        core::sa::{SymbolArt, SymbolArtLayer},
        parser,
        renderer::resource,
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
        let bytes = Vec::from(RAW_FILE);
        let sa = parser::parse(bytes.into()).unwrap();
        let layers = sa.layers();
        let mut result_image = DynamicImage::new(256, 256, image::ColorType::Rgba8);
        for layer in layers.iter().rev() {
            if layer.is_hidden() {
                continue;
            }

            let resource = crate::renderer::resource::Resource::new().unwrap();

            let image = match resource.get_image(layer.symbol().id()) {
                Some(image) => image,
                None => {
                    println!("Symbol {} not found", layer.symbol().id());
                    continue;
                }
            };

            let mut new_image = DynamicImage::new(256, 256, image::ColorType::Rgba8).into();
            imageproc::geometric_transformations::warp_into(
                &image.to_image(),
                &layer.try_into().unwrap(),
                imageproc::geometric_transformations::Interpolation::Bilinear,
                image::Rgba([0; 4]),
                &mut new_image,
            );
            imageops::overlay(&mut result_image, &new_image, 0, 0);
        }
        result_image.save(format!("test.png")).unwrap();
    }
}
