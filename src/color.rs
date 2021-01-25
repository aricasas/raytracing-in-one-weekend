use super::vec3::Vec3;
use std::ops;

#[derive(Debug)]
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

    pub fn linear_blend(t: f64, start: &Self, end: &Self) -> Self {
        (start * (1.0 - t)) + (end * (t))
    }

    pub fn write(&self) {
        println!(
            "{} {} {}",
            (255.0 * self.r()) as u32,
            (255.0 * self.g()) as u32,
            (255.0 * self.b()) as u32
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
