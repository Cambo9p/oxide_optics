mod graphics;

use graphics::image::*;
use std::fs::File;
use std::io::{self,Write};


fn sample_image() -> Image {
    return Image::new(200, 200, render);
}

fn render(row: usize, col: usize) -> Pixel {
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

fn main() {
    let filename = "image.ppm".to_string();
    let image = sample_image();
    let _ = write_image_to_file(image, filename).unwrap();
}
