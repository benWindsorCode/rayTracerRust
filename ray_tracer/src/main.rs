use std::io::Write;

use ray_tracer::Vec3;
use ray_tracer::Point3;
use ray_tracer::Colour;

fn main() {
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height-1).rev() {
        eprintln!("Lines remaining: {}", j);
        std::io::stdout().flush().ok().expect("Couldn't flush stdout");

        for i in 0..image_width {
            let r = (i as f32) / (image_width - 1) as f32;
            let g = (j as f32) / (image_height - 1) as f32;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}\n", ir, ig, ib);
        }
    }

}
