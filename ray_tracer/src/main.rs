use std::io::Write;

use ray_tracer::Vec3;
use ray_tracer::Point3;
use ray_tracer::Ray;
use ray_tracer::Colour;

fn main() {

    // Image
    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.9);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    //Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height-1).rev() {
        eprintln!("Lines remaining: {}", j);
        std::io::stdout().flush().ok().expect("Couldn't flush stdout");

        for i in 0..image_width {
            let colour = Colour { 
                x: (i as f64) / (image_width - 1) as f64,
                y: (j as f64) / (image_height - 1) as f64,
                z: 0.25,
            };
            
            colour.write_colour();
        }
    }

    eprintln!("\nDone.\n");
    std::io::stdout().flush().ok().expect("Couldn't flush stdout");
}

fn ray_colour(ray: &Ray) -> Colour {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    
    Colour::new(1.0, 1.0, 1.0) * (1.0 -t) + Colour::new(0.5, 0.7, 1.0) * t 
}
