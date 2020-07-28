use std::fs::File;
use std::io::prelude::*;

fn main(){
    // Image
    let image_width : u32 = 256;
    let image_height : u32 = 256;
    let mut image = File::create("image.ppm").unwrap();

    // Render
    image.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes()).expect("panic!");
    for j in (0..image_height).rev(){
        eprintln!("\rScanlines remaining: {} ", j); // Prints simple progress indicator to error stream
        for i in (0..image_width).rev(){
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b:f64 = 0.25;

            let ir:u32 = (255.999 * r) as u32;
            let ig:u32 = (255.999 * g) as u32;
            let ib:u32 = (255.999 * b) as u32;
            image.write(format!("{} {} {}\n",ir,ig,ib).as_bytes()).expect("Failed to write a pixel!");
        }
    }
    eprintln!("\nDone!\n");
}
