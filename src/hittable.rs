use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Clone)]
pub struct HitRecord {
    /// The value of 't' when hit occurs
    pub t: f64,
    /// The point in which the hit occurs
    pub p: Vec3,
    pub normal: Vec3,
    /// If the ray hit the surface from outside then it's true. If it hit it from the inside, then it's false
    pub front_face: bool,
}
impl HitRecord {
    pub const fn new() -> Self {
        Self {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: true,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&ray.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;

                // Write 'temp_record' into 'record'
                record.t = temp_record.t;
                record.p = temp_record.p;
                record.normal = temp_record.normal;
                record.front_face = temp_record.front_face;
            }
        }

        hit_anything
    }
}
