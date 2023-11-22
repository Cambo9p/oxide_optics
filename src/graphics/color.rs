
use crate::graphics::vector::Vec3d;

pub struct Color { v: Vec3d }

#[allow(dead_code)]
impl Color {
   pub fn new(v: Vec3d) -> Color {
       Color { v }
   }

   pub fn write_color(self) {
       println!("{} {} {} ", (255.99 * self.v.x), (255.99 * self.v.y), (255.99 * self.v.z));
   }
}
