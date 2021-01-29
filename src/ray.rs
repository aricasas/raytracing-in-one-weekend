use super::color::Color;
use super::hittable::{Hittable, HittableList};
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

    pub fn calculate_color(ray: &Self, world: &HittableList, depth: u32) -> Color {
        // If ray has bounced too many times
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let record = world.hit(ray, 0.0001, f64::INFINITY);

        if record.hit_anything {
            let scatter_record = record.material.scatter(ray, &record);
            if scatter_record.scattered {
                return scatter_record.attenuation
                    * Self::calculate_color(&scatter_record.scattered_ray, world, depth - 1);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        // If no hits
        let unit_direction = ray.direction.unit_vector();
        Color::linear_blend(
            0.5 * (unit_direction.y() + 1.0),
            &Color::new(1.0, 1.0, 1.0),
            &Color::new(0.5, 0.7, 1.0),
        )
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
