use crate::color::Color;
use crate::vec3::Vec3;

pub mod checker;
pub mod solid;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}
