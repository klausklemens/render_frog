use crate::util::color::ColorRGB;
use super::vector::*;
use super::drawable::Drawable;

pub struct Lightsource {
    pub center: Vector,
    pub direction: Vector,
    pub f32: intensity,
    pub color: ColorRGB,
}

impl Lightsource {
    pub fn new() -> Sphere {
        Sphere {
            center: Vector::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 0.0),
            intensity: 0.0,
            color: ColorRGB::new(0, 0, 0),
        }
    }
}

impl Drawable for Lightsource {
    fn distance(&self, point: &Vector) -> f32 {
        point.distance(&self.center)
    }

    fn get_color(&self) -> ColorRGB {
        self.color.clone()
    }
}
