use std::fmt;
use crate::graphics::vector::Vec3d;

#[derive(Clone)]
pub struct Pixel {
    r: i32,
    g: i32,
    b: i32,
}

impl Pixel {
    pub fn new(r: i32, g:i32, b:i32) -> Pixel {
        Pixel { r, g, b }
    }

    pub fn new_from_vec(v: Vec3d) -> Pixel {
        Pixel { r: (v.x as i32), g: (v.y as i32), b: (v.z as i32) }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)?;
        Ok(())
    }

}

pub struct Image {
   pixels: Vec<Vec<Pixel>>, 
}

impl Image {
    
    // takes the width and height of the image and a function that generates each pixel and returns
    // a new image
    pub fn new<F>(width: usize, height: usize, f: F) -> Image 
    where
        F: Fn(i32, i32) -> Pixel,
    {
        let mut pixels: Vec<Vec<Pixel>> = vec![vec![Pixel::new(0,0,0); width]; height];
        for row in 0..height {
            println!("scanlines remaining: {}", height - row);
            for col in 0..width {
                let row_i32 = row as i32;
                let col_i32 = col as i32;
                pixels[row][col] = f(row_i32, col_i32);
            } 
        }
        println!("done");
        Image { pixels }
        
    }

}

impl fmt::Display for Image {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P3\n{} {}\n255\n", self.pixels[0].len(), self.pixels.len())?;
        for row in &self.pixels {
            for pixel in row {
                write!(f, "{}\n", pixel)?;
            }
        }
        Ok(())
    }
}
