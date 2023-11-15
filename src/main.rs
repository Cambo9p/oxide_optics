mod graphics;

use graphics::image::*;
use std::fs::File;
use std::io::{self,Write};


fn sample_image() -> Image {
    return Image::new(1, 1, |row, col| Pixel::new((row + 100) as i32, (col + 50) as i32, 0));
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
