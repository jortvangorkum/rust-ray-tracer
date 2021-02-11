use nalgebra::{Unit, Vector3};

use crate::engine_objects::{Material, Ray, bvh::AABB};

pub trait Primitive {
    fn intersect(self: &Self, ray: &Ray) -> Option<f64>;
    fn get_material(self: &Self, materials: &Vec<Material>) -> Material;
    fn get_normal(self: &Self, intersection_point: &Vector3<f64>) -> Unit<Vector3<f64>>;
    fn get_centroid(self: &Self) -> Vector3<f64>;
    fn get_bounds(self: &Self) -> AABB;
}