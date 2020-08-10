use ray_tracer::Ray;
use ray_tracer::HitRecord;

trait Hittable {
    fn hit(ray: &Ray, t_min: f64, t_max: f64, hit_record: &HitRecord) -> bool;
}
