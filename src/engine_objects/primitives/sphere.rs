use nalgebra::Vector3;

use crate::engine_objects::{color::Color, ray::Ray};

use super::primitive::Primitive;

pub struct Sphere {
    pub origin: Vector3<f64>,
    pub radius2: f64,
    pub color: Color,
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

    fn get_color(&self) -> Color {
        return self.color;
    }
}