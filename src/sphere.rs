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
        // Since a sphere is a quadratic equation, we can solve it
        // using the quadratic formula.
        // For this ray intersection it's actually a version that's a bit simplified
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        // Since we are solving this using the quadratic formula,
        // we can check if there are solutions using the discriminant test
        //
        // https://www.khanacademy.org/math/algebra/x2f8bb11595b61c86:quadratic-functions-equations/x2f8bb11595b61c86:quadratic-formula-a1/a/discriminant-review
        //
        // if (d < 0) => No real solution (ray doesn't hit the sphere)
        // if (d == 0) => One real solution (ray hits sphere on only one point.
        //                                   This can happen if the ray is
        //                                   tangent to the sphere)
        // if (d > 0) => Two distinct real solutions (ray hits sphere on two points)
        if discriminant < 0.0 {
            // No hit. Or ray is tangent to the sphere, but we ignore those
            return HitRecord::new();
        }
        // Hit!

        // Find solution inside the range using the quadratic formula
        // We are testing both solutions and seeing if at least one matches
        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discriminant_sqrt) / a;

        if (root < t_min) || (root > t_max) {
            root = (-half_b + discriminant_sqrt) / a;
            if (root < t_min) || (root > t_max) {
                // No hit within bounds
                return HitRecord::new();
            }
        }

        let mut record = HitRecord::new();

        record.hit_anything = true;
        record.t = root;
        record.p = ray.at(record.t);
        record.material = self.material.clone();

        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);

        record
    }
}
