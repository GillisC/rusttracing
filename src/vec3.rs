#![allow(dead_code)]

use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
use float_cmp::approx_eq;

type Point = Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn build(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
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

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
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

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 { 
    fn div_assign(&mut self, rhs: f64) {
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
        approx_eq!(f64, normalized.length(), 1.0, ulps = 2);
    }
}
