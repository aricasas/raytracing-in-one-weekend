use crate::hittable::HitRecord;
use crate::materials::{Material, ScatterRecord};
use crate::Color;
use crate::Ray;
use crate::Vec3;

#[derive(Clone)]
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
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(&ray.direction.unit_vector(), &record.normal);

        let scatter_record = ScatterRecord::new(
            self.albedo,
            Ray::new(
                record.p,
                reflected + Vec3::random_in_unit_sphere() * self.fuzz,
                ray.time,
            ),
        );

        if Vec3::dot(&scatter_record.scattered_ray.direction, &record.normal) > 0.0 {
            Some(scatter_record)
        } else {
            None
        }
    }
}
