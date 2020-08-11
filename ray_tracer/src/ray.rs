use super::Point3;
use super::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(self: &Self, t: f64) -> Point3 {
        self.origin + self.direction*t
    }
}
