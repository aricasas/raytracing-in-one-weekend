use super::vec3::Vec3;

#[derive(Debug)]
pub struct Color(Vec3);

impl Color {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vec3::new(x, y, z))
    }

    pub const fn r(&self) -> f64 {
        self.0.r()
    }
    pub const fn g(&self) -> f64 {
        self.0.g()
    }
    pub const fn b(&self) -> f64 {
        self.0.b()
    }
}
