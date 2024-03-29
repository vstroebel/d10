use std::array::from_fn;
use super::{format_color, Color, Rgb, EPSILON};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct Hsl {
    pub data: [f32; 4],
}

impl Hsl {
    pub fn new(h: f32, s: f32, l: f32) -> Hsl {
        Hsl {
            data: [h, s, l, 1.0],
        }
    }

    pub fn new_with_alpha(h: f32, s: f32, l: f32, alpha: f32) -> Hsl {
        Hsl {
            data: [h, s, l, alpha],
        }
    }

    pub fn new_from_fn<F: Fn(usize) -> f32>(func: F) -> Hsl {
        Hsl {
            data: from_fn(func)
        }
    }

    pub fn hue(&self) -> f32 {
        self.data[0]
    }

    pub fn set_hue(&mut self, hue: f32) {
        self.data[0] = hue;
    }

    pub fn saturation(&self) -> f32 {
        self.data[1]
    }

    pub fn set_saturation(&mut self, saturation: f32) {
        self.data[1] = saturation;
    }

    pub fn lightness(&self) -> f32 {
        self.data[2]
    }

    pub fn set_lightness(&mut self, lightness: f32) {
        self.data[2] = lightness;
    }

    pub fn with_hue(&self, hue: f32) -> Hsl {
        Hsl {
            data: [hue, self.data[1], self.data[2], self.data[3]],
        }
    }

    pub fn with_saturation(&self, saturation: f32) -> Hsl {
        Hsl {
            data: [self.data[0], saturation, self.data[2], self.data[3]],
        }
    }

    pub fn with_lightness(&self, lightness: f32) -> Hsl {
        Hsl {
            data: [self.data[0], self.data[1], lightness, self.data[3]],
        }
    }
}

impl Default for Hsl {
    fn default() -> Hsl {
        Hsl {
            data: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

impl Color for Hsl {
    fn to_hsl(&self) -> Hsl {
        *self
    }

    fn to_rgb(&self) -> Rgb {
        let hue = self.hue();
        let saturation = self.saturation();
        let lightness = self.lightness();

        let (red, green, blue) = if saturation == 0.0 {
            // achromatic
            (lightness, lightness, lightness)
        } else {
            fn hue2rgb(p: f32, q: f32, mut t: f32) -> f32 {
                if t < 0.0 {
                    t += 1.0
                };
                if t > 1.0 {
                    t -= 1.0
                };

                if t < 1.0 / 6.0 {
                    return p + (q - p) * 6.0 * t;
                };
                if t < 1.0 / 2.0 {
                    return q;
                };
                if t < 2.0 / 3.0 {
                    return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
                };

                p
            }

            let q = if lightness < 0.5 {
                lightness * (1.0 + saturation)
            } else {
                lightness + saturation - lightness * saturation
            };
            let p = 2.0 * lightness - q;

            (
                hue2rgb(p, q, hue + 1.0 / 3.0),
                hue2rgb(p, q, hue),
                hue2rgb(p, q, hue - 1.0 / 3.0),
            )
        };

        Rgb::new_with_alpha(red, green, blue, self.alpha())
    }

    fn alpha(&self) -> f32 {
        self.data[3]
    }

    fn set_alpha(&mut self, alpha: f32) {
        self.data[3] = alpha;
    }

    fn with_alpha(&self, alpha: f32) -> Hsl {
        Hsl {
            data: [self.data[0], self.data[1], self.data[2], alpha],
        }
    }

    fn data(&self) -> &[f32] {
        &self.data
    }

    fn data_mut(&mut self) -> &mut [f32] {
        &mut self.data
    }

    fn try_map_color_channels<E, F: FnMut(f32) -> Result<f32, E>>(
        &self,
        mut func: F,
    ) -> Result<Self, E> {
        Ok(Self::new_with_alpha(
            func(self.data[0])?,
            func(self.data[1])?,
            func(self.data[2])?,
            self.data[3],
        ))
    }

    fn type_name(&self) -> &'static str {
        "hsl"
    }
}

impl PartialEq for Hsl {
    fn eq(&self, other: &Hsl) -> bool {
        for (v1, v2) in self.data.iter().zip(other.data.iter()) {
            if (v1 - v2).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

impl Display for Hsl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_color(self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::{Color, Hsl, Rgb};

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
        ((0.0, 0.5, 0.5), (0.5, 1.0, 0.25)),
    ];

    #[test]
    fn test_rgb_to_hsl() {
        for (from, to) in &RGB_HSL {
            assert_eq!(
                Rgb::new(from.0, from.1, from.2).to_hsl(),
                Hsl::new(to.0, to.1, to.2),
                "Error in conversion from {:?} to {:?}",
                from,
                to
            );
        }
    }

    #[test]
    fn test_hsl_to_rgb() {
        for (to, from) in &RGB_HSL {
            assert_eq!(
                Hsl::new(from.0, from.1, from.2).to_rgb(),
                Rgb::new(to.0, to.1, to.2),
                "Error in conversion from {:?} to {:?}",
                from,
                to
            );
        }
    }

    #[test]
    fn type_name() {
        assert_eq!(Hsl::default().type_name(), "hsl");
    }

    #[test]
    fn to_string() {
        assert_eq!(
            Hsl::new_with_alpha(0.0, 0.0, 0.0, 1.0).to_string(),
            "hsl(0.0, 0.0, 0.0)"
        );
        assert_eq!(
            Hsl::new_with_alpha(1.0, 1.0, 1.0, 1.0).to_string(),
            "hsl(1.0, 1.0, 1.0)"
        );
        assert_eq!(
            Hsl::new_with_alpha(0.0, 0.0, 0.0, 0.0).to_string(),
            "hsla(0.0, 0.0, 0.0, 0.0)"
        );
        assert_eq!(
            Hsl::new_with_alpha(0.3, 0.6, 0.9, 0.5).to_string(),
            "hsla(0.3, 0.6, 0.9, 0.5)"
        );
        assert_eq!(
            Hsl::new_with_alpha(0.33, 0.666, 0.999, 0.5555).to_string(),
            "hsla(0.33, 0.666, 0.999, 0.5555)"
        );
    }

    #[test]
    fn test_setters() {
        let mut color = Hsl::new_with_alpha(0.1, 0.3, 0.5, 0.7);
        assert_eq!(color.hue(), 0.1);
        assert_eq!(color.with_hue(0.2).hue(), 0.2);
        color.set_hue(0.2);
        assert_eq!(color.hue(), 0.2);

        assert_eq!(color.saturation(), 0.3);
        assert_eq!(color.with_saturation(0.4).saturation(), 0.4);
        color.set_saturation(0.4);
        assert_eq!(color.saturation(), 0.4);

        assert_eq!(color.lightness(), 0.5);
        assert_eq!(color.with_lightness(0.6).lightness(), 0.6);
        color.set_lightness(0.6);
        assert_eq!(color.lightness(), 0.6);

        assert_eq!(color.alpha(), 0.7);
        assert_eq!(color.with_alpha(0.8).alpha(), 0.8);
        color.set_alpha(0.8);
        assert_eq!(color.alpha(), 0.8);
    }
}
