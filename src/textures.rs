use crate::Color;
use crate::Vec3;

mod checker;
mod image;
mod solid;
pub use self::image::Image;
pub use checker::CheckerTexture;
pub use solid::SolidColor;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}
