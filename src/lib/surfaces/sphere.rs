use super::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::Ray;
use crate::Vec3;
use std::f64::consts::{PI, TAU};

#[derive(Clone)]
pub struct Sphere<T: Material + Clone + 'static> {
    center: Vec3,
    radius: f64,
    material: T,
}

impl<T: Material + Clone + 'static> Sphere<T> {
    pub fn new(center: Vec3, radius: f64, material: T) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    /// Return texture coordinates (u, v) from a point in a unit sphere
    ///
    /// # Arguments
    /// * `p` - a given point on the sphere of radius one, centered at the origin.
    ///
    /// # Returns
    /// (u: f64, v: f64)
    /// * `u` - returned value \[0,1\] of angle around the Y axis from X=-1.
    /// * `v` - returned value \[0,1\] of angle from Y=-1 to Y=+1.
    fn get_sphere_uv(p: Vec3) -> (f64, f64) {
        let theta = f64::acos(-p.y());
        let phi = f64::atan2(-p.z(), p.x()) + PI;

        let u = phi / TAU;
        let v = theta / PI;

        (u, v)
    }
}

impl<T: Material + Clone + 'static> Hittable for Sphere<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
            return None;
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
                return None;
            }
        }

        let mut record = HitRecord::new(root, ray.at(root), self.material.clone());

        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);

        let (u, v) = Self::get_sphere_uv(outward_normal);
        record.set_texture_coordinates(u, v);

        Some(record)
    }

    fn bounding_box(&self, _time: (f64, f64)) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}
