use crate::{core::sa::Color, Result};
use image::{imageops, GenericImage, ImageBuffer, Pixel, Rgba, RgbaImage};
use imageproc::geometric_transformations::Projection;
use std::sync::mpsc;

use crate::core::{
    result::SARError,
    sa::{SymbolArt, SymbolArtLayer},
};
use rayon::prelude::*;

use super::resource::{self};

pub trait Drawer<S, L>
where
    S: SymbolArt<Layer = L>,
    L: SymbolArtLayer,
{
    fn draw(&self, sa: &S) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>>;
    fn draw_with_scale(&self, sa: &S, scale: f32) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>>;
}

pub struct SymbolArtDrawer {
    resource: resource::Resource,
    canvas_size: (u32, u32),
    view_size: (u32, u32),
    chunk_size: usize,
    suppress_failure: bool,
}

impl SymbolArtDrawer {
    pub fn new() -> Self {
        let resource = resource::Resource::new().unwrap();
        let canvas_size = (256, 256);
        let view_size = (193, 96);

        Self {
            resource,
            canvas_size,
            view_size,
            chunk_size: 10,
            suppress_failure: true,
        }
    }

    pub fn with_raise_error(mut self, raise_error: bool) -> Self {
        self.suppress_failure = !raise_error;
        self
    }

    fn calc_canvas_size(&self, scale: f32) -> (u32, u32) {
        (
            (self.canvas_size.0 as f32 * scale) as u32,
            (self.canvas_size.1 as f32 * scale) as u32,
        )
    }

    fn calc_view_size(&self, scale: f32) -> (u32, u32) {
        (
            (self.view_size.0 as f32 * scale) as u32,
            (self.view_size.1 as f32 * scale) as u32,
        )
    }

    fn get_projection<L>(&self, layer: &L, scale: f32) -> Result<Projection>
    where
        L: SymbolArtLayer,
    {
        let top_left = layer.top_left();
        let bottom_left = layer.bottom_left();
        let top_right = layer.top_right();
        let bottom_right = layer.bottom_right();

        let symbol_width = self.resource.symbol_pixels as f32;
        let from = [
            (0.0, 0.0),
            (symbol_width, 0.0),
            (symbol_width, symbol_width),
            (0.0, symbol_width),
        ];
        let to = [
            (top_left.x as f32 * scale, top_left.y as f32 * scale),
            (top_right.x as f32 * scale, top_right.y as f32 * scale),
            (bottom_right.x as f32 * scale, bottom_right.y as f32 * scale),
            (bottom_left.x as f32 * scale, bottom_left.y as f32 * scale),
        ];

        let projection =
            imageproc::geometric_transformations::Projection::from_control_points(from, to)
                .ok_or(SARError::ProjectionError(from, to))?;

        Ok(projection)
    }

    fn render_symbol(base: &mut RgbaImage, symbol: &mut RgbaImage, color: RenderColor) {
        for (x, y, pixel) in base.enumerate_pixels_mut() {
            let symbol_pixel = symbol.get_pixel(x, y);
            if symbol_pixel[3] > 0 {
                match color {
                    RenderColor::Color(color) => pixel.blend(&color.into()),
                    RenderColor::None => {
                        pixel.blend(symbol_pixel);
                    }
                }
            }
        }
    }
}

pub enum RenderColor {
    Color(Color),
    None,
}

impl Default for SymbolArtDrawer {
    fn default() -> Self {
        Self {
            resource: resource::Resource::new().unwrap(),
            canvas_size: (256, 256),
            view_size: (193, 96),
            chunk_size: 10,
            suppress_failure: true,
        }
    }
}

impl<S, L> Drawer<S, L> for SymbolArtDrawer
where
    S: SymbolArt<Layer = L>,
    L: SymbolArtLayer + Sync,
{
    fn draw(&self, sa: &S) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        self.draw_with_scale(sa, 1.0)
    }

    fn draw_with_scale(&self, sa: &S, scale: f32) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let canvas_size = self.calc_canvas_size(scale);
        let mut canvas = RgbaImage::from_pixel(canvas_size.0, canvas_size.1, image::Rgba([0; 4]));

        let (tx, rx) = mpsc::channel();
        let mut overlays = sa
            .layers()
            .par_chunks(self.chunk_size)
            .rev()
            .enumerate()
            .filter_map(|(i, chunk)| {
                let tx = tx.clone();
                let mut canvas = RgbaImage::new(canvas_size.0, canvas_size.1);
                for layer in chunk.iter().rev() {
                    if layer.is_hidden() {
                        continue;
                    }

                    let image = match self.resource.get_image(layer.symbol().id()) {
                        Some(image) => image,
                        None => {
                            if self.suppress_failure {
                                return None;
                            }

                            tx.send(SARError::SymbolNotFound(layer.symbol().id()))
                                .unwrap();
                            return None;
                        }
                    };

                    let mut symbol = RgbaImage::new(canvas_size.0, canvas_size.1);
                    let projection = match self.get_projection(layer, scale) {
                        Ok(projection) => projection,
                        Err(e) => {
                            if self.suppress_failure {
                                return None;
                            }

                            tx.send(e).unwrap();
                            return None;
                        }
                    };

                    imageproc::geometric_transformations::warp_into(
                        &image.inner().to_image(),
                        &projection,
                        imageproc::geometric_transformations::Interpolation::Nearest,
                        image::Rgba([0; 4]),
                        &mut symbol,
                    );

                    if let resource::Image::Color(_) = image {
                        SymbolArtDrawer::render_symbol(&mut canvas, &mut symbol, RenderColor::None);
                    } else {
                        SymbolArtDrawer::render_symbol(
                            &mut canvas,
                            &mut symbol,
                            RenderColor::Color(layer.color()),
                        );
                    }
                }

                Some((i, canvas))
            })
            .collect::<Vec<_>>();

        drop(tx);
        if let Ok(e) = rx.recv() {
            return Err(e);
        }

        overlays.sort_by_key(|(i, _)| *i);
        for (_, overlay) in overlays {
            imageops::overlay(&mut canvas, &overlay, 0, 0);
        }

        let view_size = self.calc_view_size(scale);
        Ok(canvas
            .sub_image(
                canvas_size.0 / 2 - view_size.0 / 2,
                canvas_size.1 / 2 - view_size.1 / 2,
                view_size.0,
                view_size.1,
            )
            .to_image())
    }
}

#[cfg(test)]
mod tests {
    use image::codecs::png::PngEncoder;

    use super::*;
    use crate::{parse, test::RAW_FILE};

    #[test]
    fn test_drawer() {
        let bytes = Vec::from(RAW_FILE);
        let sa = parse(bytes).unwrap();

        let drawer = SymbolArtDrawer::new().with_raise_error(true);
        let image = drawer.draw(&sa).unwrap();

        // Assert
        let mut buff = Vec::new();
        image
            .write_with_encoder(PngEncoder::new(&mut buff))
            .unwrap();
        assert_eq!(buff.len(), include_bytes!("fixture/test.png").len());
    }

    #[test]
    fn test_drawer_with_scale() {
        let bytes = Vec::from(RAW_FILE);
        let sa = parse(bytes).unwrap();

        let drawer = SymbolArtDrawer::default();
        let image = drawer.draw_with_scale(&sa, 2.0).unwrap();

        // Assert
        let mut buff = Vec::new();
        image
            .write_with_encoder(PngEncoder::new(&mut buff))
            .unwrap();
        assert_eq!(buff.len(), include_bytes!("fixture/testx2.png").len());
    }
}
