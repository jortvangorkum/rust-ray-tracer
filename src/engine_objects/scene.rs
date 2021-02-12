use super::{Material, bvh::BVH, lights::PointLight, primitives::Primitive, ray::Ray};

pub struct Scene {
    pub bvh: BVH,
    pub primitives: Vec<Box<dyn Primitive>>,
    pub lights: Vec<PointLight>,
    pub materials: Vec<Material>,
}

impl Scene {
    pub fn _get_nearest_intersection(&self, ray: &Ray) -> Option<(&Box<dyn Primitive>, f64)> {
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