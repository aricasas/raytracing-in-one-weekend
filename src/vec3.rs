#[derive(Debug)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
    pub const fn x(&self) -> f64 {
        self.0
    }
    pub const fn y(&self) -> f64 {
        self.1
    }
    pub const fn z(&self) -> f64 {
        self.2
    }
    pub const fn r(&self) -> f64 {
        self.0
    }
    pub const fn g(&self) -> f64 {
        self.1
    }
    pub const fn b(&self) -> f64 {
        self.2
    }
    pub fn length_squared(&self) -> f64 {
        (self.0.powi(2)) + (self.1.powi(2)) + (self.2.powi(2))
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    // pub fn unit_vector(&self) -> Self {
    //
    // }
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        // (u.0 * v.0) + (u.1 * v.1) + (u.2 * v.2)
        u.0.mul_add(v.0, u.1.mul_add(v.1, u.2 * v.2))
    }
    pub fn cross(u: &Self, v: &Self) -> Self {
        Self::new(
            u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
            u.0 * v.1 - u.1 * v.0,
        )
    }
}

// TODO
// Implement operator overloads
