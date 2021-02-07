use nalgebra::Vector3;

use super::camera::Camera;

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