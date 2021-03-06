use nalgebra::{Unit, Vector3};

use crate::engine_objects::{Material, Ray, bvh::AABB};

use super::primitive::Primitive;

pub struct Sphere {
    pub origin: Vector3<f64>,
    pub radius2: f64,
    pub material_index: usize,
}

impl Primitive for Sphere {
    fn intersect(self: &Self, ray: &Ray) -> Option<f64> {
        let mut t0: f64; let mut t1: f64;

        let l: Vector3<f64> = self.origin - ray.origin;
        let tca = l.dot(&ray.direction);

        if tca < 0.0 { return None; }
        
        let d2 = l.dot(&l) - tca * tca;
        
        if d2 > self.radius2 { return None; }
        
        let thc = (self.radius2 - d2).sqrt();
        t0 = tca - thc;
        t1 = tca + thc;

        if t0 > t1 { std::mem::swap(&mut t0, &mut t1); }

        if t0 < 0.0 {
            t0 = t1;
            if t0 < 0.0 { return None; }
        }

        return Some(t0);
    }

    fn get_material(&self, materials: &Vec<Material>) -> Material {
        return materials[self.material_index];
    }

    fn get_normal(&self, intersection_point: &Vector3<f64>) -> Unit<Vector3<f64>> {
        return Unit::new_normalize(intersection_point - self.origin);
    }

    fn get_centroid(self: &Self) -> Vector3<f64> {
        return self.origin;
    }

    fn get_bounds(self: &Self) -> AABB {
        let r = self.radius2.sqrt();
        let bmin = Vector3::new(self.origin.x - r, self.origin.y - r, self.origin.z - r);
        let bmax = Vector3::new(self.origin.x + r, self.origin.y + r, self.origin.z + r);
        return AABB {
            bmin,
            bmax,
        };
    }
}