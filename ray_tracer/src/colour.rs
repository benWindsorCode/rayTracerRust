use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug)]
pub struct Colour {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Colour {
    pub fn new(x: f64, y: f64, z: f64) -> Colour {
        Colour { x, y, z }
    }

    pub fn write_colour(self: &Self) -> () {
        println!("{} {} {}\n", (255.999 * self.x) as i64, (255.999 * self.y) as i64, (255.999 * self.z) as i64);
    }
}

impl Add for Colour {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Colour {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul<Colour> for Colour {
    type Output = Self;

    fn mul(self, other: Colour) -> Self {
        Self { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl Mul<f64> for Colour {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl Div<f64> for Colour {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self { x: (1.0/other) * self.x, y: (1.0/other) * self.y, z: (1.0/other) * self.z }
    }
}
