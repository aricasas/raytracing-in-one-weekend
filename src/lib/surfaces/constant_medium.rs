use rand::Rng;

use super::Aabb;
use crate::hittable::HitRecord;
use crate::materials::Isotropic;
use crate::{hittable::Hittable, materials::Material, textures::Texture};
use crate::{Ray, Vec3};

#[derive(Clone)]
pub struct ConstantMedium<T: Hittable, G: Material + Clone> {
    boundary: T,
    phase_function: G,
    neg_inv_density: f64,
}

impl<T: Hittable, S: Texture> ConstantMedium<T, Isotropic<S>> {
    pub fn new(boundary: T, texture: S, density: f64) -> Self {
        Self {
            boundary,
            phase_function: Isotropic::new(texture),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl<T: Hittable, G: 'static + Material + Clone> Hittable for ConstantMedium<T, G> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec1 = self.boundary.hit(ray, f64::NEG_INFINITY, f64::INFINITY)?;
        let mut rec2 = self.boundary.hit(ray, rec1.t + 0.0001, f64::INFINITY)?;

        rec1.t = f64::max(rec1.t, t_min);
        rec2.t = f64::min(rec2.t, t_max);

        if rec1.t >= rec2.t {
            return None;
        }

        rec1.t = f64::max(rec1.t, 0.0);

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * f64::ln(rand::thread_rng().gen());

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let rec_t = rec1.t + hit_distance / ray_length;
        let mut record = HitRecord::new(rec_t, ray.at(rec_t), self.phase_function.clone());
        record.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        record.front_face = true;

        Some(record)
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<Aabb> {
        self.boundary.bounding_box(time)
    }
}
