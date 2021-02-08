use super::Color;

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub diffuse_color: Color,
    pub refraction_index: Option<f64>,
    pub specular_cof: f64,
    pub refraction_cof: f64,
    // Alpha: 1.0 = Opaque, Alpha: 0.0 = Transparent
    // pub alpha: f64,
}