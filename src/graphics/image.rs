use std::fmt;

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
    pub fn new(width: usize, height: usize, f: fn(usize, usize) -> Pixel) -> Image {
        let mut pixels: Vec<Vec<Pixel>> = vec![vec![Pixel::new(0,0,0); width]; height];
        for row in 0..height {
            for col in 0..width {
                pixels[row][col] = f(row, col);
            } 
        }
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
