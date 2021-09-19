use super::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::Ray;
use crate::Vec3;

#[derive(Clone)]
pub struct XYRect<T: Material + Clone + 'static> {
    x: (f64, f64),
    y: (f64, f64),
    k: f64,
    material: T,
}
impl<T: Material + Clone + 'static> XYRect<T> {
    pub fn new(x: (f64, f64), y: (f64, f64), k: f64, material: T) -> Self {
        Self { x, y, k, material }
    }
}
impl<T: Material + Clone + 'static> Hittable for XYRect<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z()) / ray.direction.z();
        if t < t_min || t > t_max {
            return None;
        }

        let hit_point = ray.at(t);

        if hit_point.x() < self.x.0
            || hit_point.x() > self.x.1
            || hit_point.y() < self.y.0
            || hit_point.y() > self.y.1
        {
            return None;
        }

        let mut record = HitRecord::new(t, hit_point, self.material.clone());
        record.u = (hit_point.x() - self.x.0) / (self.x.1 - self.x.0);
        record.v = (hit_point.y() - self.y.0) / (self.y.1 - self.y.0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        record.set_face_normal(ray, outward_normal);

        Some(record)
    }

    fn bounding_box(&self, _time: (f64, f64)) -> Option<Aabb> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        Some(Aabb::new(
            Vec3::new(self.x.0, self.y.0, self.k - 0.0001),
            Vec3::new(self.x.1, self.y.1, self.k + 0.0001),
        ))
    }
}

#[derive(Clone)]
pub struct XZRect<T: Material + Clone + 'static> {
    x: (f64, f64),
    z: (f64, f64),
    k: f64,
    material: T,
}
impl<T: Material + Clone + 'static> XZRect<T> {
    pub fn new(x: (f64, f64), z: (f64, f64), k: f64, material: T) -> Self {
        Self { x, z, k, material }
    }
}
impl<T: Material + Clone + 'static> Hittable for XZRect<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y()) / ray.direction.y();
        if t < t_min || t > t_max {
            return None;
        }

        let hit_point = ray.at(t);

        if hit_point.x() < self.x.0
            || hit_point.x() > self.x.1
            || hit_point.z() < self.z.0
            || hit_point.z() > self.z.1
        {
            return None;
        }

        let mut record = HitRecord::new(t, hit_point, self.material.clone());
        record.u = (hit_point.x() - self.x.0) / (self.x.1 - self.x.0);
        record.v = (hit_point.z() - self.z.0) / (self.z.1 - self.z.0);
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        record.set_face_normal(ray, outward_normal);

        Some(record)
    }

    fn bounding_box(&self, _time: (f64, f64)) -> Option<Aabb> {
        // The bounding box must have non-zero width in each dimension, so pad the Y
        // dimension a small amount.
        Some(Aabb::new(
            Vec3::new(self.x.0, self.k - 0.0001, self.z.0),
            Vec3::new(self.x.1, self.k + 0.0001, self.z.1),
        ))
    }
}

#[derive(Clone)]
pub struct YZRect<T: Material + Clone + 'static> {
    y: (f64, f64),
    z: (f64, f64),
    k: f64,
    material: T,
}
impl<T: Material + Clone + 'static> YZRect<T> {
    pub fn new(y: (f64, f64), z: (f64, f64), k: f64, material: T) -> Self {
        Self { y, z, k, material }
    }
}
impl<T: Material + Clone + 'static> Hittable for YZRect<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x()) / ray.direction.x();
        if t < t_min || t > t_max {
            return None;
        }

        let hit_point = ray.at(t);

        if hit_point.y() < self.y.0
            || hit_point.y() > self.y.1
            || hit_point.z() < self.z.0
            || hit_point.z() > self.z.1
        {
            return None;
        }

        let mut record = HitRecord::new(t, hit_point, self.material.clone());
        record.u = (hit_point.y() - self.y.0) / (self.y.1 - self.y.0);
        record.v = (hit_point.z() - self.z.0) / (self.z.1 - self.z.0);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        record.set_face_normal(ray, outward_normal);

        Some(record)
    }

    fn bounding_box(&self, _time: (f64, f64)) -> Option<Aabb> {
        // The bounding box must have non-zero width in each dimension, so pad the X
        // dimension a small amount.
        Some(Aabb::new(
            Vec3::new(self.k - 0.0001, self.y.0, self.z.0),
            Vec3::new(self.k + 0.0001, self.y.1, self.z.1),
        ))
    }
}
