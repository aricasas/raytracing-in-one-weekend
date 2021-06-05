use crate::hittable::HitRecord;
use crate::materials::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::textures::Texture;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Lambertian<T: Texture + Clone> {
    pub albedo: T,
}

/// A material with lambertian reflectance (matte)
impl<T: Texture + Clone> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl<T: Texture + Clone> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        Some(ScatterRecord {
            scattered_ray: Ray::new(record.p, scatter_direction, ray.time),
            attenuation: self.albedo.value(record.u, record.v, &record.p),
        })
    }
}
