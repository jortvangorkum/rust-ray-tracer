use nalgebra::{Unit, Vector3};

use crate::EPSILON;

use super::{Scene, Camera, Color, lights::PointLight, Screen};

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Unit<Vector3<f64>>,
}

impl Ray {
    pub fn init() -> Ray {
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

    fn calculate_light_energy(scene: &Scene, shadow_ray: &mut Ray, intersection_point: &Vector3<f64>, normal: &Vector3<f64>) -> f64 {
        let mut energy = 0.0;
        for light in scene.lights.iter() {
            let light_vector: Vector3<f64>    = light.origin - intersection_point;
            let distance: f64                 = light_vector.magnitude() - (light_vector.magnitude() * EPSILON);
            let direction: Unit<Vector3<f64>> = Unit::new_normalize(light.origin - intersection_point);
            let origin: Vector3<f64>          = intersection_point + (direction.scale( EPSILON));

            shadow_ray.update_shadow(origin, direction);

            if !PointLight::occluded(&scene, shadow_ray, distance) {
                let dist_falloff = 1.0 / (distance * distance);
                let angle_falloff = normal.dot(&direction);
                energy +=  dist_falloff * angle_falloff * light.intensity;
            }
        }
        return energy;
    }

    pub fn trace(self: &Self, scene: &Scene, shadow_ray: &mut Ray) -> Color {
        let intersection = scene.get_nearest_intersection(self);
        
        if let Some((primitive, distance)) = intersection {
            let intersection_point: Vector3<f64> = self.get_intersection_point(distance);
            let normal = primitive.get_normal(&intersection_point);

            let energy = Ray::calculate_light_energy(scene, shadow_ray, &intersection_point, &normal);
            let color = primitive.get_material(&scene.materials).diffuse_color * energy;

            return color;
        }

        return Color::black();
    }
}