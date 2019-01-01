use std::ops::{Add, Mul, Not, Rem};

#[derive(Copy, Clone)]
struct Vec3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Add for Vec3d {
    type Output = Vec3d;
    fn add(self, other: Vec3d) -> Self::Output {
        Vec3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul for Vec3d {
    type Output = Vec3d;
    fn mul(self, other: Vec3d) -> Self::Output {
        Vec3d {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
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
