use crate::Point3;
use crate::HitRecord;
use crate::ray::Ray;
use crate::traits::Hittable;

#[derive(Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            // Check if the first root is in range
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let hit_record = HitRecord::new(
                    ray.at(temp), 
                    temp, 
                    ray,
                    &((ray.at(temp) - self.center) / self.radius), // outward normal    
                );

                return Some(hit_record);
            }

            // Check if the second root is in range
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let hit_record = HitRecord::new(
                    ray.at(temp), 
                    temp, 
                    ray,
                    &((ray.at(temp) - self.center) / self.radius), // outward normal    
                );

                return Some(hit_record);
            }
        }

        None
    }
}
