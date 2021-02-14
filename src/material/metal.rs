use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterRecord {
        let reflected = Vec3::reflect(&ray.direction.unit_vector(), &record.normal);

        let mut scatter_record = ScatterRecord::new();

        scatter_record.scattered_ray = Ray::new(
            record.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
        );
        scatter_record.attenuation = self.albedo;
        scatter_record.did_scatter =
            Vec3::dot(&scatter_record.scattered_ray.direction, &record.normal) > 0.0;

        scatter_record
    }
}
