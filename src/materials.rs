use crate::Vec3;

use super::hittable::HitRecord;
use super::Color;
use super::Ray;

pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;
pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;

/// A trait that all materials must implement
///
/// Defines the scattering behaviour of that material
pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterRecord>;
    fn emitted(&self, u: f64, v: f64, point: &Vec3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

/// A struct to store relevant data of a ray scattering off something
pub struct ScatterRecord {
    pub attenuation: Color,
    /// The scattered `Ray` with its new direction
    pub scattered_ray: Ray,
}
impl ScatterRecord {
    /// Returns a new `ScatterRecord`
    pub const fn new(attenuation: Color, scattered_ray: Ray) -> Self {
        Self {
            attenuation,
            scattered_ray,
        }
    }
}
