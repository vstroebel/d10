use super::{clamp, format_color, Color, Hsl, EPSILON};
use crate::errors::ParseEnumError;

use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Intensity {
    Average,
    Rec601Luma,
    Rec709Luma,
    Brightness,
    Lightness,
    Saturation,
    Red,
    Green,
    Blue,
}

impl FromStr for Intensity {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Intensity::*;
        match s {
            "average" => Ok(Average),
            "rec601luma" => Ok(Rec601Luma),
            "rec709luma" | "default" => Ok(Rec709Luma),
            "brightness" => Ok(Brightness),
            "lightness" => Ok(Lightness),
            "saturation" => Ok(Saturation),
            "red" => Ok(Red),
            "green" => Ok(Green),
            "blue" => Ok(Blue),
            _ => Err(ParseEnumError::new(s, "Intensity")),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    pub data: [f32; 4],
}

impl Rgb {
    pub fn new(red: f32, green: f32, blue: f32) -> Rgb {
        Rgb {
            data: [clamp(red), clamp(green), clamp(blue), 1.0],
        }
    }

    pub fn new_with_alpha(red: f32, green: f32, blue: f32, alpha: f32) -> Rgb {
        Rgb {
            data: [clamp(red), clamp(green), clamp(blue), clamp(alpha)],
        }
    }

    pub fn red(&self) -> f32 {
        self.data[0]
    }

    pub fn set_red(&mut self, red: f32) {
        self.data[0] = red;
    }

    pub fn with_red(&self, red: f32) -> Rgb {
        Rgb {
            data: [red, self.data[1], self.data[2], self.data[3]],
        }
    }

    pub fn green(&self) -> f32 {
        self.data[1]
    }

    pub fn set_green(&mut self, green: f32) {
        self.data[1] = green;
    }

    pub fn with_green(&self, green: f32) -> Rgb {
        Rgb {
            data: [self.data[0], green, self.data[2], self.data[3]],
        }
    }

    pub fn blue(&self) -> f32 {
        self.data[2]
    }

    pub fn set_blue(&mut self, blue: f32) {
        self.data[2] = blue;
    }

    pub fn with_blue(&self, blue: f32) -> Rgb {
        Rgb {
            data: [self.data[0], self.data[1], blue, self.data[3]],
        }
    }

    pub fn is_grayscale(&self) -> bool {
        (self.red() - self.green()).abs() < EPSILON && (self.green() - self.blue()).abs() < EPSILON
    }

    pub fn to_gray(self) -> Rgb {
        self.to_gray_with_intensity(Intensity::Rec709Luma)
    }

    pub fn to_gray_with_intensity(self, intensity: Intensity) -> Rgb {
        use Intensity::*;
        let v = match intensity {
            Rec601Luma => {
                self.data[0] * 0.298_839 + self.data[1] * 0.586_811 + self.data[2] * 0.114_350
            }
            Rec709Luma => {
                self.data[0] * 0.212_656 + self.data[1] * 0.715_158 + self.data[2] * 0.072_186
            }
            Average => (self.data[0] + self.data[1] + self.data[2]) / 3.0,
            Brightness => self.max(),
            Lightness => (self.min() + self.max()) / 2.0,
            Saturation => self.to_hsl().saturation(),
            Red => self.red(),
            Green => self.green(),
            Blue => self.blue(),
        };

        Rgb {
            data: [v, v, v, self.alpha()],
        }
    }

    pub fn invert(&self) -> Rgb {
        self.map_channels_unclamped(|v| 1.0 - v)
    }

    pub fn difference(&self, color: &Rgb) -> Rgb {
        Rgb::new(
            (self.red() - color.red()).abs(),
            (self.green() - color.green()).abs(),
            (self.blue() - color.blue()).abs(),
        )
    }

    pub fn with_gamma(&self, gamma: f32) -> Rgb {
        self.map_channels(|v| clamp(v).powf(1.0 / gamma))
    }

    pub fn with_gamma_opt(&self, gamma: f32) -> Rgb {
        self.map_channels(|v| {
            if v > 0.5 {
                clamp((v - 0.5) * 2.0).powf(1.0 / gamma) / 2.0 + 0.5
            } else {
                clamp(v * 2.0).powf(1.0 * gamma) / 2.0
            }
        })
    }

    pub fn with_level(&self, black_point: f32, white_point: f32, gamma: f32) -> Rgb {
        self.map_channels(|v| {
            let diff = white_point - black_point;
            let factor = if diff.abs() < f32::EPSILON {
                1.0 / EPSILON
            } else {
                1.0 / diff
            };

            let v = v - black_point;
            let v = v * factor;

            clamp(v).powf(1.0 / gamma)
        })
    }

    pub fn with_brightness(&self, factor: f32) -> Rgb {
        self.map_channels(|v| v + factor)
    }

    pub fn with_saturation(&self, factor: f32) -> Rgb {
        let hsl = self.to_hsl();
        Hsl {
            data: [
                hsl.hue(),
                clamp(hsl.saturation() * factor),
                hsl.lightness(),
                self.alpha(),
            ],
        }
        .to_rgb()
    }

    pub fn stretch_saturation(&self, factor: f32) -> Rgb {
        let hsl = self.to_hsl();

        let s = hsl.saturation() - 0.5;
        let s = (s * factor) + 0.5;

        Hsl {
            data: [hsl.hue(), clamp(s), hsl.lightness(), self.alpha()],
        }
        .to_rgb()
    }

    pub fn with_gamma_saturation(&self, gamma: f32) -> Rgb {
        let hsl = self.to_hsl();
        Hsl {
            data: [
                hsl.hue(),
                clamp(hsl.saturation()).powf(1.0 / gamma),
                hsl.lightness(),
                self.alpha(),
            ],
        }
        .to_rgb()
    }

    pub fn with_lightness(&self, factor: f32) -> Rgb {
        let hsl = self.to_hsl();
        Hsl {
            data: [
                hsl.hue(),
                hsl.saturation(),
                clamp(hsl.lightness() * factor),
                self.alpha(),
            ],
        }
        .to_rgb()
    }

    pub fn with_hue_rotate(&self, radians: f32) -> Rgb {
        let hsl = self.to_hsl();

        let mut hue = hsl.hue() + radians / 360.0;
        if hue >= 1.0 {
            hue -= 1.0;
        } else if hue < 0.0 {
            hue += 1.0;
        }

        Hsl {
            data: [hue, hsl.saturation(), hsl.lightness(), self.alpha()],
        }
        .to_rgb()
    }

    pub fn with_contrast(&self, factor: f32) -> Rgb {
        self.map_channels(|v| (v - 0.5) * factor + 0.5)
    }

    pub fn with_brightness_contrast(&self, brightness: f32, contrast: f32) -> Rgb {
        self.map_channels(|v| (v + brightness - 0.5) * contrast + 0.5)
    }

    pub fn alpha_blend(&self, color: Rgb) -> Rgb {
        Rgb::new_with_alpha(
            color.data[0] * color.alpha() + (1.0 - color.alpha()) * self.data[0],
            color.data[1] * color.alpha() + (1.0 - color.alpha()) * self.data[1],
            color.data[2] * color.alpha() + (1.0 - color.alpha()) * self.data[2],
            (self.alpha() + color.alpha()).min(1.0),
        )
    }

    pub fn with_vibrance(&self, factor: f32) -> Rgb {
        //TODO: Find a good algorithm for this

        /*
         * Increase saturation using a sinus function based on the original saturation.
         */

        let hsl = self.to_hsl();

        let s = hsl.saturation();

        let mut scale = factor;

        // Try to not saturate red colors a much as others
        scale *= ((hsl.hue() * std::f32::consts::PI).sin() * 1.5).min(1.0);

        let s = s + (s * std::f32::consts::PI).sin() * scale;

        Hsl::new(hsl.hue(), s.min(1.0).max(0.0), hsl.lightness()).to_rgb()
    }

    pub fn with_sepia(&self) -> Rgb {
        let red = (self.red() * 0.393) + (self.green() * 0.769) + (self.blue() * 0.189);
        let green = (self.red() * 0.349) + (self.green() * 0.686) + (self.blue() * 0.168);
        let blue = (self.red() * 0.272) + (self.green() * 0.534) + (self.blue() * 0.131);

        Rgb::new_with_alpha(red, green, blue, self.alpha())
    }

    pub fn mod_color_channels<F: Fn(f32) -> f32>(&mut self, func: F) {
        for i in 0..3 {
            self.data[i] = func(self.data[i]);
        }
    }

    pub fn map_channels_unclamped<F: Fn(f32) -> f32>(&self, func: F) -> Rgb {
        Rgb {
            data: [
                func(self.data[0]),
                func(self.data[1]),
                func(self.data[2]),
                self.alpha(),
            ],
        }
    }

    pub fn map_channels<F: Fn(f32) -> f32>(&self, func: F) -> Rgb {
        Rgb {
            data: [
                clamp(func(self.data[0])),
                clamp(func(self.data[1])),
                clamp(func(self.data[2])),
                self.alpha(),
            ],
        }
    }

    pub fn max(&self) -> f32 {
        self.data[0..=2].iter().cloned().fold(0.0, f32::max)
    }

    pub fn min(&self) -> f32 {
        self.data[0..=2].iter().cloned().fold(1.0, f32::min)
    }

    pub fn modulate(&self, hue: f32, saturation: f32, lightness: f32) -> Rgb {
        let hsl = self.to_hsl();

        hsl.with_hue(clamp(hsl.hue() * hue))
            .with_saturation(clamp(hsl.saturation() * saturation))
            .with_lightness(clamp(hsl.lightness() * lightness))
            .to_rgb()
    }

    pub const NONE: Rgb = Rgb {
        data: [0.0, 0.0, 0.0, 0.0],
    };
    pub const WHITE: Rgb = Rgb {
        data: [1.0, 1.0, 1.0, 1.0],
    };
    pub const BLACK: Rgb = Rgb {
        data: [0.0, 0.0, 0.0, 1.0],
    };
    pub const RED: Rgb = Rgb {
        data: [1.0, 0.0, 0.0, 1.0],
    };
    pub const GREEN: Rgb = Rgb {
        data: [0.0, 1.0, 0.0, 1.0],
    };
    pub const BLUE: Rgb = Rgb {
        data: [0.0, 0.0, 1.0, 1.0],
    };
    pub const CYAN: Rgb = Rgb {
        data: [0.0, 1.0, 1.0, 1.0],
    };
    pub const MAGENTA: Rgb = Rgb {
        data: [1.0, 0.0, 1.0, 1.0],
    };
    pub const YELLOW: Rgb = Rgb {
        data: [1.0, 1.0, 0.0, 1.0],
    };
}

impl Default for Rgb {
    fn default() -> Rgb {
        Rgb::NONE
    }
}

impl Color for Rgb {
    fn to_rgb(&self) -> Rgb {
        *self
    }

    fn alpha(&self) -> f32 {
        self.data[3]
    }

    fn set_alpha(&mut self, alpha: f32) {
        self.data[3] = alpha;
    }

    fn with_alpha(&self, alpha: f32) -> Rgb {
        Rgb {
            data: [self.data[0], self.data[1], self.data[2], alpha],
        }
    }

    fn data(&self) -> &[f32] {
        &self.data
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
        "rgb"
    }
}

impl PartialEq for Rgb {
    fn eq(&self, other: &Rgb) -> bool {
        for (v1, v2) in self.data.iter().zip(other.data.iter()) {
            if (v1 - v2).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_color(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::Rgb;
    use crate::color::{Color, Intensity};
    use std::str::FromStr;

    #[test]
    fn test_is_grayscale() {
        assert!(Rgb::new(0.5, 0.5, 0.5).is_grayscale());
        assert!(!Rgb::new(1.0, 0.5, 0.5).is_grayscale());
    }

    #[test]
    fn test_with_gamma() {
        assert_eq!(
            Rgb::new(0.5, 0.0, 0.0).with_gamma(1.5),
            Rgb::new(0.629_960_54, 0.0, 0.0)
        );
        assert_eq!(
            Rgb::new(0.5, 0.0, 0.0).with_gamma(0.5),
            Rgb::new(0.25, 0.0, 0.0)
        );
        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_gamma(1.5),
            Rgb::new(1.0, 0.629_953_44, 0.0)
        );
        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_gamma(0.5),
            Rgb::new(1.0, 0.25, 0.0)
        );
        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_gamma(1.5),
            Rgb::new(1.0, 0.629_960_54, 0.0)
        );
        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_gamma(1.0),
            Rgb::new(1.0, 0.5, 0.0)
        );
    }

    #[test]
    fn test_with_level() {
        assert_eq!(
            Rgb::new(0.5, 0.0, 0.0).with_level(0.0, 1.0, 1.5),
            Rgb::new(0.629_960_54, 0.0, 0.0)
        );
        assert_eq!(
            Rgb::new(0.5, 0.0, 0.0).with_level(0.0, 1.0, 0.5),
            Rgb::new(0.25, 0.0, 0.0)
        );
        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_level(0.0, 1.0, 1.5),
            Rgb::new(1.0, 0.629_953_44, 0.0)
        );
        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_level(0.0, 1.0, 0.5),
            Rgb::new(1.0, 0.25, 0.0)
        );
        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_level(0.0, 1.0, 1.5),
            Rgb::new(1.0, 0.629_960_54, 0.0)
        );
        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_level(0.0, 1.0, 1.0),
            Rgb::new(1.0, 0.5, 0.0)
        );

        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_level(-0.5, 1.5, 1.0),
            Rgb::new(0.749_996_2, 0.499_992_37, 0.250_003_8)
        );
        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_level(-0.5, 1.1, 1.0),
            Rgb::new(0.937_499_05, 0.624_994_3, 0.312_504_77)
        );
        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_level(-0.1, 1.5, 1.0),
            Rgb::new(0.687_495_23, 0.374_990_46, 0.062_500_95)
        );

        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_level(-0.5, 1.5, 1.2),
            Rgb::new(0.786_831_44, 0.561_226_84, 0.314_976_72)
        );
        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_level(-0.5, 1.1, 1.2),
            Rgb::new(0.947_631_06, 0.675_928_9, 0.379_354_54)
        );
        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_level(-0.1, 1.5, 1.2),
            Rgb::new(0.731_807_4, 0.441_596_1, 0.099_214_16)
        );

        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_level(-0.5, 1.5, 0.8),
            Rgb::new(0.697_947_7, 0.420_447_08, 0.176_775_77)
        );
        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_level(-0.5, 1.1, 0.8),
            Rgb::new(0.922_499_4, 0.555_703_04, 0.233_646_14)
        );
        assert_eq!(
            Rgb::new(1.0, 0.5, 0.0).with_level(-0.1, 1.5, 0.8),
            Rgb::new(0.626_016_6, 0.293_446_24, 0.031_250_477)
        );

        assert_eq!(
            Rgb::new(1.0, 1.0, 1.0).with_level(0.05, 1.05, 1.0),
            Rgb::new(0.95, 0.95, 0.95)
        );

        assert_eq!(
            Rgb::new(0.1, 0.2, 0.3).with_level(0.05, 1.05, 1.0),
            Rgb::new(0.05, 0.15, 0.25)
        );
    }

    #[test]
    fn type_name() {
        assert_eq!(Rgb::default().type_name(), "rgb");
    }

    #[test]
    fn to_string() {
        assert_eq!(
            Rgb::new_with_alpha(0.0, 0.0, 0.0, 1.0).to_string(),
            "rgb(0.0, 0.0, 0.0)"
        );
        assert_eq!(
            Rgb::new_with_alpha(1.0, 1.0, 1.0, 1.0).to_string(),
            "rgb(1.0, 1.0, 1.0)"
        );
        assert_eq!(
            Rgb::new_with_alpha(0.0, 0.0, 0.0, 0.0).to_string(),
            "rgba(0.0, 0.0, 0.0, 0.0)"
        );
        assert_eq!(
            Rgb::new_with_alpha(0.3, 0.6, 0.9, 0.5).to_string(),
            "rgba(0.3, 0.6, 0.9, 0.5)"
        );
        assert_eq!(
            Rgb::new_with_alpha(0.33, 0.666, 0.999, 0.5555).to_string(),
            "rgba(0.33, 0.666, 0.999, 0.5555)"
        );
    }

    #[test]
    fn parse_intensity() {
        let res = Intensity::from_str("default").unwrap();
        assert_eq!(res, Intensity::Rec709Luma);

        let res = Intensity::from_str("bad value");

        assert!(res.is_err());
        let res = res.unwrap_err();
        assert_eq!(res.input, "bad value");
        assert_eq!(res.enum_type, "Intensity");
    }

    #[test]
    fn test_setters() {
        let mut color = Rgb::new_with_alpha(0.1, 0.3, 0.5, 0.7);
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
