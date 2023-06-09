mod vector;
mod ray;
use std::{fs::{self, OpenOptions}, io::Write, env};
use vector::{Vector3, Colour3};
use ray::Ray;
use winconsole;


fn ray_colour(ray: Ray) -> Colour3 { // Function for generating background
    let unit_direction = ray.direction().unit_vector(); // Turn Vector from regular vector to fixed length unit vector (max size is 1, completely directional)
    let t = 0.5*(unit_direction.y() + 1.0); // Normalises the range from -1 - 1 to 0 - 1. This will help with the gradient process
    let result = Colour3::new(1.0, 1.0, 1.0)*(1.0 - t) + Colour3::new(0.5, 0.7, 1.0)*t; // Linearly blend white and blue depending on y coordinate. 
    result
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // Debugging
    let image_width = 256;
    let image_height = 256;

    let mut test_vector: Vector3 = Vector3::new(0f32, 0f32, 0f32);
    let mut test_vector2: Vector3 = Vector3::new(10f32, 0f32, 0f32);
    let test_ray = Ray::new(test_vector, test_vector2);

    println!("{:?}", test_ray.at(-0.5f32));


    write_metadata(image_width, image_height).expect("failed to write metadata");
    let mut final_render = OpenOptions::new()
    .append(true)
    .open("final.ppm")
    .expect("cant open final render file");
    let mut result = String::new();
    for j in(0..image_height).rev() {
        winconsole::console::set_title(&format!("{}/{}", (j*image_width).to_string(), (image_height*image_width).to_string())).expect("failed to change console title");
        for i in 0..image_width {
            let r = i as f32 / (image_height-1) as f32;
            let g = j as f32 / (image_width-1) as f32;
            let b = 0.25f32;

            let rgb = Colour3::new(r, g, b);
            let rgb_arr = rgb.to_rgb();

            let str_result = format!("{} {} {}{}", rgb_arr[0], rgb_arr[1], rgb_arr[2], "\n"); 
            result.push_str(&str_result);
        }
    }
    final_render.write(result.as_bytes()).expect("failed to write to file");
    //println!("{:?}", test_vector + test_vector2);
}

fn write_metadata(w: i32, h: i32) -> Result<(), std::io::Error>{
    let to_write = format!("{}{}{} {}{}{}{}", "P3", "\n", w.to_string(), h.to_string(), "\n", 255.to_string(), "\n");
    let e = fs::write("./final.ppm", to_write);
    e
}