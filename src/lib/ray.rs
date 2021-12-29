use crate::hittable::Hittable;
use crate::Color;
use crate::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub const fn new(origin: Vec3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    /// Returns the position of the ray when it travels `t` in its direction.
    /// Based on the ray formula 'P(t)=A+tb'
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }

    /// Calculates the final color of the ray
    #[allow(clippy::missing_panics_doc)] // We check for nones, so unwrapping will never fail
    pub fn calculate_color<T: Hittable>(&self, world: &T, background: &Color, depth: u32) -> Color {
        // If ray has bounced too many times
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let hit_record = world.hit(self, 0.0001, f64::INFINITY);

        if hit_record.is_none() {
            return *background;
        }

        let intersection = hit_record.unwrap();
        let scatter_record = intersection.material.scatter(self, &intersection);
        let emitted =
            intersection
                .material
                .emitted(intersection.u, intersection.v, &intersection.p);

        if scatter_record.is_none() {
            return emitted;
        }

        let scatter = scatter_record.unwrap();

        emitted
            + scatter.attenuation
                * scatter
                    .scattered_ray
                    .calculate_color(world, background, depth - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 0.0);
        assert_eq!(ray.at(3.0), Vec3::new(0.0, 3.0, 0.0));

        let ray = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.8, 0.6, 0.0), 0.0);
        assert_eq!(ray.at(5.0), Vec3::new(5.0, 4.0, 1.0));

        let ray = Ray::new(Vec3::new(5.0, 4.0, 1.0), Vec3::new(0.8, 0.6, 0.0), 0.0);
        assert_eq!(ray.at(-5.0), Vec3::new(1.0, 1.0, 1.0));
    }
}
