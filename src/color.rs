use super::vec3::Vec3;

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
        Self::new(
            (1.0 - t) * start.r() + (t * end.r()),
            (1.0 - t) * start.g() + (t * end.g()),
            (1.0 - t) * start.b() + (t * end.b()),
        )
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
