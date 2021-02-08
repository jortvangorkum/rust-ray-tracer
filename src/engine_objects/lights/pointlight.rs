use nalgebra::Vector3;

use crate::{EPSILON, engine_objects::{Ray, Scene}};

pub struct PointLight {
    pub origin: Vector3<f64>,
    pub intensity: f64,
}

impl PointLight {
    pub fn occluded(scene: &Scene, shadow_ray: &Ray, dist: f64) -> bool {
        for primitive in &scene.primitives {
            let material = primitive.get_material(&scene.materials);
            if !(material.refraction_cof > EPSILON) {
                let intersection = primitive.intersect(shadow_ray);
                if let Some(prim_dist) = intersection {
                    if prim_dist < dist {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}