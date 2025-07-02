#![allow(dead_code)]

use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
use float_cmp::approx_eq;

pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn build(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn z(&self) -> f32 { self.z }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns true if the all the vectors components are near zero.
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(&self, normal: &Vec3) -> Self {
        *self - 2.0 * Vec3::dot(self, normal) * *normal
    }


    /// Returns a new `Vec3` with random components in the range [0.0, 1.0).
    pub fn random() -> Self {
        Vec3::build(
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>()
        )
    }

    /// Returns a new `Vec3` with random components in the range [min, max).
    pub fn random_range(min: f32, max: f32) -> Self {
        Vec3::build(
            rand::random::<f32>() * (max - min) + min,
            rand::random::<f32>() * (max - min) + min,
            rand::random::<f32>() * (max - min) + min
        )
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
        return u.x * v.x + u.y * v.y + u.z * v.z;
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Self {
        Vec3::build(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x
        )
    }

    pub fn normalize(v: &Vec3) -> Self {
        *v / v.length()
    }

    /// Returns a new `Vec3` that is a random unit vector.
    pub fn random_unit_vector() -> Self {
        loop {
            let v = Vec3::random_range(-1.0, 1.0);
            let len_squared = v.length_squared();
            if 1e-160 < len_squared && len_squared < 1.0 {
                return v / len_squared.sqrt();
            }
        }
    }

    /// Returns a new `Vec3` that is a random unit vector in the hemisphere defined by the normal.
    /// This is used for generating random directions for diffuse reflection.
    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Vec3::random_unit_vector();
        if Vec3::dot(&on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
        
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vec3 { 
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec3::new();
        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 0.0);
        assert_eq!(v.z(), 0.0);
    }

    #[test]
    fn test_build() {
        let v = Vec3::build(0.1, 0.2, 0.3);
        assert_eq!(v.x(), 0.1);
        assert_eq!(v.y(), 0.2);
        assert_eq!(v.z(), 0.3);
    }

    #[test]
    fn test_eq() {
        let v = Vec3::build(0.1, 0.2, 0.3);
        let u = Vec3::build(0.1, 0.2, 0.3);
        assert_eq!(v, u);
    }

    #[test]
    fn test_add() {
        let v = Vec3::build(0.1, 0.2, 0.3);
        let u = Vec3::build(0.1, 0.2, 0.3);
        let result = v + u;
        assert_eq!(result.x, 0.2);
        assert_eq!(result.y, 0.4);
        assert_eq!(result.z, 0.6);
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vec3::build(0.1, 0.2, 0.3);
        let u = Vec3::build(0.1, 0.2, 0.3);
        v += u;
        assert_eq!(v.x, 0.2);
        assert_eq!(v.y, 0.4);
        assert_eq!(v.z, 0.6);
    }
    
    #[test]
    fn test_sub() {
        let v = Vec3::build(0.1, 0.2, 0.3);
        let u = Vec3::build(0.1, 0.2, 0.3);
        let result = v - u;
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut v = Vec3::build(0.1, 0.2, 0.3);
        let u = Vec3::build(0.1, 0.2, 0.3);
        v -= u;
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_neg() {
        let v = -Vec3::build(0.1, 0.2, 0.3);
        assert_eq!(v.x, -0.1);
        assert_eq!(v.y, -0.2);
        assert_eq!(v.z, -0.3);
    }

    #[test]
    fn test_mul_constant() {
        let v = Vec3::build(0.1, 0.2, 0.3);
        let result = v * 2.0;
        assert_eq!(result.x, 0.2);
        assert_eq!(result.y, 0.4);
        assert_eq!(result.z, 0.6);
    }

    #[test]
    fn test_mul_vec() {
        let u = Vec3::build(0.1, 0.2, 0.3);
        let v = Vec3::build(2.0, 2.0, 2.0);
        let result = u * v;
        assert_eq!(result.x, 0.2);
        assert_eq!(result.y, 0.4);
        assert_eq!(result.z, 0.6);
    }

    #[test]
    fn test_div() {
        let v = Vec3::build(0.2, 0.2, 0.2);
        let result = v / 2.0;
        assert_eq!(result.x, 0.1);
        assert_eq!(result.y, 0.1);
        assert_eq!(result.z, 0.1);
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vec3::build(0.2, 0.2, 0.2);
        v /= 2.0;
        assert_eq!(v.x, 0.1);
        assert_eq!(v.y, 0.1);
        assert_eq!(v.z, 0.1);
    }

    #[test]
    fn test_length() {
        let v = Vec3::build(2.0, 0.0, 0.0);
        assert_eq!(v.length(), 2.0);
    }

    #[test]
    fn test_dot_product() {
        let u = Vec3::build(1.0, 2.0, 3.0);
        let v = Vec3::build(4.0, -5.0, 6.0);
        let result = Vec3::dot(&u, &v);
        assert_eq!(result, 12.0);
    }

    #[test]
    fn test_cross_product() {
        let u = Vec3::build(1.0, 0.0, 0.0);
        let v = Vec3::build(0.0, 1.0, 0.0);
        let result = Vec3::cross(&u, &v);
        assert_eq!(result, Vec3 { x: 0.0, y: 0.0, z: 1.0 });
    }

    #[test]
    fn test_normalize() {
        let v = Vec3::build(5.0, 0.1, 6.0);
        let normalized = Vec3::normalize(&v);
        approx_eq!(f32, normalized.length(), 1.0, ulps = 2);
    }
}
