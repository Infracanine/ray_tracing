use std::fs::File;
use std::io::prelude::*;

mod ray;
mod colour;
mod vec3;
fn main(){
    // Image creation
    const ASPECT_RATIO : f64 = 16.0 / 9.0;
    const IMAGE_WIDTH : u32 = 400;
    const IMAGE_HEIGHT : u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    let mut image = File::create("image.ppm").expect("Failed to create image file!");

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin : vec3::Point3 = vec3::zeroes();
    let horizontal : vec3::Vec3 = vec3::new(viewport_width,0.0,0.0);
    let vertical : vec3::Vec3 = vec3::new(0.0,viewport_height,0.0);
    let bottom_left_corner = &origin - &(vec3::clone(&horizontal)/2.0) - vec3::clone(&vertical)/2.0 - vec3::new(0.0,0.0,focal_length);
    // Render 
    image.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes()).expect("Failed to write PNN prelude");
    for j in (0..IMAGE_HEIGHT).rev(){
        eprintln!("\rScanlines remaining: {} ", j); // Prints simple progress indicator to error stream
        for i in (0..IMAGE_WIDTH).rev(){
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = ray::Ray{
                // Need to determine if should implement the copy trait for vec
                orig: vec3::clone(&origin),
                dir: &bottom_left_corner + &(u*&horizontal) + (v*&vertical) - vec3::clone(&origin)
            };
            let pixel_colour = ray::ray_colour(r);
            colour::write_colour(&image,pixel_colour);
        }
    }
    eprintln!("\nDone!\n");
}
