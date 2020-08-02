use std::fs::File;
use std::io::prelude::*;

mod vec3;

fn main(){
    // Image creation
    let image_width : u32 = 256;
    let image_height : u32 = 256;
    let mut image = File::create("image.ppm").expect("Failed to create image file!");

    // Testing code goes here
    let test = vec3::Vec3{x:1.0,y:2.0,z:3.0};
    // let test2 = vec3::Vec3{x:1.0,y:2.0,z:3.0};               
    // let test3 = test / 2.0;
    println!("test vector is now: {}",test);


    // Render
    image.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes()).expect("Failed to write PNN prelude");
    for j in (0..image_height).rev(){
        eprintln!("\rScanlines remaining: {} ", j); // Prints simple progress indicator to error stream
        for i in (0..image_width).rev(){
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b:f64 = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;
            image.write(format!("{} {} {}\n",ir,ig,ib).as_bytes()).expect("Failed to write a pixel!");
        }
    }
    eprintln!("\nDone!\n");
}
