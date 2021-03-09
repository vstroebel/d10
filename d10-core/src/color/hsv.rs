use super::{Color, RGB, EPSILON};

#[derive(Debug, Copy, Clone)]
pub struct HSV {
    pub data: [f32; 4]
}

impl HSV {
    pub fn new(h: f32, s: f32, v: f32) -> HSV {
        HSV { data: [h, s, v, 1.0] }
    }

    pub fn new_with_alpha(h: f32, s: f32, v: f32, alpha: f32) -> HSV {
        HSV { data: [h, s, v, alpha] }
    }

    pub fn hue(&self) -> f32 {
        self.data[0]
    }

    pub fn saturation(&self) -> f32 {
        self.data[1]
    }

    pub fn value(&self) -> f32 {
        self.data[2]
    }

    pub fn with_hue(&self, hue: f32) -> HSV {
        HSV { data: [hue, self.data[1], self.data[2], self.data[3]] }
    }

    pub fn with_saturation(&self, saturation: f32) -> HSV {
        HSV { data: [self.data[0], saturation, self.data[2], self.data[3]] }
    }

    pub fn with_value(&self, value: f32) -> HSV {
        HSV { data: [self.data[0], self.data[1], value, self.data[3]] }
    }
}

impl Default for HSV {
    fn default() -> HSV {
        HSV {
            data: [0.0, 0.0, 0.0, 0.0]
        }
    }
}

impl Color for HSV {
    fn to_hsv(&self) -> HSV {
        *self
    }

    fn to_rgb(&self) -> RGB {
        let hue = self.hue() * 360.0;
        let saturation = self.saturation();
        let value = self.value();


        if saturation <= 0.0 {
            return RGB {
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

        RGB {
            data: [red, green, blue, self.alpha()]
        }
    }

    fn alpha(&self) -> f32 {
        self.data[3]
    }

    fn data(&self) -> &[f32] {
        &self.data
    }
}

impl PartialEq for HSV {
    fn eq(&self, other: &HSV) -> bool {
        for (v1, v2) in self.data.iter().zip(other.data.iter()) {
            if (v1 - v2).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::color::{RGB, HSV, Color};

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
            assert_eq!(RGB::new(from.0, from.1, from.2).to_hsv(), HSV::new(to.0, to.1, to.2),
                       "Error in conversion from {:?} to {:?}", from, to);
        }
    }

    #[test]
    fn test_hsv_to_rgb() {
        for (to, from) in &RGB_HSV {
            assert_eq!(HSV::new(from.0, from.1, from.2).to_rgb(), RGB::new(to.0, to.1, to.2),
                       "Error in conversion from {:?} to {:?}", from, to);
        }
    }
}