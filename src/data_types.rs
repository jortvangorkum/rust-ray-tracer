use std::{ops::Add, ops::{AddAssign, Deref}};

use nalgebra::{Unit, Vector3, clamp};

#[derive(Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn to_u32(self: &Self) -> u32 {
        let r: u32 = clamp((self.red * 256.0) as u32, 0, 255);
        let g: u32 = clamp((self.green * 256.0) as u32, 0, 255);
        let b: u32 = clamp((self.blue * 256.0) as u32, 0, 255);
        return (r << 16) + (g << 8) + b;
    }
    
    pub fn black() -> Color {
        return Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        };
    }

    pub fn red() -> Color {
        return Color {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
        };
    }

    pub fn blue() -> Color {
        return Color {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
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

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn get_color(&self) -> Color;
}

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

pub struct Scene {
    pub primitives: Vec<Box<dyn Primitive>>,
}

impl Scene {
    pub fn get_nearest_intersection(&self, ray: &Ray) -> Option<(&Box<dyn Primitive>, f64)> {
        let mut nearest_intersection: Option<(&Box<dyn Primitive>, f64)> = None;
        
        for primitive in &self.primitives {
            let intersection = primitive.intersect(&ray);
            if let Some(distance) = intersection {
                match nearest_intersection {
                    None => nearest_intersection = Some((primitive, distance)),
                    Some((_, nearest_distance)) => {
                        if distance < nearest_distance {
                            nearest_intersection = Some((primitive, distance));
                        }
                    }
                }
            }

        }

        return nearest_intersection;
    }
}

pub struct Camera {
    pub origin: Vector3<f64>,
    pub forward: Vector3<f64>,
    pub up: Vector3<f64>,
    pub fov: f64,
}

pub struct Screen {
    pub width: u32,
    pub height: u32,
    pub center: Vector3<f64>,
    pub ratio: f64,
    pub p0: Vector3<f64>,
    pub p1: Vector3<f64>,
    pub p2: Vector3<f64>,
}

impl Screen {
    pub fn create_screen(camera: &Camera, width: u32, height: u32) -> Screen {
        let pi = std::f64::consts::PI;
        let distance = 1.0 / (camera.fov / 2.0 * pi / 180.0).tan();
        let ratio = width as f64 / height as f64;
        let center: Vector3<f64> = camera.origin + distance * camera.forward; 
        let r: Vector3<f64> = camera.up.cross(&camera.forward);
        let u: Vector3<f64> = camera.forward.cross(&r); 

        Screen {
            width,
            height,
            center,
            ratio,
            p0: center - r * ratio + u,
            p1: center + r * ratio + u,
            p2: center - r * ratio - u,
        }
    }
}

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Unit<Vector3<f64>>,
}

impl Ray {
    pub fn initial() -> Ray {
        return Ray {
            origin: Vector3::zeros(),
            direction: Unit::new_normalize(Vector3::zeros()),
        };
    }

    pub fn update(&mut self, x: usize, y: usize, camera: &Camera, screen: &Screen) {
        let w: f64 = screen.width as f64;
        let h: f64 = screen.height as f64;
        let u: f64 = (x as f64) / w;
        let v: f64 = (y as f64) / h;
        let point_on_screen: Vector3<f64>     = screen.p0 + u * (screen.p1 - screen.p0) + v * (screen.p2 - screen.p0);
        let ray_direction: Unit<Vector3<f64>> = Unit::new_normalize(point_on_screen - camera.origin);
        let ray_origin: Vector3<f64>          = camera.origin;

        self.origin    = ray_origin;
        self.direction = ray_direction;
    }
}