use super::vector::Vector;

pub trait Drawable {
    fn distance(&self, point: &Vector) -> f32; 
}
