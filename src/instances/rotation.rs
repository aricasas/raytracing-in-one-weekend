use crate::{
    hittable::{HitRecord, Hittable},
    surfaces::Aabb,
    Ray, Vec3,
};

#[derive(Clone)]
pub struct RotateY<T: Hittable> {
    surface: T,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Option<Aabb>,
}
impl<T: Hittable> RotateY<T> {
    pub fn new(surface: T, angle: f64) -> Self {
        let (sin_theta, cos_theta) = f64::sin_cos(angle);

        let bounding_box = surface.bounding_box((0.0, 1.0)).map(|b_box| {
            let mut minimum = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
            let mut maximum = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f64 * b_box.maximum.x() + (1 - i) as f64 * b_box.minimum.x();
                        let y = j as f64 * b_box.maximum.y() + (1 - j) as f64 * b_box.minimum.y();
                        let z = k as f64 * b_box.maximum.z() + (1 - k) as f64 * b_box.minimum.z();

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(new_x, y, new_z);

                        for c in 0..3_u8 {
                            minimum[c] = f64::min(minimum[c], tester[c]);
                            maximum[c] = f64::max(maximum[c], tester[c]);
                        }
                    }
                }
            }

            Aabb::new(minimum, maximum)
        });

        Self {
            surface,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }
}
impl<T: Hittable> Hittable for RotateY<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let new_origin = Vec3::new(
            self.cos_theta * ray.origin.x() - self.sin_theta * ray.origin.z(),
            ray.origin.y(),
            self.sin_theta * ray.origin.x() + self.cos_theta * ray.origin.z(),
        );
        let new_direction = Vec3::new(
            self.cos_theta * ray.direction.x() - self.sin_theta * ray.direction.z(),
            ray.direction.y(),
            self.sin_theta * ray.direction.x() + self.cos_theta * ray.direction.z(),
        );

        let rotated_ray = Ray::new(new_origin, new_direction, ray.time);

        let mut hit_record = self.surface.hit(&rotated_ray, t_min, t_max)?;

        let mut rotated_p = hit_record.p;
        let mut rotated_normal = hit_record.normal;

        rotated_p[0] = self.cos_theta * hit_record.p.x() + self.sin_theta * hit_record.p.z();
        rotated_p[2] = -self.sin_theta * hit_record.p.x() + self.cos_theta * hit_record.p.z();

        rotated_normal[0] =
            self.cos_theta * hit_record.normal.x() + self.sin_theta * hit_record.normal.z();
        rotated_normal[2] =
            -self.sin_theta * hit_record.normal.x() + self.cos_theta * hit_record.normal.z();

        hit_record.p = rotated_p;
        hit_record.set_face_normal(&rotated_ray, rotated_normal);

        Some(hit_record)
    }

    fn bounding_box(&self, _time: (f64, f64)) -> Option<Aabb> {
        self.bounding_box.clone()
    }
}

pub trait RotationY {
    fn rotate_y_by(&self, angle: f64) -> RotateY<Self>
    where
        Self: Hittable + Sized + Clone;
}
impl<T: Hittable + Sized + Clone> RotationY for T {
    fn rotate_y_by(&self, angle: f64) -> RotateY<Self>
    where
        Self: Hittable + Sized + Clone,
    {
        RotateY::new(self.clone(), angle)
    }
}
