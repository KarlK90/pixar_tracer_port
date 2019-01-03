use std::ops::{Add, Mul, Not, Rem};
use std::fmt::Debug;
use std::fmt;

#[derive(Copy, Clone, Default)]
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Debug for Vec3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Add<Vec3d> for Vec3d {
    type Output = Vec3d;
    fn add(self, other: Vec3d) -> Self::Output {
        Vec3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f32> for Vec3d {
    type Output = Vec3d;
    fn add(self, other: f32) -> Self::Output {
        Vec3d {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Mul<Vec3d> for Vec3d {
    type Output = Vec3d;
    fn mul(self, other: Vec3d) -> Self::Output {
        Vec3d {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3d {
    type Output = Vec3d;
    fn mul(self, other: f32) -> Self::Output {
        Vec3d {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Rem for Vec3d {
    type Output = f32;
    fn rem(self, other: Vec3d) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Not for Vec3d {
    type Output = Vec3d;
    fn not(self) -> Vec3d {
        self * (1.0 / (self % self).sqrt())
    }
}

impl Vec3d {
    pub fn new(v: f32) -> Self {
        Self { x: v, y: v, z: v }
    }
}
