use std::{fs::{self, OpenOptions}, io::Write};
mod vector;
use vector::Vector3;
use winconsole;


fn main() {
    let image_width = 256;
    let image_height = 256;

    let test_vector: Vector3 = Vector3::new(2f64, 4f64, 6f64);
    let test_vector2: Vector3 = Vector3::new(2f64, 4f64, 6f64);

    write_metadata(image_width, image_height);
    let mut final_render = OpenOptions::new()
    .append(true)
    .open("final.ppm")
    .expect("cant open final render file");
    let mut result = String::new();
    for j in (0..image_height) {
        winconsole::console::set_title(&format!("{}/{}", (j*image_width).to_string(), (image_height*image_width).to_string()));
        for i in 0..image_width {
            let r = i;
            let g = j;
            let b = 1;
            
            let str_result = format!("{} {} {}{}", r.to_string(), g.to_string(), b.to_string(), "\n"); 
            result.push_str(&str_result);
        }
    }
    let write_result = final_render.write(result.as_bytes());
    println!("{:?}", test_vector + test_vector2);
}

fn write_metadata(w: i32, h: i32) -> Result<(), std::io::Error>{
    let to_write = format!("{}{}{} {}{}{}{}", "P3", "\n", w.to_string(), h.to_string(), "\n", 255.to_string(), "\n");
    let e = fs::write("./final.ppm", to_write);
    e
}
