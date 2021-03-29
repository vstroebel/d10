use crate::color::{Color, Rgb, clamp, EPSILON, format_color};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct Srgb {
    pub data: [f32; 4]
}

impl Srgb {
    pub fn new(red: f32, green: f32, blue: f32) -> Srgb {
        Srgb { data: [clamp(red), clamp(green), clamp(blue), 1.0] }
    }

    pub fn new_with_alpha(red: f32, green: f32, blue: f32, alpha: f32) -> Srgb {
        Srgb { data: [clamp(red), clamp(green), clamp(blue), clamp(alpha)] }
    }

    pub fn red(&self) -> f32 {
        self.data[0]
    }

    pub fn set_red(&mut self, red: f32) {
        self.data[0] = red;
    }

    pub fn with_red(&self, red: f32) -> Srgb {
        Srgb { data: [red, self.data[1], self.data[2], self.data[3]] }
    }

    pub fn green(&self) -> f32 {
        self.data[1]
    }

    pub fn set_green(&mut self, green: f32) {
        self.data[1] = green;
    }

    pub fn with_green(&self, green: f32) -> Srgb {
        Srgb { data: [self.data[0], green, self.data[2], self.data[3]] }
    }

    pub fn blue(&self) -> f32 {
        self.data[2]
    }

    pub fn set_blue(&mut self, blue: f32) {
        self.data[2] = blue;
    }

    pub fn with_blue(&self, blue: f32) -> Srgb {
        Srgb { data: [self.data[0], self.data[1], blue, self.data[3]] }
    }
}

impl Default for Srgb {
    fn default() -> Srgb {
        Srgb::new_with_alpha(0.0, 0.0, 0.0, 0.0)
    }
}

impl Color for Srgb {
    fn to_rgb(&self) -> Rgb {
        Rgb::new_with_alpha(
            gamma_to_linear(self.data[0]),
            gamma_to_linear(self.data[1]),
            gamma_to_linear(self.data[2]),
            self.data[3],
        )
    }

    fn alpha(&self) -> f32 {
        self.data[3]
    }

    fn set_alpha(&mut self, alpha: f32) {
        self.data[3] = alpha;
    }

    fn with_alpha(&self, alpha: f32) -> Srgb {
        Srgb { data: [self.data[0], self.data[1], self.data[2], alpha] }
    }

    fn data(&self) -> &[f32] {
        &self.data
    }

    fn to_srgb(&self) -> Srgb {
        *self
    }

    fn try_map_color_channels<E, F: FnMut(f32) -> Result<f32, E>>(&self, mut func: F) -> Result<Self, E> {
        Ok(Self::new_with_alpha(
            func(self.data[0])?,
            func(self.data[1])?,
            func(self.data[2])?,
            self.data[3]))
    }

    fn type_name(&self) -> &'static str {
        "srgb"
    }
}

impl PartialEq for Srgb {
    fn eq(&self, other: &Srgb) -> bool {
        for (v1, v2) in self.data.iter().zip(other.data.iter()) {
            if (v1 - v2).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

impl Display for Srgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_color(self, f)
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

#[cfg(test)]
mod tests {
    use crate::color::{Color, Srgb};

    #[test]
    fn type_name() {
        assert_eq!(Srgb::default().type_name(), "srgb");
    }

    #[test]
    fn to_string() {
        assert_eq!(Srgb::new_with_alpha(0.0, 0.0, 0.0, 1.0).to_string(), "srgb(0.0, 0.0, 0.0)");
        assert_eq!(Srgb::new_with_alpha(1.0, 1.0, 1.0, 1.0).to_string(), "srgb(1.0, 1.0, 1.0)");
        assert_eq!(Srgb::new_with_alpha(0.0, 0.0, 0.0, 0.0).to_string(), "srgba(0.0, 0.0, 0.0, 0.0)");
        assert_eq!(Srgb::new_with_alpha(0.3, 0.6, 0.9, 0.5).to_string(), "srgba(0.3, 0.6, 0.9, 0.5)");
        assert_eq!(Srgb::new_with_alpha(0.33, 0.666, 0.999, 0.5555).to_string(), "srgba(0.33, 0.666, 0.999, 0.5555)");
    }

    #[test]
    fn test_setters() {
        let mut color = Srgb::new_with_alpha(0.1, 0.3, 0.5, 0.7);
        assert_eq!(color.red(), 0.1);
        assert_eq!(color.with_red(0.2).red(), 0.2);
        color.set_red(0.2);
        assert_eq!(color.red(), 0.2);

        assert_eq!(color.green(), 0.3);
        assert_eq!(color.with_green(0.4).green(), 0.4);
        color.set_green(0.4);
        assert_eq!(color.green(), 0.4);

        assert_eq!(color.blue(), 0.5);
        assert_eq!(color.with_blue(0.6).blue(), 0.6);
        color.set_blue(0.6);
        assert_eq!(color.blue(), 0.6);

        assert_eq!(color.alpha(), 0.7);
        assert_eq!(color.with_alpha(0.8).alpha(), 0.8);
        color.set_alpha(0.8);
        assert_eq!(color.alpha(), 0.8);
    }
}