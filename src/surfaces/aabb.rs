use crate::ray::Ray;
use crate::vec3::Vec3;

/// Axis-aligned bounding box
#[derive(Clone, Debug)]
pub struct Aabb {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl Aabb {
    pub const fn new(minimum: Vec3, maximum: Vec3) -> Self {
        Self { minimum, maximum }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        // Optimized version by Andrew Kensler

        let mut t_min = t_min;
        let mut t_max = t_max;

        for a in 0_u8..3 {
            let inverse_direction = 1.0 / ray.direction[a];

            let mut t0 = (self.minimum[a] - ray.origin[a]) * inverse_direction;
            let mut t1 = (self.maximum[a] - ray.origin[a]) * inverse_direction;

            if inverse_direction < 0.0 {
                std::mem::swap(&mut t0, &mut t1)
            };

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: &Self, box1: &Self) -> Self {
        let min = Vec3::new(
            f64::min(box0.minimum.x(), box1.minimum.x()),
            f64::min(box0.minimum.y(), box1.minimum.y()),
            f64::min(box0.minimum.z(), box1.minimum.z()),
        );
        let max = Vec3::new(
            f64::max(box0.maximum.x(), box1.maximum.x()),
            f64::max(box0.maximum.y(), box1.maximum.y()),
            f64::max(box0.maximum.z(), box1.maximum.z()),
        );

        Self::new(min, max)
    }
}

impl std::cmp::PartialEq for Aabb {
    fn eq(&self, other: &Self) -> bool {
        (self.minimum == other.minimum) && (self.maximum == other.maximum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surrounding_box() {
        let aabb1 = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let aabb2 = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let aabb3 = Aabb::new(Vec3::new(-4.0, -1.5, -0.5), Vec3::new(0.5, 2.0, 0.0));

        assert_eq!(Aabb::surrounding_box(&aabb1, &aabb1), aabb1);
        assert_eq!(Aabb::surrounding_box(&aabb1, &aabb2), aabb2);
        assert_eq!(Aabb::surrounding_box(&aabb1, &aabb3), aabb3);
        assert_eq!(
            Aabb::surrounding_box(&aabb2, &aabb3),
            Aabb::new(Vec3::new(-4.0, -1.5, -1.0), Vec3::new(1.0, 2.0, 1.0))
        );
    }
}
