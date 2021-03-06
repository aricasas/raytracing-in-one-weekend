use rand::Rng;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub const fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);

        // r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
        (1.0 - r0).mul_add((1.0 - cosine).powi(5), r0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.unit_vector();
        let cos_theta = Vec3::dot(&-unit_direction, &record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;

        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen::<f64>()
        {
            Vec3::reflect(&unit_direction, &record.normal)
        } else {
            Vec3::refract(&unit_direction, &record.normal, refraction_ratio)
        };

        let scatter_record =
            ScatterRecord::new(Color::new(1.0, 1.0, 1.0), Ray::new(record.p, direction));

        Some(scatter_record)
    }
}
