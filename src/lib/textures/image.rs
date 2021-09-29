use image::{ImageBuffer, Rgb};
use std::sync::Arc;

use super::Texture;
use crate::{Color, Vec3};

#[derive(Clone)]
pub struct Image {
    image: Arc<ImageBuffer<Rgb<u8>, Vec<u8>>>,
}
impl Image {
    pub fn new(image: Arc<ImageBuffer<Rgb<u8>, Vec<u8>>>) -> Self {
        Self { image }
    }
}

impl Texture for Image {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Color {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

        let image_width = self.image.width();
        let image_height = self.image.height();

        let pixel_coord_x = ((u * f64::from(image_width)) as u32).min(image_width);
        let pixel_coord_y = ((v * f64::from(image_height)) as u32).min(image_height);

        let pixel = self.image[(pixel_coord_x, pixel_coord_y)];

        pixel.into()
    }
}
