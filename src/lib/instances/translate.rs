use crate::{
    hittable::{HitRecord, Hittable},
    surfaces::Aabb,
    Ray, Vec3,
};

#[derive(Clone)]
pub struct Translate<T: Hittable> {
    surface: T,
    offset: Vec3,
}
impl<T: Hittable> Translate<T> {
    pub fn new(surface: T, offset: Vec3) -> Self {
        Self { surface, offset }
    }
}
impl<T: Hittable> Hittable for Translate<T> {
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        self.surface
            .hit(&moved_ray, t_min, t_max)
            .map(|mut hit_record| {
                hit_record.p += self.offset;
                hit_record.set_face_normal(&moved_ray, hit_record.normal);
                hit_record
            })
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<Aabb> {
        self.surface
            .bounding_box(time)
            .map(|bb| Aabb::new(bb.minimum + self.offset, bb.maximum + self.offset))
    }
}

pub trait Translation {
    fn translate_by(&self, offset: Vec3) -> Translate<Self>
    where
        Self: Hittable + Sized + Clone;
}
impl<T: Hittable + Sized + Clone> Translation for T {
    fn translate_by(&self, offset: Vec3) -> Translate<Self> {
        Translate::new(self.clone(), offset)
    }
}
