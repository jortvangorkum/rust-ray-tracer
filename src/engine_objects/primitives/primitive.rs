use crate::engine_objects::{color::Color, ray::Ray};

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn get_color(&self) -> Color;
}