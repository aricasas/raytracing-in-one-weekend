use super::color::Color;
use super::hittable::HitRecord;
use super::ray::Ray;
use super::vec3::Vec3;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

/// A trait that all materials must implement
///
/// Defines the scattering behaviour of that material
pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterRecord;
}

/// A struct to store relevant data of a ray scattering off something
pub struct ScatterRecord {
    pub did_scatter: bool,
    pub attenuation: Color,
    /// The scattered `Ray` with its new direction
    pub scattered_ray: Ray,
}
impl ScatterRecord {
    /// Returns a new `ScatterRecord` with empty values meant to be replaced
    pub const fn new() -> Self {
        Self {
            did_scatter: false,
            attenuation: Color::new(0.0, 0.0, 0.0),
            scattered_ray: Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
        }
    }
}
