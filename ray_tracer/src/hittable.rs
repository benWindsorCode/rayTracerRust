use crate::{Vec3, Point3};
use crate::ray::Ray;

#[derive(Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, ray: &Ray, outward_normal: &Vec3) -> HitRecord {
        let front_face = ray.direction.dot(*outward_normal) < 0.0;
        let normal = if front_face { *outward_normal } else { *outward_normal * -1.0 };

        HitRecord{ p, normal, t, front_face }
    }
}
