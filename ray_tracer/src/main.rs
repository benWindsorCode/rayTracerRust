use std::io::Write;

use ray_tracer::Vec3;
use ray_tracer::Point3;
use ray_tracer::ray::Ray;
use ray_tracer::colour::Colour;
use ray_tracer::traits::Hittable;
use ray_tracer::objects::sphere::Sphere;
use ray_tracer::consts::INFINITY;
use ray_tracer::hittable::HittableList;
use ray_tracer::camera::Camera;
use ray_tracer::random::random_double;

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth: i64 = 50;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();


    //Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Lines remaining: {}", j);
        std::io::stdout().flush().ok().expect("Couldn't flush stdout");
        for i in 0..image_width {
            let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u: f64 = (i as f64 + random_double()) / (image_width-1) as f64;
                let v: f64 = (j as f64 + random_double()) / (image_height-1) as f64;

                let ray = cam.get_ray(u, v);
                pixel_colour = pixel_colour + ray_colour(&ray, &world, max_depth);
            }
            pixel_colour.write_colour(samples_per_pixel);
        }
    }

    eprintln!("\nDone.\n");
    std::io::stdout().flush().ok().expect("Couldn't flush stdout");
}

// Colour sphere by normal, and make gradient in background
fn ray_colour(ray: &Ray, world: &HittableList, depth: i64) -> Colour {

    // If we bounce up to the depth then we are fully black
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    // Cutoff t values at 0.001 to account for floating point approximation
    let hit_record = world.hit(ray, 0.001, INFINITY);

    match hit_record {
        Some(rec) => {
            let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
            return ray_colour(&Ray::new(rec.p, target - rec.p), &world, depth - 1) * 0.5;
        },
        None => {
            // If we dont hit the sphere then we are on the background, in which case make gradient
            let unit_direction = ray.direction.unit_vector();

            let t = 0.5 * (unit_direction.y + 1.0);
            
            return Colour::new(1.0, 1.0, 1.0) * (1.0 -t) + Colour::new(0.5, 0.7, 1.0) * t 
        }
    }
}
