use rand::Rng;
use std::ops;

use super::utilities;
use super::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Color(Vec3);

impl Color {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vec3::new(x, y, z))
    }

    pub const fn r(&self) -> f64 {
        self.0.x()
    }
    pub const fn g(&self) -> f64 {
        self.0.y()
    }
    pub const fn b(&self) -> f64 {
        self.0.z()
    }
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        Self::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn linear_blend(t: f64, start: &Self, end: &Self) -> Self {
        (start * (1.0 - t)) + (end * (t))
    }

    pub fn to_writeable_ints(&self, samples_per_pixel: u32) -> [u8; 3] {
        let samples_per_pixel = f64::from(samples_per_pixel);

        // Divide the color by the number of samples and gamma-correct for gamma=2.0
        let r = (self.r() / samples_per_pixel).sqrt();
        let g = (self.g() / samples_per_pixel).sqrt();
        let b = (self.b() / samples_per_pixel).sqrt();

        let r = (255.0 * utilities::clamp(r, 0.0, 0.999)).round() as u8;
        let g = (255.0 * utilities::clamp(g, 0.0, 0.999)).round() as u8;
        let b = (255.0 * utilities::clamp(b, 0.0, 0.999)).round() as u8;

        [r, g, b]
    }

    pub fn write(&self, samples_per_pixel: u32) {
        let color_values = self.to_writeable_ints(samples_per_pixel);

        println!(
            "{} {} {}",
            color_values[0], color_values[1], color_values[2]
        );
    }
}

// Addition
impl ops::Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.r() + other.r(),
            self.g() + other.g(),
            self.b() + other.b(),
        )
    }
}
impl ops::Add for &Color {
    type Output = Color;
    fn add(self, other: Self) -> Color {
        Color::new(
            self.r() + other.r(),
            self.g() + other.g(),
            self.b() + other.b(),
        )
    }
}
impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl std::iter::Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0.0, 0.0, 0.0), |a, b| a + b)
    }
}

// Multiplication
impl ops::Mul for Color {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(
            self.r() * other.r(),
            self.g() * other.g(),
            self.b() * other.b(),
        )
    }
}
impl ops::Mul for &Color {
    type Output = Color;
    fn mul(self, other: Self) -> Color {
        Color::new(
            self.r() * other.r(),
            self.g() * other.g(),
            self.b() * other.b(),
        )
    }
}
impl ops::Mul<f64> for Color {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self::new(self.r() * other, self.g() * other, self.b() * other)
    }
}
impl ops::Mul<f64> for &Color {
    type Output = Color;
    fn mul(self, other: f64) -> Color {
        Color::new(self.r() * other, self.g() * other, self.b() * other)
    }
}

// Tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_write() {
        Color::new(0.0, 0.3, 0.1).write(1);
        Color::new(0.5, 0.4, 0.9).write(1);
        Color::new(1.0, 0.5, 2.0).write(1);
    }
}
