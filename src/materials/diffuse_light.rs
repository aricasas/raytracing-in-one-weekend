use std::sync::Arc;

use crate::hittable::HitRecord;
use crate::materials::{Material, ScatterRecord};
use crate::textures::Texture;
use crate::Color;
use crate::Ray;
use crate::Vec3;

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    pub emit: Arc<T>,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> Self {
        Self {
            emit: Arc::new(emit),
        }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, u: f64, v: f64, point: &Vec3) -> Color {
        self.emit.value(u, v, point)
    }
}
