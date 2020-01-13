use std::fmt;
use std::fmt::Debug;
use std::intrinsics::*;
use std::ops::{Add, Mul, Not, Rem, Sub};
#[derive(Copy, Clone, Default)]
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3d {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3d {
        Vec3d { x, y, z }
    }
}

impl Debug for Vec3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Add<Vec3d> for Vec3d {
    type Output = Vec3d;
    fn add(self, other: Vec3d) -> Self::Output {
        unsafe {
            Vec3d {
                x: fadd_fast(self.x, other.x),
                y: fadd_fast(self.y, other.y),
                z: fadd_fast(self.z, other.z),
            }
        }
    }
}

impl Add<f32> for Vec3d {
    type Output = Vec3d;
    fn add(self, other: f32) -> Self::Output {
        unsafe {
            Vec3d {
                x: fadd_fast(self.x, other),
                y: fadd_fast(self.y, other),
                z: fadd_fast(self.z, other),
            }
        }
    }
}

impl Sub<Vec3d> for Vec3d {
    type Output = Vec3d;
    fn sub(self, other: Self) -> Self::Output {
        unsafe {
            Vec3d {
                x: fsub_fast(self.x, other.x),
                y: fsub_fast(self.y, other.y),
                z: fsub_fast(self.z, other.z),
            }
        }
    }
}

impl Mul<Vec3d> for Vec3d {
    type Output = Vec3d;
    fn mul(self, other: Vec3d) -> Self::Output {
        unsafe {
            Vec3d {
                x: fmul_fast(self.x, other.x),
                y: fmul_fast(self.y, other.y),
                z: fmul_fast(self.z, other.z),
            }
        }
    }
}

impl Mul<f32> for Vec3d {
    type Output = Vec3d;
    fn mul(self, other: f32) -> Self::Output {
        unsafe {
            Vec3d {
                x: fmul_fast(self.x, other),
                y: fmul_fast(self.y, other),
                z: fmul_fast(self.z, other),
            }
        }
    }
}

impl Rem for Vec3d {
    type Output = f32;
    fn rem(self, other: Vec3d) -> Self::Output {
        unsafe {
            fadd_fast(
                fadd_fast(fmul_fast(self.x, other.x), fmul_fast(self.y, other.y)),
                fmul_fast(self.z, other.z),
            )
        }
    }
}

impl Not for Vec3d {
    type Output = Vec3d;
    fn not(self) -> Vec3d {
        self * (1.0 / (self % self).sqrt())
    }
}
