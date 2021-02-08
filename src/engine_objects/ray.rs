use nalgebra::{Unit, Vector3, clamp};

use crate::{EPSILON, RECURSION_LIMIT};

use super::{Camera, Color, Material, Scene, Screen, lights::PointLight, primitives::Primitive};

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

    fn determine_diffuse_color(scene: &Scene, shadow_ray: &mut Ray, material: &Material, intersection_point: &Vector3<f64>, normal: &Vector3<f64>) -> Color {
        let energy = Ray::calculate_light_energy(scene, shadow_ray, &intersection_point, &normal);
        let color = material.diffuse_color * energy;
        return color;
    }

    fn determine_specular_color(self: &mut Self, scene: &Scene, shadow_ray: &mut Ray, intersection_point: &Vector3<f64>, normal: &Vector3<f64>, depth: &u32) -> Color {
        let reflect_dir: Vector3<f64> = self.direction.into_inner() - normal.scale(2.0 * normal.dot(&self.direction));
        self.origin = intersection_point + (reflect_dir.scale(EPSILON));
        self.direction = Unit::new_normalize(reflect_dir);
        return self.trace(scene, shadow_ray, depth + 1);
    }

    fn get_refractive_direction(self: &Self, material: &Material, normal: &Vector3<f64>) -> Option<Unit<Vector3<f64>>> {
        let mut cosi = clamp(self.direction.dot(normal),-1.0, 1.0);
        let mut etai = 1.0;
        let mut etat = material.refraction_index
            .expect("No refraction index is defined for this material");
        let mut normal_refraction = *normal;

        if cosi < 0.0 { cosi = -cosi; }
        else {
            normal_refraction = -normal_refraction;
            std::mem::swap(&mut etai, &mut etat);
        }

        let eta = etai / etat;
        let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
        
        if k < 0.0 { return None; }
        
        return Some(Unit::new_normalize(self.direction.scale(eta) + (eta * cosi - k.sqrt()) * normal_refraction));
    }

    fn determine_refractive_color(self: &mut Self, scene: &Scene, shadow_ray: &mut Ray, intersection_point: &Vector3<f64>, normal: &Vector3<f64>, material: &Material, depth: &u32) -> Color {
        let refractive_direction = self.get_refractive_direction(material, normal)
            .expect("No refractive direction could be calculated");
        self.origin = intersection_point + (refractive_direction.scale(EPSILON));
        self.direction = refractive_direction;
        return self.trace(scene, shadow_ray, depth + 1);
    }

    pub fn trace(self: &mut Self, scene: &Scene, shadow_ray: &mut Ray, depth: u32) -> Color {
        if depth > RECURSION_LIMIT { return Color::black(); }

        let intersection = scene.get_nearest_intersection(self);
        
        if let Some((primitive, distance)) = intersection {
            let intersection_point: Vector3<f64> = self.get_intersection_point(distance);
            let normal = primitive.get_normal(&intersection_point);
            let material = primitive.get_material(&scene.materials);
            let diffuse_cof = 1.0 - material.specular_cof - material.refraction_cof;
            let mut color = Color::black();

            if diffuse_cof > EPSILON {
                let diffuse_color = Ray::determine_diffuse_color(scene, shadow_ray, &material, &intersection_point, &normal);
                color += diffuse_color * diffuse_cof;
            }

            if material.specular_cof > EPSILON {
                let specular_color = self.determine_specular_color(scene, shadow_ray, &intersection_point, &normal, &depth);
                color += specular_color * material.specular_cof;
            }

            if material.refraction_cof > EPSILON {
                let refractive_color = self.determine_refractive_color(scene, shadow_ray, &intersection_point, &normal, &material, &depth);
                color += refractive_color * material.refraction_cof;
            }

            return color;
        }

        return Color::black();
    }
}