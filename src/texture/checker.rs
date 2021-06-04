use super::{solid::SolidColor, Texture};
use crate::{color::Color, material::Material, vec3::Vec3};

#[derive(Clone)]
pub struct CheckerTexture<T: Texture, G: Texture> {
    odd: T,
    even: G,
}

impl<T: Texture, G: Texture> CheckerTexture<T, G> {
    pub fn new(odd: T, even: G) -> Self {
        Self { odd, even }
    }
}
impl CheckerTexture<SolidColor, SolidColor> {
    pub const fn from_color(odd: Color, even: Color) -> Self {
        Self {
            odd: SolidColor::from_color(odd),
            even: SolidColor::from_color(even),
        }
    }
}

impl<T: Texture, G: Texture> Texture for CheckerTexture<T, G> {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let sines = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
