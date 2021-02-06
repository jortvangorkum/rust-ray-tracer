use std::{ops::Add, ops::AddAssign};

use nalgebra::{Unit, Vector3};

#[derive(Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn to_u32(self: &Self) -> u32 {
        let r: u32 = self.red as u32;
        let g: u32 = self.green as u32;
        let b: u32 = self.blue as u32;
        return r << 16 + g << 8 + b;
    }

    pub fn black() -> Color {
        return Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        };
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self: Self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        *self = Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

pub struct Sphere {
    pub origin: Vector3<f64>,
    pub radius2: f64,
    pub color: Color,
}

impl Sphere {
    pub fn intersect(self: &Self, ray: &Ray) -> (bool, f64) {
        let mut t0: f64; let mut t1: f64;

        let L: Vector3<f64> = self.origin - ray.origin;
        let tca = L.dot(&ray.direction);

        if tca < 0.0 { return (false, 0.0); }
        
        let d2 = L.dot(&L) - tca * tca;
        
        if d2 > self.radius2 { return (false, 0.0); }
        
        let thc = (self.radius2 - d2).sqrt();
        t0 = tca - thc;
        t1 = tca + thc;

        if t0 > t1 { std::mem::swap(&mut t0, &mut t1); }

        if t0 < 0.0 {
            t0 = t1;
            if t0 < 0.0 { return (false, 0.0); }
        }

        return (true, t0);
    }
}

pub struct Scene {
    pub sphere: Sphere,
}

pub struct Camera {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

pub struct Screen {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub center: Vector3<f64>,
    pub p0: Vector3<f64>,
    pub p1: Vector3<f64>,
    pub p2: Vector3<f64>,
}

impl Screen {
    pub fn create_screen(camera: &Camera, width: u32, height: u32, fov: f64) -> Screen {
        let pi = std::f64::consts::PI;
        let distance = 1f64 / (fov / 2f64 * pi / 180f64).tan();
        let center: Vector3<f64> = camera.origin + distance * camera.direction; 

        Screen {
            width,
            height,
            fov,
            center,
            p0: center + Vector3::new(-1.0, 1.0, 0.0),
            p1: center + Vector3::new(1.0, 1.0, 0.0),
            p2: center + Vector3::new(-1.0, -1.0, 0.0),
        }
    }
}

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Unit<Vector3<f64>>,
}

impl Ray {
    pub fn create_prime(x: usize, y: usize, camera: &Camera, screen: &Screen) -> Ray {
        let u: f64 = (x as f64) / (screen.width as f64);
        let v: f64 = (y as f64) / (screen.height as f64);
        let point_on_screen: Vector3<f64> = screen.p0 + u * (screen.p1 - screen.p0) + v * (screen.p2 - screen.p0);
        let ray_direction: Unit<Vector3<f64>> = Unit::new_normalize(point_on_screen - camera.origin);
        let ray_origin: Vector3<f64> = camera.origin;

        return Ray {
            origin: ray_origin,
            direction: ray_direction,
        }
    }
}