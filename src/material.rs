use super::color::Color;
use super::hittable::HitRecord;
use super::ray::Ray;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

/// A trait that all materials must implement
///
/// Defines the scattering behaviour of that material
pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterRecord>;
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
