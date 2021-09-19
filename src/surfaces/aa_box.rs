use crate::{
    hittable::{Hittable, HittableList},
    materials::Material,
    Vec3,
};

use super::{Aabb, XYRect, XZRect, YZRect};

#[derive(Clone)]
pub struct AABox {
    box_min: Vec3,
    box_max: Vec3,
    sides: HittableList,
}
impl AABox {
    pub fn new<T: Material + Clone + 'static>(p0: Vec3, p1: Vec3, material: T) -> Self {
        let box_min = p0;
        let box_max = p1;

        let mut sides = HittableList::new();
        sides.push(XYRect::new(
            (p0.x(), p1.x()),
            (p0.y(), p1.y()),
            p0.z(),
            material.clone(),
        ));
        sides.push(XYRect::new(
            (p0.x(), p1.x()),
            (p0.y(), p1.y()),
            p1.z(),
            material.clone(),
        ));

        sides.push(XZRect::new(
            (p0.x(), p1.x()),
            (p0.z(), p1.z()),
            p1.y(),
            material.clone(),
        ));
        sides.push(XZRect::new(
            (p0.x(), p1.x()),
            (p0.z(), p1.z()),
            p1.y(),
            material.clone(),
        ));

        sides.push(YZRect::new(
            (p0.y(), p1.y()),
            (p0.z(), p1.z()),
            p1.x(),
            material.clone(),
        ));
        sides.push(YZRect::new(
            (p0.y(), p1.y()),
            (p0.z(), p1.z()),
            p1.x(),
            material,
        ));

        Self {
            box_min,
            box_max,
            sides,
        }
    }
}
impl Hittable for AABox {
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time: (f64, f64)) -> Option<super::Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }
}
