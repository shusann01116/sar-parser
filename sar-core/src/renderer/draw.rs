use crate::Result;
use image::{imageops, GenericImage, ImageBuffer, Rgba, RgbaImage};
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

pub(super) struct DrawerImpl {
    resource: resource::Resource,
    canvas_size: (u32, u32),
    view_size: (u32, u32),
}

impl DrawerImpl {
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

        const WIDTH: f32 = 64.0;
        let from = [(0.0, 0.0), (WIDTH, 0.0), (WIDTH, WIDTH), (0.0, WIDTH)];
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
}

impl Default for DrawerImpl {
    fn default() -> Self {
        Self {
            resource: resource::Resource::new().unwrap(),
            canvas_size: (256, 256),
            view_size: (193, 96),
        }
    }
}

impl<S, L> Drawer<S, L> for DrawerImpl
where
    S: SymbolArt<Layer = L>,
    L: SymbolArtLayer + Sync,
{
    fn draw(&self, sa: &S) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        self.draw_with_scale(sa, 1.0)
    }

    fn draw_with_scale(&self, sa: &S, scale: f32) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let canvas_size = self.calc_canvas_size(scale);
        let mut canvas = RgbaImage::new(canvas_size.0, canvas_size.1);

        let (tx, rx) = mpsc::channel();
        let overlays = sa
            .layers()
            .par_chunks(10)
            .rev()
            .filter_map(|chunk| {
                let tx = tx.clone();
                let mut canvas = RgbaImage::new(canvas_size.0, canvas_size.1);
                for layer in chunk {
                    if layer.is_hidden() {
                        continue;
                    }

                    let image = match self.resource.get_image(layer.symbol().id()) {
                        Some(image) => image,
                        None => {
                            tx.send(SARError::SymbolNotFound(layer.symbol().id()))
                                .unwrap();
                            return None;
                        }
                    };

                    let mut symbol = RgbaImage::new(canvas_size.0, canvas_size.1);
                    let projection = match self.get_projection(layer, scale) {
                        Ok(projection) => projection,
                        Err(e) => {
                            tx.send(e).unwrap();
                            return None;
                        }
                    };
                    imageproc::geometric_transformations::warp_into(
                        &image.to_image(),
                        &projection,
                        imageproc::geometric_transformations::Interpolation::Bilinear,
                        image::Rgba([0; 4]),
                        &mut symbol,
                    );
                    imageops::overlay(&mut canvas, &symbol, 0, 0);
                }

                Some(canvas)
            })
            .collect::<Vec<_>>();

        drop(tx);
        if let Ok(e) = rx.recv() {
            return Err(e);
        }

        for overlay in overlays {
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
    use std::time::Instant;

    use super::*;
    use crate::{parser, test::RAW_FILE};

    #[test]
    fn test_drawer() {
        let now = Instant::now();
        println!("Started at {:?}", now);

        let bytes = Vec::from(RAW_FILE);
        let sa = parser::parse(bytes.into()).unwrap();
        println!("Parsed in {}ms", now.elapsed().as_millis());

        let drawer = DrawerImpl::default();
        let image = drawer.draw(&sa).unwrap();
        println!("Drawn in {}ms", now.elapsed().as_millis());

        image.save(format!("test.png")).unwrap();
    }

    #[test]
    fn test_drawer_with_scale() {
        let now = Instant::now();
        println!("Started at {:?}", now);

        let bytes = Vec::from(RAW_FILE);
        let sa = parser::parse(bytes.into()).unwrap();
        println!("Parsed in {}ms", now.elapsed().as_millis());

        let drawer = DrawerImpl::default();
        let image = drawer.draw_with_scale(&sa, 2.0).unwrap();
        println!("Drawn in {}ms", now.elapsed().as_millis());

        image.save(format!("testx2.png")).unwrap();
    }
}
