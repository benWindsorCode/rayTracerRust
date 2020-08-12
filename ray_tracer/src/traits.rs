use crate::ray::Ray;
use crate::hittable::HitRecord;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
