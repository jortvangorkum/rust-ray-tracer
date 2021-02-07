use nalgebra::{Unit, Vector3};

use super::{camera::Camera, screen::Screen};

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

    pub fn get_intersection_point(self: &Self, dist: f64) -> Vector3<f64> {
        return self.origin + self.direction.scale(dist);
    }

    pub fn update_shadow(self: &mut Self, origin: Vector3<f64>, direction: Unit<Vector3<f64>>) {
        self.origin    = origin;
        self.direction = direction;
    }

    pub fn update_prim(&mut self, x: usize, y: usize, camera: &Camera, screen: &Screen) {
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