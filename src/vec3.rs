use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::ops;

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}
impl Distribution<Axis> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        match rng.gen_range(0..=2) {
            0 => Axis::X,
            1 => Axis::Y,
            _ => Axis::Z,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
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

    /// Returns true if the vector is close to zero in all dimensions
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x().abs() < s) && (self.y().abs() < s) && (self.z().abs() < s)
    }

    /// Returns a `Vec3` with random x, y, and z values within the range [0,1)
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        Self::new(rng.gen(), rng.gen(), rng.gen())
    }
    /// Returns a `Vec3` with random x, y, and z values within the range specified
    pub fn random_min_max(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();

        Self::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_min_max(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();

        loop {
            let p = Self::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    /// Random `Vec3` with length 1
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn reflect(v: &Self, n: &Self) -> Self {
        v - &(n * Self::dot(v, n) * 2.0)
    }

    pub fn refract(uv: &Self, normal: &Self, etai_over_etat: f64) -> Self {
        let cos_theta: f64 = Self::dot(&-uv, normal).min(1.0);
        let r_out_perpendicular = (uv + &(normal * cos_theta)) * etai_over_etat;
        let r_out_parallel = normal * -((1.0 - r_out_perpendicular.length_squared()).abs().sqrt());

        r_out_perpendicular + r_out_parallel
    }

    pub fn length_squared(&self) -> f64 {
        // (self.x() ^ 2) + (self.y() ^ 2) + (self.z() ^ 2)
        self.z()
            .mul_add(self.z(), self.x().mul_add(self.x(), self.y() * self.y()))
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }
    /// Dot product of two `Vec3`
    pub fn dot(u: &Self, v: &Self) -> f64 {
        // (u.x() * v.x()) + (u.y() * v.y()) + (u.z() * v.z())
        u.z().mul_add(v.z(), u.x().mul_add(v.x(), u.y() * v.y()))
    }
    /// Cross product of two `Vec3`
    pub fn cross(u: &Self, v: &Self) -> Self {
        Self::new(
            u.y() * v.z() - u.z() * v.y(),
            u.z() * v.x() - u.x() * v.z(),
            u.x() * v.y() - u.y() * v.x(),
        )
    }
}

// Operator overloads

// Indexing
impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!(),
        }
    }
}
impl ops::Index<u8> for Vec3 {
    type Output = f64;
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!(),
        }
    }
}
impl ops::Index<Axis> for Vec3 {
    type Output = f64;
    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.0,
            Axis::Y => &self.1,
            Axis::Z => &self.2,
        }
    }
}
impl ops::IndexMut<u8> for Vec3 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!(),
        }
    }
}

// Comparison
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        (self.x() == other.x()) && (self.y() == other.y()) && (self.z() == other.z())
    }
}
impl Eq for Vec3 {}

// Negation
impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}
impl ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

// Addition
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.x();
        self.1 += other.y();
        self.2 += other.z();
    }
}
impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}
impl ops::Add for &Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Vec3 {
        Vec3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

// Substraction
impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.x();
        self.1 -= other.y();
        self.2 -= other.z();
    }
}
impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}
impl ops::Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Vec3 {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

// Multiplication
impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.x();
        self.1 *= rhs.y();
        self.2 *= rhs.z();
    }
}
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}
impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}
impl ops::Mul for &Vec3 {
    type Output = Vec3;
    fn mul(self, other: Self) -> Vec3 {
        Vec3::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self::new(self.x() * other, self.y() * other, self.z() * other)
    }
}
impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.x() * other, self.y() * other, self.z() * other)
    }
}

// Division
impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.x();
        self.1 /= rhs.y();
        self.2 /= rhs.z();
    }
}
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}
impl ops::Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(
            self.x() / other.x(),
            self.y() / other.y(),
            self.z() / other.z(),
        )
    }
}
impl ops::Div for &Vec3 {
    type Output = Vec3;
    fn div(self, other: Self) -> Vec3 {
        Vec3::new(
            self.x() / other.x(),
            self.y() / other.y(),
            self.z() / other.z(),
        )
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self::new(self.x() / other, self.y() / other, self.z() / other)
    }
}
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self.x() / other, self.y() / other, self.z() / other)
    }
}

// Tests
#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]
    use super::*;

    #[test]
    fn test_comparison() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
        assert_ne!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
    }
    #[test]
    fn test_negation() {
        assert_eq!(-Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
        assert_eq!(-Vec3::new(-4.0, -5.0, -6.0), Vec3::new(4.0, 5.0, 6.0));
    }
    #[test]
    fn test_addition() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);
        vec += Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(vec, Vec3::new(5.0, 7.0, 9.0));
        let vec2 = Vec3::new(7.0, 8.0, 9.0);
        vec += vec2;
        assert_eq!(vec, Vec3::new(12.0, 15.0, 18.0));
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 5.0, 6.0),
            Vec3::new(5.0, 7.0, 9.0)
        );
    }
    #[test]
    fn test_substraction() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);
        vec -= Vec3::new(6.0, 5.0, 4.0);
        assert_eq!(vec, Vec3::new(-5.0, -3.0, -1.0));
        let vec2 = Vec3::new(7.0, 8.0, 9.0);
        vec -= vec2;
        assert_eq!(vec, Vec3::new(-12.0, -11.0, -10.0));
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) - Vec3::new(6.0, 5.0, 4.0),
            Vec3::new(-5.0, -3.0, -1.0)
        );
    }
    #[test]
    fn test_multiplication() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);
        vec *= Vec3::new(4.0, 6.0, 6.0);
        assert_eq!(vec, Vec3::new(4.0, 12.0, 18.0));
        let vec2 = Vec3::new(0.5, 1.5, 3.0);
        vec *= vec2;
        assert_eq!(vec, Vec3::new(2.0, 18.0, 54.0));

        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) * Vec3::new(6.0, 5.0, 4.0),
            Vec3::new(6.0, 10.0, 12.0)
        );
    }
    #[test]
    fn test_division() {
        let mut vec = Vec3::new(4.0, 12.0, 18.0);
        vec /= Vec3::new(4.0, 6.0, 6.0);
        assert_eq!(vec, Vec3::new(1.0, 2.0, 3.0));
        let vec2 = Vec3::new(0.5, 4.0, 3.0);
        vec /= vec2;
        assert_eq!(vec, Vec3::new(2.0, 0.5, 1.0));
        assert_eq!(
            Vec3::new(16.0, 24.0, 33.0) / Vec3::new(2.0, 6.0, 3.0),
            Vec3::new(8.0, 4.0, 11.0)
        );
    }

    #[test]
    fn test_getters() {
        let vec = Vec3::new(3.0, 2.0, 1.0);
        assert_eq!(vec.x(), 3.0);
        assert_eq!(vec.y(), 2.0);
        assert_eq!(vec.z(), 1.0);
    }

    #[test]
    fn test_length_squared() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).length_squared(), 14.0);
        assert_eq!(Vec3::new(4.0, 5.0, 6.0).length_squared(), 77.0);
        assert_eq!(Vec3::new(7.0, 8.0, 9.0).length_squared(), 194.0);
    }

    #[test]
    fn test_length() {
        assert_eq!(Vec3::new(1.0, 2.0, 2.0).length(), 3.0);
        assert_eq!(Vec3::new(1.0, 4.0, 8.0).length(), 9.0);
        assert_eq!(Vec3::new(2.0, 3.0, 6.0).length(), 7.0);
    }

    #[test]
    fn test_unit_vector() {
        let vec = Vec3::new(1.0, 2.0, 3.0).unit_vector();
        let unit_vector = Vec3::new(0.26726, 0.53452, 0.80178);

        let difference = vec - unit_vector;
        assert!(difference.x() < 0.00001);
        assert!(difference.y() < 0.00001);
        assert!(difference.z() < 0.00001);
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Vec3::dot(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(4.0, 5.0, 6.0)),
            32.0
        );
        assert_eq!(
            Vec3::dot(&Vec3::new(0.5, 3.0, 7.0), &Vec3::new(-4.0, 2.5, 8.0)),
            61.5
        );
        assert_eq!(
            Vec3::dot(&Vec3::new(12.5, 18.6, 22.14), &Vec3::new(1.1, 5.2, 1.23)),
            137.7022
        );
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Vec3::cross(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(4.0, 5.0, 6.0)),
            Vec3::new(-3.0, 6.0, -3.0)
        );
        assert_eq!(
            Vec3::cross(&Vec3::new(0.5, 3.0, 7.0), &Vec3::new(-4.0, 2.5, 8.0)),
            Vec3::new(6.5, -32.0, 13.25)
        );

        let calculated_cross_product =
            Vec3::cross(&Vec3::new(12.5, 18.6, 22.14), &Vec3::new(1.1, 5.2, 1.23));
        let cross_product = Vec3::new(-92.25, 8.979, 44.54);

        let difference = calculated_cross_product - cross_product;
        assert!(difference.x() < 0.00001);
        assert!(difference.y() < 0.00001);
        assert!(difference.z() < 0.00001);
    }
}
