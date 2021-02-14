use std::sync::Arc;

use super::color::Color;
use super::material::lambertian::Lambertian;
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;

/// A struct to store relevant data of a ray intersecting with a surface
pub struct HitRecord {
    /// `true` if the ray actually hit something
    pub hit_anything: bool,
    /// The value of `t` when hit occurs.
    /// Where `t` is part of the ray formula 'P(t)=A+tb'
    pub t: f64,
    /// The point in space where the hit occurs
    pub p: Vec3,
    /// A normal vector perpendicular to the hit surface.
    pub normal: Vec3,
    /// If the ray hit the surface from outside then it's `true`. If it hit it from the inside, then it's `false`.
    pub front_face: bool,
    /// The `Material` of the surface it hit.
    pub material: Arc<dyn Material + Send + Sync>,
}
impl HitRecord {
    /// Returns a new `HitRecord` with nonsense values meant to be replaced
    pub fn new() -> Self {
        Self {
            hit_anything: false,
            front_face: true,
            p: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: Arc::new(Lambertian {
                albedo: Color::new(0.0, 0.0, 0.0),
            }),
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

/// A trait that defines any hittable surface or geometry
pub trait Hittable {
    /// Calculates a ray intesction with a surface
    ///
    /// The intesection only counts if `t_min` < `t` < `t_max`
    /// where `t` is part of the ray formula 'P(t)=A+tb'
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord;
}

/// A list to store hittable surfaces
pub struct HittableList {
    surfaces: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    /// Returns an empty `HittableList`
    pub fn new() -> Self {
        Self {
            surfaces: Vec::new(),
        }
    }

    /// Gets rid of all the objects inside the `HittableList`
    pub fn clear(&mut self) {
        self.surfaces.clear();
    }
    /// Push a new `Hittable` into the list
    pub fn push(&mut self, surface: Box<dyn Hittable + Send + Sync>) {
        self.surfaces.push(surface);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord {
        let mut hit_record = HitRecord::new();
        let mut closest_distance_so_far = t_max;

        for surface in &self.surfaces {
            let temp_record = surface.hit(ray, t_min, closest_distance_so_far);

            if temp_record.hit_anything {
                closest_distance_so_far = temp_record.t;

                hit_record = temp_record;
            }
        }

        hit_record
    }
}
