use super::Texture;
use crate::{Color, Vec3};

#[derive(Clone)]
pub struct Solid {
    color_value: Color,
}

impl Solid {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            color_value: Color::new(r, g, b),
        }
    }
    pub const fn from_color(color_value: Color) -> Self {
        Self { color_value }
    }
}

impl Texture for Solid {
    fn value(&self, _: f64, _: f64, _: &Vec3) -> Color {
        self.color_value
    }
}
