use super::color::Color;
use super::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Returns the position of the ray when it travels "t" in its direction
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }

    pub fn color(&self) -> Color {
        let t = self.hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5);

        if t > 0.0 {
            let normal = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
            return Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5;
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::linear_blend(t, &Color::new(1.0, 1.0, 1.0), &Color::new(0.5, 0.7, 1.0))
    }

    fn hit_sphere(&self, center: Vec3, radius: f64) -> f64 {
        let oc = self.origin - center;
        let a = Vec3::dot(&self.direction, &self.direction);
        let b = 2.0 * Vec3::dot(&oc, &self.direction);
        let c = Vec3::dot(&oc, &oc) - radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-b - discriminant.sqrt()) / (2.0 * a)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(ray.at(3.0), Vec3::new(0.0, 3.0, 0.0));

        let ray = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.8, 0.6, 0.0));
        assert_eq!(ray.at(5.0), Vec3::new(5.0, 4.0, 1.0));

        let ray = Ray::new(Vec3::new(5.0, 4.0, 1.0), Vec3::new(0.8, 0.6, 0.0));
        assert_eq!(ray.at(-5.0), Vec3::new(1.0, 1.0, 1.0));
    }
}
