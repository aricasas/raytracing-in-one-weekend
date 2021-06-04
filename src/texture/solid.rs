use super::Texture;
use crate::{color::Color, vec3::Vec3};

#[derive(Clone)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            color_value: Color::new(r, g, b),
        }
    }
    pub const fn from_color(color_value: Color) -> Self {
        Self { color_value }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: &Vec3) -> Color {
        self.color_value.clone()
    }
}
