pub mod ray;
pub mod colour;
pub mod objects;
pub mod traits;
pub mod hittable;
pub mod utils;
pub mod consts;
pub mod random;

use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use crate::colour::Colour;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(self: &Self, other: Vec3) -> f64 {
        (self.x*other.x) + (self.y*other.y) + (self.z*other.z)
    }

    pub fn cross(self: &Self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z - other.y,
            y: self.z * other.x - self.x - other.z,
            z: self.x * other.y - self.y - other.x,
        }
    }

    pub fn length_squared(self: &Self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self: &Self) -> f64 {
        return self.length_squared().sqrt()
    }

    pub fn unit_vector(self: &Self) -> Vec3 {
       return *self / self.length() 
    }
}


impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Add<Colour> for Vec3 {
    type Output = Colour;

    fn add(self, other: Colour) -> Colour {
        Colour { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self {
        Self { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self { x: (1.0/other) * self.x, y: (1.0/other) * self.y, z: (1.0/other) * self.z }
    }
}

