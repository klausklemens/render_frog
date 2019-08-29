use crate::util::color::ColorRGB;
use super::vector::Vector;

pub trait Drawable {
    fn distance(&self, point: &Vector) -> f32; 
    fn get_color(&self) -> ColorRGB;
    fn get_surface_normal(&self, normal_position: &Vector) -> Vector;
}
