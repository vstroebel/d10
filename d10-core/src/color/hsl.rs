use super::{Color, RGB, EPSILON};

#[derive(Debug, Copy, Clone)]
pub struct HSL {
    pub data: [f32; 4]
}

impl HSL {
    pub fn new(h: f32, s: f32, l: f32) -> HSL {
        HSL { data: [h, s, l, 1.0] }
    }

    pub fn new_with_alpha(h: f32, s: f32, l: f32, alpha: f32) -> HSL {
        HSL { data: [h, s, l, alpha] }
    }

    pub fn hue(&self) -> f32 {
        self.data[0]
    }

    pub fn saturation(&self) -> f32 {
        self.data[1]
    }

    pub fn lightness(&self) -> f32 {
        self.data[2]
    }

    pub fn with_hue(&self, hue: f32) -> HSL {
        HSL { data: [hue, self.data[1], self.data[2], self.data[3]] }
    }

    pub fn with_saturation(&self, saturation: f32) -> HSL {
        HSL { data: [self.data[0], saturation, self.data[2], self.data[3]] }
    }

    pub fn with_lightness(&self, lightness: f32) -> HSL {
        HSL { data: [self.data[0], self.data[1], lightness, self.data[3]] }
    }
}

impl Default for HSL {
    fn default() -> HSL {
        HSL {
            data: [0.0, 0.0, 0.0, 0.0]
        }
    }
}

impl Color for HSL {
    fn to_hsl(&self) -> HSL {
        *self
    }

    fn to_rgb(&self) -> RGB {
        let hue = self.hue();
        let saturation = self.saturation();
        let lightness = self.lightness();

        let (red, green, blue) = if saturation == 0.0 {
            // achromatic
            (lightness, lightness, lightness)
        } else {
            fn hue2rgb(p: f32, q: f32, mut t: f32) -> f32 {
                if t < 0.0 { t += 1.0 };
                if t > 1.0 { t -= 1.0 };

                if t < 1.0 / 6.0 { return p + (q - p) * 6.0 * t; };
                if t < 1.0 / 2.0 { return q; };
                if t < 2.0 / 3.0 { return p + (q - p) * (2.0 / 3.0 - t) * 6.0; };

                p
            }

            let q = if lightness < 0.5 { lightness * (1.0 + saturation) } else { lightness + saturation - lightness * saturation };
            let p = 2.0 * lightness - q;

            (hue2rgb(p, q, hue + 1.0 / 3.0),
             hue2rgb(p, q, hue),
             hue2rgb(p, q, hue - 1.0 / 3.0))
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

impl PartialEq for HSL {
    fn eq(&self, other: &HSL) -> bool {
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
    use crate::color::{RGB, HSL, Color};

    const RGB_HSL: [((f32, f32, f32), (f32, f32, f32)); 15] = [
        ((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)),
        ((1.0, 1.0, 1.0), (0.0, 0.0, 1.0)),
        ((0.5, 0.5, 0.5), (0.0, 0.0, 0.5)),
        ((1.0, 0.0, 0.0), (0.0, 1.0, 0.5)),
        ((0.0, 1.0, 0.0), (0.33333334, 1.0, 0.5)),
        ((0.0, 0.0, 1.0), (0.6666667, 1.0, 0.5)),
        ((1.0, 0.5, 0.5), (0.0, 1.0, 0.75)),
        ((0.5, 1.0, 0.5), (0.33333334, 1.0, 0.75)),
        ((0.5, 0.5, 1.0), (0.6666667, 1.0, 0.75)),
        ((0.5, 0.0, 0.0), (0.0, 1.0, 0.25)),
        ((0.0, 0.5, 0.0), (0.33333334, 1.0, 0.25)),
        ((0.0, 0.0, 0.5), (0.6666667, 1.0, 0.25)),
        ((0.5, 0.0, 0.5), (0.8333333, 1.0, 0.25)),
        ((0.5, 0.5, 0.0), (0.16666667, 1.0, 0.25)),
        ((0.0, 0.5, 0.5), (0.5, 1.0, 0.25))];

    #[test]
    fn test_rgb_to_hsl() {
        for (from, to) in &RGB_HSL {
            assert_eq!(RGB::new(from.0, from.1, from.2).to_hsl(), HSL::new(to.0, to.1, to.2),
                       "Error in conversion from {:?} to {:?}", from, to);
        }
    }

    #[test]
    fn test_hsl_to_rgb() {
        for (to, from) in &RGB_HSL {
            assert_eq!(HSL::new(from.0, from.1, from.2).to_rgb(), RGB::new(to.0, to.1, to.2),
                       "Error in conversion from {:?} to {:?}", from, to);
        }
    }
}