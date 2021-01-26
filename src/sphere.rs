use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use super::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub const fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let discriminant_sqrt = discriminant.sqrt();

        // Find root inside the range

        let mut root = (-half_b - discriminant_sqrt) / a;
        if (root < t_min) || (root > t_max) {
            root = (-half_b + discriminant_sqrt) / a;
            if (root < t_min) || (root > t_max) {
                return false;
            }
        }

        record.t = root;
        record.p = ray.at(root);
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);

        true
    }
}
