use std::sync::Arc;

use super::hittable::{HitRecord, Hittable};
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            // No hit
            return HitRecord::new();
        }

        let discriminant_sqrt = discriminant.sqrt();

        // Find root inside the range

        let mut root = (-half_b - discriminant_sqrt) / a;
        if (root < t_min) || (root > t_max) {
            root = (-half_b + discriminant_sqrt) / a;
            if (root < t_min) || (root > t_max) {
                // No hit
                return HitRecord::new();
            }
        }

        let mut record = HitRecord::new();

        record.hit_anything = true;
        record.t = root;
        record.p = ray.at(root);
        record.material = self.material.clone();

        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);

        record
    }
}
