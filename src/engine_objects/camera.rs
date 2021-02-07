use minifb::{Key, Window};
use nalgebra::{Matrix4, Vector3};

pub struct Camera {
    pub origin: Vector3<f64>,
    pub forward: Vector3<f64>,
    pub up: Vector3<f64>,
    pub fov: f64,
}

impl Camera {
    pub fn update_input(self: &mut Self, window: &Window) {
        // Movement speed
        let speed = if window.is_key_down(Key::LeftShift) { 0.30 } else { 0.15 };

        // Translation
        if window.is_key_down(Key::D) { self.translate_x(speed); }
        if window.is_key_down(Key::A) { self.translate_x(-speed); }
        if window.is_key_down(Key::R) { self.translate_y(speed); }
        if window.is_key_down(Key::F) { self.translate_y(-speed); }
        if window.is_key_down(Key::W) { self.translate_z(speed); }
        if window.is_key_down(Key::S) { self.translate_z(-speed); }
        // Rotation
        if window.is_key_down(Key::Up)    { self.rotate_x(speed); }
        if window.is_key_down(Key::Down)  { self.rotate_x(-speed); }
        if window.is_key_down(Key::Right) { self.rotate_y(speed); }
        if window.is_key_down(Key::Left)  { self.rotate_y(-speed); }
        if window.is_key_down(Key::E)     { self.rotate_z(speed); }
        if window.is_key_down(Key::Q)     { self.rotate_z(-speed); }
    }

    fn translate_x(self: &mut Self, dist: f64) {
        self.origin.x += dist;
    }

    fn translate_y(self: &mut Self, dist: f64) {
        self.origin.y += dist;
    }

    fn translate_z(self: &mut Self, dist: f64) {
        self.origin.z += dist;
    }

    fn rotate_x(self: &mut Self, angle: f64) {
        let rotation_matrix = Matrix4::from_scaled_axis(Vector3::x() * angle);
        self.up = rotation_matrix.transform_vector(&self.up);
        self.forward = rotation_matrix.transform_vector(&self.forward);
    }

    fn rotate_y(self: &mut Self, angle: f64) {
        let rotation_matrix = Matrix4::from_scaled_axis(Vector3::y() * angle);
        self.up = rotation_matrix.transform_vector(&self.up);
        self.forward = rotation_matrix.transform_vector(&self.forward);
    }

    fn rotate_z(self: &mut Self, angle: f64) {
        let rotation_matrix = Matrix4::from_scaled_axis(Vector3::z() * angle);
        self.up = rotation_matrix.transform_vector(&self.up);
        self.forward = rotation_matrix.transform_vector(&self.forward);
    }
}