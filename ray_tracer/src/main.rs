use std::io::Write;

use ray_tracer::Vec3;
use ray_tracer::Point3;
use ray_tracer::ray::Ray;
use ray_tracer::colour::Colour;
use ray_tracer::traits::Hittable;
use ray_tracer::objects::sphere::Sphere;
use ray_tracer::consts::INFINITY;
use ray_tracer::hittable::HittableList;

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);


    //Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Lines remaining: {}", j);
        std::io::stdout().flush().ok().expect("Couldn't flush stdout");

        for i in 0..image_width {
            let u: f64 = i as f64 / (image_width-1) as f64;
            let v: f64 = j as f64 / (image_height-1) as f64;

            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let pixel_colour = ray_colour(&ray, &world);
            
            pixel_colour.write_colour();
        }
    }

    eprintln!("\nDone.\n");
    std::io::stdout().flush().ok().expect("Couldn't flush stdout");
}

// Colour sphere by normal, and make gradient in background
fn ray_colour(ray: &Ray, world: &HittableList) -> Colour {
    let hit_record = world.hit(ray, 0.0, INFINITY);

    match hit_record {
        Some(rec) => {
            return (rec.normal + Colour::new(1.0, 1.0, 1.0)) * 0.5;
        },
        None => {
            // If we dont hit the sphere then we are on the background, in which case make gradient
            let unit_direction = ray.direction.unit_vector();

            let t = 0.5 * (unit_direction.y + 1.0);
            
            return Colour::new(1.0, 1.0, 1.0) * (1.0 -t) + Colour::new(0.5, 0.7, 1.0) * t 
        }
    }
}
