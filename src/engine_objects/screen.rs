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
    pub fn new(camera: &Camera, width: u32, height: u32) -> Screen {
        let pi = std::f64::consts::PI;
        let distance = 1.0 / (camera.fov / 2.0 * pi / 180.0).tan();
        let ratio = width as f64 / height as f64;
        let center: Vector3<f64> = camera.origin + distance * camera.forward; 
        let r: Vector3<f64> = camera.up.cross(&camera.forward);
        let u: Vector3<f64> = camera.up; 

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

    pub fn update_screen(self: &mut Self, camera: &Camera) {
        let pi = std::f64::consts::PI;
        let distance = 1.0 / (camera.fov / 2.0 * pi / 180.0).tan();
        let center: Vector3<f64> = camera.origin + distance * camera.forward; 
        let r: Vector3<f64> = camera.up.cross(&camera.forward);
        let u: Vector3<f64> = camera.forward.cross(&r); 

        self.p0 = center - r * self.ratio + u;
        self.p1 = center + r * self.ratio + u;
        self.p2 = center - r * self.ratio - u;
    }
}