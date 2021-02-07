use std::ops::{Add, AddAssign};

use nalgebra::clamp;

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

    pub fn green() -> Color {
        return Color {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
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