use crate::graphics::vector::Vec3d;

pub struct Ray {

   pub origin_point: Vec3d,
   pub dir: Vec3d,
}

#[allow(dead_code)]
impl Ray {
    // returns a point 
    pub fn at(self, t: f64) -> Vec3d {
        self.origin_point + self.dir * t
    }
}
