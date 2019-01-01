use std::ops::{Add, Mul, Not, Rem};

#[derive(Copy, Clone, Default)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

impl Add<f64> for Vec3d {
    type Output = Vec3d;
    fn add(self, other: f64) -> Self::Output {
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

impl Mul<f64> for Vec3d {
    type Output = Vec3d;
    fn mul(self, other: f64) -> Self::Output {
        Vec3d {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Rem for Vec3d {
    type Output = f64;
    fn rem(self, other: Vec3d) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Not for Vec3d {
    type Output = Vec3d;
    fn not(self) -> Vec3d {
        self * Vec3d::new(1.0 / (self % self).sqrt())
    }
}

impl Vec3d {
    pub fn new(v: f64) -> Self {
        Self { x: v, y: v, z: v }
    }
}
