use std::f64::consts::{PI, TAU};

use crate::{
    hittable::{HitRecord, Hittable},
    materials::Material,
    Ray, Vec3,
};

use super::Aabb;

#[derive(Clone)]
pub struct Bowl<T: Material + Clone + 'static> {
    radius: f64,
    curvature: f64,
    material: T,
}

impl<T: Material + Clone + 'static> Bowl<T> {
    pub fn new(radius: f64, curvature: f64, material: T) -> Self {
        Self {
            radius,
            curvature,
            material,
        }
    }

    fn get_uv(&self, hit: Vec3, dist_from_origin: f64) -> (f64, f64) {
        // A map of the Earth should end up looking kind of like the UN logo
        (
            1.0 - (f64::atan2(hit.z(), hit.x()) + PI) / TAU,
            // This one is incorrect, it maps the v coordinate linearly depending
            // on how far it is from the center divided by the radius. But since
            // the surface is curved, this should be calculated in a different way.
            1.0 - dist_from_origin / self.radius,
        )
    }
}

impl<T: Material + Clone + 'static> Hittable for Bowl<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let d = self.curvature
            * (ray.direction.x() * ray.direction.x() + ray.direction.z() * ray.direction.z());
        let e = 2.0
            * self.curvature
            * (ray.origin.x() * ray.direction.x() + ray.origin.z() * ray.direction.z())
            - ray.direction.y();
        let f = self.curvature
            * (ray.origin.x() * ray.origin.x() + ray.origin.z() * ray.origin.z())
            - ray.origin.y();

        let discriminant = e * e - 4.0 * d * f;

        if discriminant < 0.0 {
            return None;
        }

        let disc_sqrt = discriminant.sqrt();

        let root1 = (-e - disc_sqrt) / (2.0 * d);
        let root2 = (-e + disc_sqrt) / (2.0 * d);

        let solve = |root| {
            let hit = ray.at(root);

            let dist_from_origin_squared = hit.x() * hit.x() + hit.z() * hit.z();

            if dist_from_origin_squared > self.radius * self.radius {
                return None;
            }

            let mut record = HitRecord::new(root, hit, self.material.clone());

            let dist_from_origin = dist_from_origin_squared.sqrt();
            let tangent_slope = 2.0 * self.curvature * dist_from_origin;
            let normal_slope = -1.0 / tangent_slope;

            let outward_normal =
                Vec3::new(-hit.x(), normal_slope * dist_from_origin, -hit.z()).unit_vector();

            record.set_face_normal(ray, outward_normal);

            let (u, v) = self.get_uv(hit, dist_from_origin);
            record.set_texture_coordinates(u, v);

            Some(record)
        };

        match (
            (root1 > t_min) && (root1 < t_max),
            (root2 > t_min) && (root2 < t_max),
        ) {
            (false, false) => None,
            (true, false) => solve(root1),
            (false, true) => solve(root2),
            (true, true) => solve(root1).or_else(|| solve(root2)),
        }
    }

    fn bounding_box(&self, _time: (f64, f64)) -> Option<Aabb> {
        let max_height = self.curvature * self.radius * self.radius;
        Some(Aabb::new(
            Vec3::new(-self.radius, 0.0, -self.radius),
            Vec3::new(self.radius, max_height, self.radius),
        ))
    }
}
