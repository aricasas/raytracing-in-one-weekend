use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Lambertian {
    pub albedo: Color,
}

/// A material with lambertian reflectance (matte)
impl Lambertian {
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> ScatterRecord {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        ScatterRecord {
            scattered_ray: Ray::new(record.p, scatter_direction),
            attenuation: self.albedo,
            did_scatter: true,
        }
    }
}
