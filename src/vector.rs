use packed_simd::*;
use std::fmt;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Not, Rem, Sub};
#[derive(Copy, Clone, Default)]
pub struct Vec3d {
    pub xyz: f32x4,
}

impl Vec3d {
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3d {
        Vec3d {
            xyz: f32x4::new(x, y, z, 0.0),
        }
    }

    #[inline(always)]
    pub fn set_x(&mut self, val: f32) {
        unsafe {
            self.xyz = self.xyz.replace_unchecked(0, val);
        }
    }
    #[inline(always)]
    pub fn set_y(&mut self, val: f32) {
        unsafe {
            self.xyz = self.xyz.replace_unchecked(1, val);
        }
    }
    #[inline(always)]
    pub fn set_z(&mut self, val: f32) {
        unsafe {
            self.xyz = self.xyz.replace_unchecked(2, val);
        }
    }
    #[inline(always)]
    pub fn get_x(&self) -> f32 {
        unsafe { self.xyz.extract_unchecked(0) }
    }
    #[inline(always)]
    pub fn get_y(&self) -> f32 {
        unsafe { self.xyz.extract_unchecked(1) }
    }
    #[inline(always)]
    pub fn get_z(&self) -> f32 {
        unsafe { self.xyz.extract_unchecked(2) }
    }

    #[inline(always)]
    pub fn min_element(&self) -> f32 {
        unsafe {
            self.xyz
                .replace_unchecked(3, std::f32::INFINITY)
                .min_element()
        }
    }

    #[inline(always)]
    pub fn min(&self, other: Self) -> Self {
        Vec3d {
            xyz: self.xyz.min(other.xyz),
        }
    }
}

impl Debug for Vec3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.get_x(), self.get_y(), self.get_z())
    }
}

impl Add<Vec3d> for Vec3d {
    type Output = Vec3d;
    #[inline(always)]
    fn add(self, other: Vec3d) -> Self::Output {
        Vec3d {
            xyz: self.xyz + other.xyz,
        }
    }
}

impl Add<f32> for Vec3d {
    type Output = Vec3d;
    #[inline(always)]
    fn add(self, other: f32) -> Self::Output {
        Vec3d {
            xyz: self.xyz + other,
        }
    }
}

impl Sub<Vec3d> for Vec3d {
    type Output = Vec3d;
    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        Vec3d {
            xyz: self.xyz - other.xyz,
        }
    }
}

impl Div<f32> for Vec3d {
    type Output = Vec3d;
    #[inline(always)]
    fn div(self, other: f32) -> Self::Output {
        Vec3d {
            xyz: self.xyz / other,
        }
    }
}

impl Div<Vec3d> for Vec3d {
    type Output = Vec3d;
    #[inline(always)]
    fn div(self, other: Vec3d) -> Self::Output {
        Vec3d {
            xyz: self.xyz / other.xyz,
        }
    }
}

impl Mul<Vec3d> for Vec3d {
    type Output = Vec3d;
    #[inline(always)]
    fn mul(self, other: Vec3d) -> Self::Output {
        Vec3d {
            xyz: self.xyz * other.xyz,
        }
    }
}

impl Mul<f32> for Vec3d {
    type Output = Vec3d;
    #[inline(always)]
    fn mul(self, other: f32) -> Self::Output {
        Vec3d {
            xyz: self.xyz * other,
        }
    }
}

impl Rem for Vec3d {
    type Output = f32;
    #[inline(always)]
    fn rem(self, other: Vec3d) -> Self::Output {
        (self * other).xyz.sum()
    }
}

impl Not for Vec3d {
    type Output = Vec3d;
    #[inline(always)]
    fn not(self) -> Vec3d {
        self * (1.0 / (self % self).sqrt())
    }
}
