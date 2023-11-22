mod graphics;

use graphics::image::{Image,Pixel};
use graphics::ray::Ray;
use graphics::vector::Vec3d;
// use graphics::color::Color;
// use graphics::vector::Vec3;
use std::fs::File;
use std::io::{self,Write};



#[allow(dead_code, unused_variables)]
fn sample_image() -> Image {
    return Image::new(200, 200, render);
}

#[allow(dead_code, unused_variables)]
fn render(row: i32, col: i32) -> Pixel {
    let image_height = 256;
    let image_width = 256;

    let r = row as f64 / (image_width - 1) as f64;
    let g = col as f64 / (image_height - 1) as f64;
    let b = 0 as f64;

    let ir = (255.999 * r) as i32;
    let ig = (255.999 * g) as i32;
    let ib = (255.999 * b) as i32;

    Pixel::new(ir, ig, ib)
}

fn write_image_to_file(image: Image, filename: String) -> io::Result<()> {
    let mut file = File::create(filename)?;
    write!(file, "{}", image)?;
    Ok(())
}

#[allow(dead_code, unused_variables)]
fn ray_color(r: Ray) -> Vec3d {
    let unit_dir = r.dir.unit_vector();
    let a = 0.5 * (unit_dir.y + 1.0);
    Vec3d::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3d::new(0.5, 0.7, 1.0) * a
}

#[allow(dead_code, unused_variables)]
fn raytraced_image() -> Image {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;

    // calc the image height 
    let mut image_height = image_width / aspect_ratio;
    image_height = if image_height >= 1.0 { image_height } else { 1.0 };

    // setup camera
    
    let focal_length = 1.0;
    let viewport_length = 2.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height);
    
    let camera_center = Vec3d::new(0.0, 0.0, 0.0);

    // calculate vectors 
    let viewport_u = Vec3d::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3d::new(0.0, -viewport_height, 0.0);

    // calculate horiz and vertial delta vectors
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // calculate the location of the upper left pixel
    let viewport_upper_left = camera_center - Vec3d::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;


    Image::new(image_width as usize, image_height as usize, |row, col| {
        let pixel_center = pixel00_loc + (pixel_delta_u * col) + (pixel_delta_v * row);
        let ray_direction = pixel_center - camera_center;
        let ray = Ray { origin_point: pixel_center, dir: ray_direction };
        
        Pixel::new_from_vec(ray_color(ray))
    })
    
}

fn main() {
    let filename = "image.ppm".to_string();
    let image = raytraced_image();
    let _ = write_image_to_file(image, filename).unwrap();
}
