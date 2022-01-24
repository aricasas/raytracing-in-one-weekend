use crate::{
    hittable::{HitRecord, Hittable},
    materials::Material,
    Ray, Vec3,
};

use super::Aabb;

#[derive(Clone)]
pub struct ParabolaX<T: Material + Clone + 'static> {
    x_range: (f64, f64),
    z_range: (f64, f64),
    quadratic_params: (f64, f64, f64),
    material: T,
}

impl<T: Material + Clone + 'static> ParabolaX<T> {
    pub fn new(
        x_range: (f64, f64),
        z_range: (f64, f64),
        quadratic_params: (f64, f64, f64),
        material: T,
    ) -> Self {
        Self {
            x_range,
            z_range,
            quadratic_params,
            material,
        }
    }

    fn get_uv(&self, hit: &Vec3) -> (f64, f64) {
        let (min_x, max_x) = self.x_range;
        let (min_z, max_z) = self.z_range;

        (
            // This one is incorrect, the u coordinate isn't taking
            // into account that the surface is curved
            (hit.x() - min_x) / (max_x - min_x),
            (max_z - hit.z()) / (max_z - min_z),
        )
    }

    fn at(&self, x: f64) -> f64 {
        let (a, b, c) = self.quadratic_params;
        a * x * x + b * x + c
    }
    fn derivative(&self, x: f64) -> f64 {
        let (a, b, _) = self.quadratic_params;
        2.0 * a * x + b
    }
    fn find_vertex(&self) -> f64 {
        let (a, b, _) = self.quadratic_params;

        -b / (2.0 * a)
    }
}

impl<T: Material + Clone + 'static> Hittable for ParabolaX<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (a, b, c) = self.quadratic_params;
        let d = a * ray.direction.x() * ray.direction.x();
        let e = (2.0 * a * ray.origin.x() + b) * ray.direction.x() - ray.direction.y();
        let f = (a * ray.origin.x() + b) * ray.origin.x() + c - ray.origin.y();

        let double_d = 2.0 * d;

        let discriminant = e * e - 2.0 * double_d * f;

        if discriminant < 0.0 {
            return None;
        }

        let disc_sqrt = discriminant.sqrt();

        let root1 = (-e - disc_sqrt) / double_d;
        let root2 = (-e + disc_sqrt) / double_d;

        let solve = |root| {
            let hit = ray.at(root);

            if hit.x() < self.x_range.0
                || hit.x() > self.x_range.1
                || hit.z() < self.z_range.0
                || hit.z() > self.z_range.1
            {
                return None;
            }

            let mut record = HitRecord::new(root, hit, self.material.clone());

            let tangent_slope = self.derivative(hit.x());
            let normal_slope = -1.0 / tangent_slope;

            let outward_normal = Vec3::new(1.0, normal_slope, 0.0).unit_vector();
            record.set_face_normal(ray, outward_normal);

            let (u, v) = self.get_uv(&hit);
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
        // Since this is a parabola, the lowest and highest points in a certain range must be the vertex,
        // or one of the edges of the range
        let y_at_vertex = self.at(self.find_vertex());
        let y_at_min_x_range = self.at(self.x_range.0);
        let y_at_max_x_range = self.at(self.x_range.1);

        let min_y = y_at_vertex.min(y_at_min_x_range).min(y_at_max_x_range);
        let max_y = y_at_vertex.max(y_at_min_x_range).max(y_at_max_x_range);

        Some(Aabb::new(
            Vec3::new(self.x_range.0, min_y, self.z_range.0),
            Vec3::new(self.x_range.1, max_y, self.z_range.1),
        ))
    }
}
