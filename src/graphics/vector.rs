use std::ops;

#[allow(dead_code)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[allow(dead_code)]
impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d{
        Vec3d { x, y, z }
    }

    pub fn len_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    pub fn len(self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn dot(self, rhs: Vec3d) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
    
    pub fn cross(self, rhs: Vec3d) -> Vec3d {
        Vec3d {  x: (self.y * rhs.z - self.z * rhs.y),
                y:(self.z * rhs.x - self.x * rhs.z),
                z: (self.x * rhs.y - self.y * rhs.x)
        }
    }

    pub fn unit_vector(self) -> Vec3d {
        self / self.len()
    }
    
}

impl ops::Add<Vec3d> for Vec3d {
    type Output = Vec3d;

    fn add(self, rhs: Vec3d) -> Self::Output {
        Vec3d { x: (self.x + rhs.x), y: (self.y + rhs.y), z: (self.z + rhs.z) }
    }
}

impl ops::Mul<f64> for Vec3d {
    type Output = Vec3d;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3d { x: (self.x * rhs), y: (self.y * rhs), z: (self.z * rhs) }
    }
}

impl ops::Mul<i32> for Vec3d {
    type Output = Vec3d;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec3d {
            x: self.x * rhs as f64,
            y: self.y * rhs as f64,
            z: self.z * rhs as f64,
        }
    }
}

impl ops::Div<f64> for Vec3d {
    type Output = Vec3d;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0/rhs)
    }
}

impl ops::Sub<Vec3d> for Vec3d {
    type Output = Vec3d;

    fn sub(self, rhs: Vec3d) -> Self::Output {
        Vec3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
