use image::{ImageBuffer, Rgb};
use std::sync::Arc;

use super::Texture;
use crate::{Color, Vec3};

pub struct Image {
    image: Arc<ImageBuffer<Rgb<u8>, Vec<u8>>>,
}
impl Image {
    pub fn new(image: Arc<ImageBuffer<Rgb<u8>, Vec<u8>>>) -> Self {
        Self { image }
    }
}

impl Texture for Image {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        todo!()
    }
}
