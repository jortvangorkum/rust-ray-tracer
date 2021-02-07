use nalgebra::{Unit, Vector3};

use crate::engine_objects::{color::Color, ray::Ray};

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn get_color(&self) -> Color;
    fn get_normal(&self, intersection_point: &Vector3<f64>) -> Unit<Vector3<f64>>;
}