mod rgb;
mod srgb;
mod hsl;
mod hsv;
mod yuv;
mod iter;

pub use rgb::{Rgb, Intensity};
pub use srgb::{Srgb, gamma_to_linear, linear_to_gamma};
pub use hsv::Hsv;
pub use hsl::Hsl;
pub use yuv::Yuv;
pub use iter::{ColorIter, ColorIterRef};

use std::fmt::{Debug, Display};

/// Minimal error to detect identical colors channel values
///
/// The precision is based on `2.pow(15)` because we want to be somewhere near 16 Bit per channel
/// and avoid problems with rounding errors when dealing with 16 bit images.
pub(crate) const EPSILON: f32 = 1.0 / 32768.0;

pub(crate) fn clamp(value: f32) -> f32 {
    value.min(1.0).max(0.0)
}

/// A trait that must be implemented by all color types
///
/// As of now this type is sealed to prevent incompatibilities with future changes.
/// This restriction might be removed when the crate is heading towards 1.0.
pub trait Color: Copy + Clone + Default + PartialEq + Send + Sync + Debug + Display + private::Sealed {
    fn to_rgb(&self) -> Rgb;

    fn alpha(&self) -> f32;

    fn with_alpha(&self, alpha: f32) -> Self;

    fn data(&self) -> &[f32];

    fn to_srgb(&self) -> Srgb {
        let rgb = self.to_rgb();

        Srgb::new_with_alpha(
            linear_to_gamma(rgb.data[0]),
            linear_to_gamma(rgb.data[1]),
            linear_to_gamma(rgb.data[2]),
            rgb.data[3],
        )
    }

    fn to_hsl(&self) -> Hsl {
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

        Hsl {
            data: [hue, saturation, lightness, rgb.alpha()]
        }
    }

    fn to_hsv(&self) -> Hsv {
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


        Hsv {
            data: [hue / 360.0, saturation, value, rgb.alpha()]
        }
    }

    fn to_yuv(&self) -> Yuv {
        Yuv {
            data: apply_matrix(&self.to_srgb().data, &yuv::RGB_TO_YUV)
        }
    }

    fn has_transparency(&self) -> bool {
        (1.0 - self.alpha()).abs() > EPSILON
    }

    /// Map all color channels and return a new color with the same alpha value
    fn map_color_channels<F: FnMut(f32) -> f32>(&self, mut func: F) -> Self {
        self.try_map_color_channels::<(), _>(|f| Ok(func(f))).unwrap()
    }

    /// Map all color channels and return a new color with the same alpha value
    fn try_map_color_channels<E, F: FnMut(f32) -> Result<f32, E>>(&self, func: F) -> Result<Self, E>;

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

/// Apply a 3x3 matrix to the color channels
///
/// This is a helper that is used to convert between colors if possible with a simple matrix.
/// The alpha channel is not affected by the conversion
pub(crate) fn apply_matrix(color: &[f32; 4], matrix: &[[f32; 3]; 3]) -> [f32; 4] {
    [
        color[0] * matrix[0][0] + color[1] * matrix[0][1] + color[2] * matrix[0][2],
        color[0] * matrix[1][0] + color[1] * matrix[1][1] + color[2] * matrix[1][2],
        color[0] * matrix[2][0] + color[1] * matrix[2][1] + color[2] * matrix[2][2],
        color[3]
    ]
}

mod private {
    use crate::color::Color;

    // Using a private to make it impossible for other crates to implement their own color type
    pub trait Sealed {}

    impl<T: Color> Sealed for T {}
}
