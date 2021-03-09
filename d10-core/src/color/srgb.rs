use crate::color::{Color, RGB, clamp, EPSILON};

#[derive(Debug, Copy, Clone)]
pub struct SRGB {
    pub data: [f32; 4]
}

impl SRGB {
    pub fn new(red: f32, green: f32, blue: f32) -> SRGB {
        SRGB { data: [clamp(red), clamp(green), clamp(blue), 1.0] }
    }

    pub fn new_with_alpha(red: f32, green: f32, blue: f32, alpha: f32) -> SRGB {
        SRGB { data: [clamp(red), clamp(green), clamp(blue), clamp(alpha)] }
    }

    pub fn red(&self) -> f32 {
        self.data[0]
    }

    pub fn green(&self) -> f32 {
        self.data[1]
    }

    pub fn blue(&self) -> f32 {
        self.data[2]
    }
}

impl Default for SRGB {
    fn default() -> SRGB {
        SRGB::new_with_alpha(0.0, 0.0, 0.0, 0.0)
    }
}

impl Color for SRGB {
    fn to_rgb(&self) -> RGB {
        RGB::new_with_alpha(
            gamma_to_linear(self.data[0]),
            gamma_to_linear(self.data[1]),
            gamma_to_linear(self.data[2]),
            self.data[3],
        )
    }

    fn alpha(&self) -> f32 {
        self.data[3]
    }

    fn data(&self) -> &[f32] {
        &self.data
    }

    fn to_srgb(&self) -> SRGB {
        *self
    }
}

impl PartialEq for SRGB {
    fn eq(&self, other: &SRGB) -> bool {
        for (v1, v2) in self.data.iter().zip(other.data.iter()) {
            if (v1 - v2).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

/// Convert a single component value from sRGB to linear RGB
pub fn gamma_to_linear(value: f32) -> f32 {
    if value > 0.04045 {
        ((value + 0.055) / 1.055).powf(2.4)
    } else {
        value / 12.92
    }
}

/// Convert a single component value from linear RGB to sRGB
pub fn linear_to_gamma(value: f32) -> f32 {
    if value > 0.003_130_805 {
        1.055 * value.powf(1.0 / 2.4) - 0.055
    } else {
        12.92 * value
    }
}
