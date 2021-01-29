use super::color::Color;
use super::hittable::HitRecord;
use super::ray::Ray;
use super::vec3::Vec3;

pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterRecord;
}

pub struct ScatterRecord {
    pub scattered: bool,
    pub attenuation: Color,
    pub scattered_ray: Ray,
}
impl ScatterRecord {
    pub const fn new() -> Self {
        Self {
            scattered: false,
            attenuation: Color::new(0.0, 0.0, 0.0),
            scattered_ray: Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
        }
    }
}
