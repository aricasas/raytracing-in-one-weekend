use super::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Clone)]
pub struct MovingSphere<T: Material + Clone + 'static> {
    center: (Vec3, Vec3),
    radius: f64,
    material: T,
    time: (f64, f64),
}

impl<T: Material + Clone + 'static> MovingSphere<T> {
    pub fn new(center: (Vec3, Vec3), radius: f64, material: T, time: (f64, f64)) -> Self {
        Self {
            center,
            radius,
            material,
            time,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center.0
            + (self.center.1 - self.center.0) * ((time - self.time.0) / (self.time.1 - self.time.0))
    }
}

impl<T: Material + Clone + 'static> Hittable for MovingSphere<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Since a sphere is a quadratic equation, we can solve it
        // using the quadratic formula.
        // For this ray intersection it's actually a version that's a bit simplified
        let oc = ray.origin - self.center(ray.time);
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

        let outward_normal = (record.p - self.center(ray.time)) / self.radius;
        record.set_face_normal(ray, outward_normal);

        Some(record)
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center(time.0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time.0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center(time.1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time.1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(Aabb::surrounding_box(&box0, &box1))
    }
}
