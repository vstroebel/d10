use super::{Color, format_color, Rgb, EPSILON};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct Hsv {
    pub data: [f32; 4],
}

impl Hsv {
    pub fn new(h: f32, s: f32, v: f32) -> Hsv {
        Hsv { data: [h, s, v, 1.0] }
    }

    pub fn new_with_alpha(h: f32, s: f32, v: f32, alpha: f32) -> Hsv {
        Hsv { data: [h, s, v, alpha] }
    }

    pub fn hue(&self) -> f32 {
        self.data[0]
    }

    pub fn set_hue(&mut self, hue: f32) {
        self.data[0] = hue;
    }

    pub fn with_hue(&self, hue: f32) -> Hsv {
        Hsv { data: [hue, self.data[1], self.data[2], self.data[3]] }
    }

    pub fn saturation(&self) -> f32 {
        self.data[1]
    }

    pub fn set_saturation(&mut self, saturation: f32) {
        self.data[1] = saturation;
    }

    pub fn with_saturation(&self, saturation: f32) -> Hsv {
        Hsv { data: [self.data[0], saturation, self.data[2], self.data[3]] }
    }

    pub fn value(&self) -> f32 {
        self.data[2]
    }

    pub fn set_value(&mut self, value: f32) {
        self.data[2] = value;
    }

    pub fn with_value(&self, value: f32) -> Hsv {
        Hsv { data: [self.data[0], self.data[1], value, self.data[3]] }
    }
}

impl Default for Hsv {
    fn default() -> Hsv {
        Hsv {
            data: [0.0, 0.0, 0.0, 0.0]
        }
    }
}

impl Color for Hsv {
    fn to_hsv(&self) -> Hsv {
        *self
    }

    fn to_rgb(&self) -> Rgb {
        let hue = self.hue() * 360.0;
        let saturation = self.saturation();
        let value = self.value();


        if saturation <= 0.0 {
            return Rgb {
                data: [value, value, value, self.alpha()]
            };
        }

        let mut hh = hue;
        if hh >= 360.0 { hh = 0.0 };

        hh /= 60.0;

        let i = hh as u32;
        let ff = hh - i as f32;

        let p = value * (1.0 - saturation);
        let q = value * (1.0 - (saturation * ff));
        let t = value * (1.0 - (saturation * (1.0 - ff)));

        let (red, green, blue) = match i {
            0 => (value, t, p),
            1 => (q, value, p),
            2 => (p, value, t),
            3 => (p, q, value),
            4 => (t, p, value),
            _ => (value, p, q)
        };

        Rgb {
            data: [red, green, blue, self.alpha()]
        }
    }

    fn alpha(&self) -> f32 {
        self.data[3]
    }

    fn set_alpha(&mut self, alpha: f32) {
        self.data[3] = alpha;
    }

    fn with_alpha(&self, alpha: f32) -> Hsv {
        Hsv { data: [self.data[0], self.data[1], self.data[2], alpha] }
    }

    fn data(&self) -> &[f32] {
        &self.data
    }

    fn try_map_color_channels<E, F: FnMut(f32) -> Result<f32, E>>(&self, mut func: F) -> Result<Self, E> {
        Ok(Self::new_with_alpha(
            func(self.data[0])?,
            func(self.data[1])?,
            func(self.data[2])?,
            self.data[3]))
    }

    fn type_name(&self) -> &'static str {
        "hsv"
    }
}

impl PartialEq for Hsv {
    fn eq(&self, other: &Hsv) -> bool {
        for (v1, v2) in self.data.iter().zip(other.data.iter()) {
            if (v1 - v2).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

impl Display for Hsv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_color(self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::{Rgb, Hsv, Color};

    const RGB_HSV: [((f32, f32, f32), (f32, f32, f32)); 15] = [
        ((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)),
        ((1.0, 1.0, 1.0), (0.0, 0.0, 1.0)),
        ((0.5, 0.5, 0.5), (0.0, 0.0, 0.5)),
        ((1.0, 0.0, 0.0), (0.0, 1.0, 1.0)),
        ((0.0, 1.0, 0.0), (0.33333334, 1.0, 1.0)),
        ((0.0, 0.0, 1.0), (0.6666667, 1.0, 1.0)),
        ((1.0, 0.5, 0.5), (0.0, 0.5, 1.0)),
        ((0.5, 1.0, 0.5), (0.33333334, 0.5, 1.0)),
        ((0.5, 0.5, 1.0), (0.6666667, 0.5, 1.0)),
        ((0.5, 0.0, 0.0), (0.0, 1.0, 0.5)),
        ((0.0, 0.5, 0.0), (0.33333334, 1.0, 0.5)),
        ((0.0, 0.0, 0.5), (0.6666667, 1.0, 0.5)),
        ((0.5, 0.0, 0.5), (0.8333333, 1.0, 0.5)),
        ((0.5, 0.5, 0.0), (0.16666667, 1.0, 0.5)),
        ((0.0, 0.5, 0.5), (0.5, 1.0, 0.5))];

    #[test]
    fn test_rgb_to_hsv() {
        for (from, to) in &RGB_HSV {
            assert_eq!(Rgb::new(from.0, from.1, from.2).to_hsv(), Hsv::new(to.0, to.1, to.2),
                       "Error in conversion from {:?} to {:?}", from, to);
        }
    }

    #[test]
    fn test_hsv_to_rgb() {
        for (to, from) in &RGB_HSV {
            assert_eq!(Hsv::new(from.0, from.1, from.2).to_rgb(), Rgb::new(to.0, to.1, to.2),
                       "Error in conversion from {:?} to {:?}", from, to);
        }
    }

    #[test]
    fn type_name() {
        assert_eq!(Hsv::default().type_name(), "hsv");
    }

    #[test]
    fn to_string() {
        assert_eq!(Hsv::new_with_alpha(0.0, 0.0, 0.0, 1.0).to_string(), "hsv(0.0, 0.0, 0.0)");
        assert_eq!(Hsv::new_with_alpha(1.0, 1.0, 1.0, 1.0).to_string(), "hsv(1.0, 1.0, 1.0)");
        assert_eq!(Hsv::new_with_alpha(0.0, 0.0, 0.0, 0.0).to_string(), "hsva(0.0, 0.0, 0.0, 0.0)");
        assert_eq!(Hsv::new_with_alpha(0.3, 0.6, 0.9, 0.5).to_string(), "hsva(0.3, 0.6, 0.9, 0.5)");
        assert_eq!(Hsv::new_with_alpha(0.33, 0.666, 0.999, 0.5555).to_string(), "hsva(0.33, 0.666, 0.999, 0.5555)");
    }

    #[test]
    fn test_setters() {
        let mut color = Hsv::new_with_alpha(0.1, 0.3, 0.5, 0.7);
        assert_eq!(color.hue(), 0.1);
        assert_eq!(color.with_hue(0.2).hue(), 0.2);
        color.set_hue(0.2);
        assert_eq!(color.hue(), 0.2);

        assert_eq!(color.saturation(), 0.3);
        assert_eq!(color.with_saturation(0.4).saturation(), 0.4);
        color.set_saturation(0.4);
        assert_eq!(color.saturation(), 0.4);

        assert_eq!(color.value(), 0.5);
        assert_eq!(color.with_value(0.6).value(), 0.6);
        color.set_value(0.6);
        assert_eq!(color.value(), 0.6);

        assert_eq!(color.alpha(), 0.7);
        assert_eq!(color.with_alpha(0.8).alpha(), 0.8);
        color.set_alpha(0.8);
        assert_eq!(color.alpha(), 0.8);
    }
}