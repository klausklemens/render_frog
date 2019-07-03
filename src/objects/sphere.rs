use crate::util::color::ColorRGB;
use super::vector::*;
use super::drawable::Drawable;

pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
    pub color: ColorRGB
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            center: Vector::new(0.0, 0.0, 0.0),
            radius: 0.0,
            color: ColorRGB::new(0, 0, 0),
        }
    }
}

impl Drawable for Sphere {
    fn distance(&self, point: &Vector) -> f32 {
        point.distance(&self.center) - &self.radius
    }
}
