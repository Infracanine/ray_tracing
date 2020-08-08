use std::io::*;
use std::fs::File;

use crate::vec3;

// writes a colour to a file
// TODO generalise this to write to any output stream (i think)
pub fn write_colour(mut out: &File, pixel_colour: vec3::Colour){
    let ir = (255.999 * pixel_colour.x) as u32;
    let ig = (255.999 * pixel_colour.y) as u32;
    let ib = (255.999 * pixel_colour.z) as u32;
    out.write(format!("{} {} {}\n",ir,ig,ib).as_bytes()).expect("Failed to write colour!");
}