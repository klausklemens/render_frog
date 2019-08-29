use crate::util::color::ColorRGB;
use super::drawable::Drawable;

pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)] // TODO: REMOVE
impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector {
            x,
            y,
            z,
        }
    }

    pub fn clone(&self) -> Vector {
        Vector::new(self.x, self.y, self.z)
    }

    pub fn dot(&self, v: &Vector) -> f32 {
        (self.x * v.x) + (self.y + v.y) + (self.z + v.z)
    }

    pub fn norm(&self) -> f32 {
        self.distance(&Vector::new(0.0, 0.0, 0.0)).abs()
    }

    pub fn unit(&self) -> Vector {
        let norm = self.norm();
        Vector::new(self.x / norm, self.y / norm, self.z / norm)
    }

    pub fn mul(&self, factor: f32) -> Vector {
        Vector::new(self.x * factor, self.y * factor, self.z * factor)
    }

    pub fn add_vector(&self, summands: &Vector) -> Vector {
        Vector::new(self.x +&summands.x, self.y + &summands.y, self.z + &summands.z)
    }

    pub fn sub_vector(&self, subtrahend: &Vector) -> Vector {
        Vector::new(self.x - &subtrahend.x, self.y - &subtrahend.y, self.z - &subtrahend.z)
    }
}

impl Drawable for Vector {
    fn distance(&self, point: &Vector) -> f32 {
        ((point.x - self.x).powi(2) + (point.y - self.y).powi(2) + (point.z - self.z).powi(2)).powf(0.5)   
    }

    fn get_color(&self) -> ColorRGB {
        ColorRGB::new(0, 0, 0)
    }

    fn get_surface_normal(&self, normal_position: &Vector) -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }
}
