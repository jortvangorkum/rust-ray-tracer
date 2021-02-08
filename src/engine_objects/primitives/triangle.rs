use nalgebra::{Unit, Vector3};

use crate::{EPSILON, engine_objects::{Material, Ray}};

use super::Primitive;

pub struct Triangle {
    pub v0: Vector3<f64>,
    pub v1: Vector3<f64>,
    pub v2: Vector3<f64>,
    pub v0v1: Vector3<f64>,
    pub v0v2: Vector3<f64>,
    pub flip_normal: bool,
    pub material_index: usize,
}

impl Triangle {
    pub fn create_triangle(v0: Vector3<f64>, v1: Vector3<f64>, v2: Vector3<f64>, flip_normal: bool, material_index: usize) -> Triangle {
        return Triangle {
            v0,
            v1,
            v2,
            v0v1: v1 - v0,
            v0v2: v2 - v0,
            flip_normal,
            material_index,
        }
    }
}

impl Primitive for Triangle {
    fn get_material(&self, materials: &Vec<Material>) -> Material {
        return materials[self.material_index];
    }

    fn get_normal(&self, _intersection_point: &Vector3<f64>) -> Unit<Vector3<f64>> {
        let normal = (self.v1 - self.v0).cross(&(self.v2 - self.v0));
        return Unit::new_normalize(if self.flip_normal { -normal } else { normal });
    }

    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let pvec: Vector3<f64> = ray.direction.cross(&self.v0v2);
        let det: f64           = self.v0v1.dot(&pvec);

        if det < EPSILON && det > -EPSILON { return None; }

        let inv_det: f64       = 1.0 / det;
        let tvec: Vector3<f64> = ray.origin - self.v0;
        let u: f64             = tvec.dot(&pvec) * inv_det;

        if u < 0.0 || u > 1.0 { return None; }

        let qvec: Vector3<f64> = tvec.cross(&self.v0v1);
        let v: f64             = ray.direction.dot(&qvec) * inv_det;

        if v < 0.0 || u + v > 1.0 { return None; }

        let distance: f64      = self.v0v2.dot(&qvec) * inv_det;

        if distance < EPSILON { return None; }

        return Some(distance);
    }
}