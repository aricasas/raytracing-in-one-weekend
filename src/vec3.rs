use rand::Rng;
use std::ops;

#[derive(Debug, Copy, Clone)]
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

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        Self::new(rng.gen(), rng.gen(), rng.gen())
    }
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
        (self.0.powi(2)) + (self.1.powi(2)) + (self.2.powi(2))
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }
    pub fn dot(u: &Self, v: &Self) -> f64 {
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

// Operator overloads

// Comparison
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0) && (self.1 == other.1) && (self.2 == other.2)
    }
}
impl Eq for Vec3 {}

// Negation
impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.0, -self.1, -self.2)
    }
}
impl ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.0, -self.1, -self.2)
    }
}

// Addition
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}
impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl ops::Add for &Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Vec3 {
        Vec3::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

// Substraction
impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}
impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl ops::Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Vec3 {
        Vec3::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

// Multiplication
impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
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
        Self::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}
impl ops::Mul for &Vec3 {
    type Output = Vec3;
    fn mul(self, other: Self) -> Vec3 {
        Vec3::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self::new(self.0 * other, self.1 * other, self.2 * other)
    }
}
impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.0 * other, self.1 * other, self.2 * other)
    }
}

// Division
impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
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
        Self::new(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}
impl ops::Div for &Vec3 {
    type Output = Vec3;
    fn div(self, other: Self) -> Vec3 {
        Vec3::new(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self::new(self.0 / other, self.1 / other, self.2 / other)
    }
}
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self.0 / other, self.1 / other, self.2 / other)
    }
}

// Tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
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
