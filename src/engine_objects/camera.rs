use nalgebra::Vector3;

pub struct Camera {
    pub origin: Vector3<f64>,
    pub forward: Vector3<f64>,
    pub up: Vector3<f64>,
    pub fov: f64,
}