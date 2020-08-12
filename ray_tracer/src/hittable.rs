use crate::{Vec3, Point3};
use crate::traits::Hittable;
use crate::ray::Ray;

#[derive(Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

// TODO: work out how to make the objects impl trait Hittable but have vec contain different
//       hittables not all the same hittable
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, ray: &Ray, outward_normal: &Vec3) -> HitRecord {
        let front_face = ray.direction.dot(*outward_normal) < 0.0;
        let normal = if front_face { *outward_normal } else { *outward_normal * -1.0 };

        HitRecord{ p, normal, t, front_face }
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec_to_return: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        let mut hit_anything = false;

        for obj in &self.objects {
            match obj.hit(&ray, t_min, closest_so_far) {
                Some(hit_record) => {
                    hit_anything = true;
                    closest_so_far = hit_record.t;
                    rec_to_return = Some(hit_record);
                }
                None => ()
            }
        }

        rec_to_return
    }
}
