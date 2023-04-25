use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

/// A 3D point
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {}, {})", self.x, self.y, self.z);
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = f32;
    fn mul(self, rhs: Self) -> Self::Output {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

pub struct Camera {
    pub position: Vec3,

    pub up: Vec3,
    pub right: Vec3,
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

pub struct Scene {
    pub camera: Camera,
    pub screen_width: i32,
    pub screen_height: i32,
    pub focal_distance: f32,
}
