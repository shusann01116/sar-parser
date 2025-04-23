#[cfg(test)]
mod tests {

    #[test]
    fn test_image_proc_projection() {
        let bytes = include_bytes!("../../assets/symbols_b.png");
        let image = image::load_from_memory(bytes).unwrap();

        let projection = imageproc::geometric_transformations::Projection::from_matrix([
            1.0, 0.2, 0.0, 0.2, 1.0, 0.0, 0.0, 0.0, 1.0,
        ])
        .unwrap();
        let transformed = imageproc::geometric_transformations::warp(
            &image.to_rgba8(),
            &projection,
            imageproc::geometric_transformations::Interpolation::Bilinear,
            image::Rgba([0; 4]),
        );

        transformed.save("test.png").unwrap();
    }
}
