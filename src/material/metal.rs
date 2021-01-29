use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterRecord {
        let reflected = Vec3::reflect(&ray.direction.unit_vector(), &record.normal);

        let mut scatter_record = ScatterRecord::new();

        scatter_record.scattered_ray = Ray::new(record.p, reflected);
        scatter_record.attenuation = self.albedo;
        scatter_record.scattered =
            Vec3::dot(&scatter_record.scattered_ray.direction, &record.normal) > 0.0;

        scatter_record
    }
}
