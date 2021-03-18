mod rgb;
mod srgb;
mod hsl;
mod hsv;
mod yuv;
mod iter;

pub use rgb::{RGB, Intensity};
pub use srgb::{SRGB, gamma_to_linear, linear_to_gamma};
pub use hsv::HSV;
pub use hsl::HSL;
pub use yuv::YUV;
pub use iter::{ColorIter, ColorIterRef};

use std::fmt::{Debug, Display};

// Minimal error to detect identical colors channel values
//
// The precision is based on `2.pow(15)` because we want to be somewhere near 16 Bit per channel
// and avoid problems with rounding errors when dealing with 16 bit images.
pub(crate) const EPSILON: f32 = 1.0 / 32768.0;

pub(crate) fn clamp(value: f32) -> f32 {
    value.min(1.0).max(0.0)
}

/// A trait that must be implemented by all color types
///
/// As of now this type is sealed to prevent incompatibilities with future changes.
/// This restriction might be removed when the crate is heading towards 1.0.
pub trait Color: Copy + Clone + Default + PartialEq + Send + Sync + Debug + Display + private::Sealed {
    fn to_rgb(&self) -> RGB;

    fn alpha(&self) -> f32;

    fn data(&self) -> &[f32];

    fn to_srgb(&self) -> SRGB {
        let rgb = self.to_rgb();

        SRGB::new_with_alpha(
            linear_to_gamma(rgb.data[0]),
            linear_to_gamma(rgb.data[1]),
            linear_to_gamma(rgb.data[2]),
            rgb.data[3],
        )
    }

    fn to_hsl(&self) -> HSL {
        let rgb = self.to_rgb();

        let max = rgb.max();
        let min = rgb.min();

        let red = rgb.red();
        let green = rgb.green();
        let blue = rgb.blue();

        let mut hue;
        let saturation;
        let lightness = (max + min) / 2.0;

        let delta = max - min;

        if delta < EPSILON {
            // achromatic
            hue = 0.0;
            saturation = 0.0;
        } else {
            let d = max - min;
            saturation = if lightness > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };

            if (max - red).abs() < EPSILON {
                hue = (green - blue) / d + (if green < blue { 6.0 } else { 0.0 });
            } else if (max - green).abs() < EPSILON {
                hue = (blue - red) / d + 2.0;
            } else {
                hue = (red - green) / d + 4.0;
            }

            hue /= 6.0;
        }

        HSL {
            data: [hue, saturation, lightness, rgb.alpha()]
        }
    }

    fn to_hsv(&self) -> HSV {
        let rgb = self.to_rgb();

        let max = rgb.max();
        let min = rgb.min();

        let red = rgb.red();
        let green = rgb.green();
        let blue = rgb.blue();

        let mut hue;
        let saturation;
        let value = max;

        let delta = max - min;
        if delta < EPSILON {
            saturation = 0.0;
            hue = 0.0;
        } else if max > 0.0 {
            saturation = delta / max;

            if red >= max {
                hue = (green - blue) / delta;
            } else if green >= max {
                hue = 2.0 + (blue - red) / delta;
            } else {
                hue = 4.0 + (red - green) / delta;
            }

            hue *= 60.0;

            if hue < 0.0 {
                hue += 360.0;
            }
        } else {
            // if max is 0, then r = g = b = 0
            // s = 0, h is undefined
            saturation = 0.0;
            hue = 0.0;
        }


        HSV {
            data: [hue / 360.0, saturation, value, rgb.alpha()]
        }
    }

    fn to_yuv(&self) -> YUV {
        let rgb = self.to_srgb();

        let y = 0.299 * rgb.red() + 0.587 * rgb.green() + 0.114 * rgb.blue();
        let u = -0.147_141_19 * rgb.red() + -0.288_869_17 * rgb.green() + 0.436_010_36 * rgb.blue();
        let v = 0.614_975_4 * rgb.red() + -0.514_965_1 * rgb.green() + -0.100_010_26 * rgb.blue();

        YUV {
            data: [y, u, v, rgb.alpha()]
        }
    }

    fn has_transparency(&self) -> bool {
        (1.0 - self.alpha()).abs() > EPSILON
    }

    /// Map all color channels and return a new color with the same alpha value
    fn map_color_channels<F: FnMut(f32) -> f32>(&self, func: F) -> Self;

    /// Return a lowercase name of this colors type (i.e. "rgb" for RGB)
    fn type_name(&self) -> &'static str;
}

// A generic implementation to format a color as a CSS alike string used to implement the Display trait
//
// TODO: Improve performance by directly writing parts to the formatter
pub(crate) fn format_color<C: Color>(color: &C, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let with_alpha = color.has_transparency();

    let mut result = String::with_capacity(28);

    result.push_str(color.type_name());
    if with_alpha {
        result.push('a');
    }

    result.push('(');

    fn format_f32(v: f32) -> String {
        let v = (v * 10_000f32).round() / 10_000f32;

        let mut v = v.to_string();
        if !v.contains('.') {
            v.push_str(".0");
        }

        v
    }

    let data = color.data();

    for v in &data[..data.len() - 1] {
        result.push_str(&format_f32(*v));
        result.push_str(", ");
    }

    if with_alpha {
        result.push_str(&format_f32(color.alpha()));
    } else {
        result.pop();
        result.pop();
    }

    result.push(')');

    f.write_str(&result)?;

    Ok(())
}

mod private {
    use crate::color::Color;

    pub trait Sealed {}

    impl<T: Color> Sealed for T {}
}
