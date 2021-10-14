use crate::{hittable::HitRecord, textures::Texture, Ray, Vec3};

use super::{Material, ScatterRecord};

#[derive(Clone)]
pub struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(texture: T) -> Self {
        Self { albedo: texture }
    }
}
impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterRecord> {
        let scattered_ray = Ray::new(record.p, Vec3::random_in_unit_sphere(), ray.time);
        let attenuation = self.albedo.value(record.u, record.v, &record.p);

        Some(ScatterRecord::new(attenuation, scattered_ray))
    }
}
