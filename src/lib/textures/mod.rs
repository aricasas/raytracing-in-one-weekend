use crate::materials::Lambertian;
use crate::Color;
use crate::Vec3;

mod checker;
mod image;
pub mod perlin;
pub use self::image::Image;
pub use checker::CheckerTexture;
pub use perlin::Noise;

pub trait Texture: Send + Sync + Clone {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
    fn lambertian(self) -> Lambertian<Self> {
        Lambertian::new(self)
    }
}
