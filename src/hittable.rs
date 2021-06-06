use std::sync::Arc;

use crate::materials::Material;
use crate::surfaces::Aabb;
use crate::Ray;
use crate::Vec3;

/// A struct to store relevant data of a ray intersecting with a surface
pub struct HitRecord {
    /// The value of `t` when hit occurs.
    /// Where `t` is part of the ray formula 'P(t)=A+tb'
    pub t: f64,
    /// The point in space where the hit occurs
    pub p: Vec3,
    pub u: f64,
    pub v: f64,
    /// A normal vector perpendicular to the hit surface.
    pub normal: Vec3,
    /// If the ray hit the surface from outside then it's `true`. If it hit it from the inside, then it's `false`.
    pub front_face: bool,
    /// The `Material` of the surface it hit.
    pub material: Box<dyn Material>,
}
impl HitRecord {
    /// Returns a new `HitRecord`
    pub fn new<T: Material + 'static>(t: f64, p: Vec3, material: T) -> Self {
        Self {
            t,
            p,
            u: 0.0,
            v: 0.0,
            material: Box::new(material),
            front_face: false,
            normal: Vec3::new(0.0, 0.0, 0.0),
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
    pub fn set_texture_coordinates(&mut self, u: f64, v: f64) {
        self.u = u;
        self.v = v;
    }
}

/// A trait that defines any hittable surface or geometry
pub trait Hittable: Send + Sync {
    /// Calculates a ray intersection with a surface
    ///
    /// The intesection only counts if `t_min` < `t` < `t_max`
    /// where `t` is part of the ray formula 'P(t)=A+tb'
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self, time: (f64, f64)) -> Option<Aabb>;
}

/// A list to store hittable surfaces
pub struct HittableList {
    pub surfaces: Vec<Arc<dyn Hittable>>,
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
    pub fn push<T: Hittable + 'static>(&mut self, surface: T) {
        self.surfaces.push(Arc::new(surface));
    }

    pub fn into_vec(self) -> Vec<Arc<dyn Hittable>> {
        self.surfaces
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_distance_so_far = t_max;

        for surface in &self.surfaces {
            let temp_record = surface.hit(ray, t_min, closest_distance_so_far);

            if let Some(intersection) = temp_record {
                closest_distance_so_far = intersection.t;
                hit_record = Some(intersection);
            }
        }

        hit_record
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<Aabb> {
        if self.surfaces.is_empty() {
            return None;
        }

        let first_box = self.surfaces[0].bounding_box(time);

        // If the first surface does have a bounding box then...
        first_box.and_then(|first_box_aabb| {
            // Loop through all the surfaces
            self.surfaces
                .iter()
                .skip(1) // Skipping the first one
                .map(|surface| surface.bounding_box(time)) // Get the bounding boxes
                .try_fold(first_box_aabb, |accumulator_aabb, current_surface| {
                    // Get a big AABB containing all the objects
                    // if every object does have one
                    current_surface.map(|current_surface_aabb| {
                        Aabb::surrounding_box(&accumulator_aabb, &current_surface_aabb)
                    })
                })
        })
        // Returns that whole operation
        // That big AABB or None if something happened to not have an AABB
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}
