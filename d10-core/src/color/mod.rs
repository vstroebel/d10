mod hsl;
mod hsv;
mod iter;
mod lab;
mod rgb;
mod srgb;
mod xyz;
mod yuv;

pub use hsl::Hsl;
pub use hsv::Hsv;
pub use iter::{ColorIter, ColorIterRef};
pub use lab::{illuminant, observer, DefaultLab, Illuminant, Lab, Lch, Observer};
pub use rgb::{Intensity, Rgb};
pub use srgb::{gamma_to_linear, linear_to_gamma, Srgb};
pub use xyz::Xyz;
pub use yuv::Yuv;

use crate::color::lab::get_refs;
use std::fmt::{Debug, Display};

/// Minimal error to detect identical colors channel values
///
/// The precision is based on `2.pow(15)` because we want to be somewhere near 16 Bit per channel
/// and avoid problems with rounding errors when dealing with 16 bit images.
pub(crate) const EPSILON: f32 = 1.0 / 32768.0;

pub(crate) fn clamp(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

/// A trait that must be implemented by all color types
///
/// As of now this type is sealed to prevent incompatibilities with future changes.
/// This restriction might be removed when the crate is heading towards 1.0.
pub trait Color:
    Copy + Clone + Default + PartialEq + Send + Sync + Debug + Display + private::Sealed
{
    fn to_rgb(&self) -> Rgb;

    fn alpha(&self) -> f32;

    fn set_alpha(&mut self, alpha: f32);

    fn with_alpha(&self, alpha: f32) -> Self;

    fn data(&self) -> &[f32];

    fn data_mut(&mut self) -> &mut [f32];

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
            saturation = if lightness > 0.5 {
                d / (2.0 - max - min)
            } else {
                d / (max + min)
            };

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
            data: [hue, saturation, lightness, rgb.alpha()],
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

        if delta >= EPSILON || max >= EPSILON {
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
            saturation = 0.0;
            hue = 0.0;
        }

        Hsv {
            data: [hue / 360.0, saturation, value, rgb.alpha()],
        }
    }

    fn to_yuv(&self) -> Yuv {
        Yuv {
            data: apply_matrix(&self.to_srgb().data, &yuv::RGB_TO_YUV),
        }
    }

    fn to_xyz(&self) -> Xyz {
        Xyz {
            data: apply_matrix(&self.to_rgb().data, &xyz::RGB_TO_XYZ),
        }
    }

    fn to_lab<I: Illuminant, O: Observer>(&self) -> Lab<I, O> {
        fn func(v: f32) -> f32 {
            if v < 0.008856 {
                (903.3 * v + 16.0) / 116.0
            } else {
                v.powf(1.0 / 3.0)
            }
        }

        let xyz = self.to_xyz();

        let refs = get_refs::<I, O>();

        let rx = xyz.x() / refs[0];
        let ry = xyz.y() / refs[1];
        let rz = xyz.z() / refs[2];

        let l = 116.0 * func(ry) - 16.0;
        let a = 500.0 * (func(rx) - func(ry));
        let b = 200.0 * (func(ry) - func(rz));

        let l = l / 100.0;
        let a = a / 128.0;
        let b = b / 128.0;

        Lab::new_with_alpha(l, a, b, xyz.alpha())
    }

    fn to_lch<I: Illuminant, O: Observer>(&self) -> Lch<I, O> {
        let lab = self.to_lab::<I, O>();

        let a = lab.a();
        let b = lab.b();

        let c = (a * a + b * b).sqrt();
        let h = (b).atan2(a);

        Lch::new_with_alpha(lab.l(), c, h, lab.alpha())
    }

    fn has_transparency(&self) -> bool {
        (1.0 - self.alpha()).abs() > EPSILON
    }

    /// Map all color channels and return a new color with the same alpha value
    fn map_color_channels<F: FnMut(f32) -> f32>(&self, mut func: F) -> Self {
        self.try_map_color_channels::<(), _>(|f| Ok(func(f)))
            .unwrap()
    }

    /// Map all color channels and return a new color with the same alpha value
    fn try_map_color_channels<E, F: FnMut(f32) -> Result<f32, E>>(
        &self,
        func: F,
    ) -> Result<Self, E>;

    /// Return a lowercase name of this colors type (i.e. "rgb" for RGB)
    fn type_name(&self) -> &'static str;
}

macro_rules! color_from {
    ($from:ident, $to:ident, $to_func:ident) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                value.$to_func()
            }
        }
    };
}

color_from!(Rgb, Srgb, to_srgb);
color_from!(Hsl, Srgb, to_srgb);
color_from!(Hsv, Srgb, to_srgb);
color_from!(Yuv, Srgb, to_srgb);
color_from!(Xyz, Srgb, to_srgb);
color_from!(Lab, Srgb, to_srgb);
color_from!(Lch, Srgb, to_srgb);

color_from!(Srgb, Rgb, to_rgb);
color_from!(Hsl, Rgb, to_rgb);
color_from!(Hsv, Rgb, to_rgb);
color_from!(Yuv, Rgb, to_rgb);
color_from!(Xyz, Rgb, to_rgb);
color_from!(Lab, Rgb, to_rgb);
color_from!(Lch, Rgb, to_rgb);

color_from!(Rgb, Hsl, to_hsl);
color_from!(Srgb, Hsl, to_hsl);
color_from!(Hsv, Hsl, to_hsl);
color_from!(Yuv, Hsl, to_hsl);
color_from!(Xyz, Hsl, to_hsl);
color_from!(Lab, Hsl, to_hsl);
color_from!(Lch, Hsl, to_hsl);

color_from!(Rgb, Hsv, to_hsv);
color_from!(Srgb, Hsv, to_hsv);
color_from!(Hsl, Hsv, to_hsv);
color_from!(Yuv, Hsv, to_hsv);
color_from!(Xyz, Hsv, to_hsv);
color_from!(Lab, Hsv, to_hsv);
color_from!(Lch, Hsv, to_hsv);

color_from!(Rgb, Yuv, to_yuv);
color_from!(Srgb, Yuv, to_yuv);
color_from!(Hsl, Yuv, to_yuv);
color_from!(Hsv, Yuv, to_yuv);
color_from!(Xyz, Yuv, to_yuv);
color_from!(Lab, Yuv, to_yuv);
color_from!(Lch, Yuv, to_yuv);

color_from!(Rgb, Xyz, to_xyz);
color_from!(Srgb, Xyz, to_xyz);
color_from!(Hsl, Xyz, to_xyz);
color_from!(Hsv, Xyz, to_xyz);
color_from!(Yuv, Xyz, to_xyz);
color_from!(Lab, Xyz, to_xyz);
color_from!(Lch, Xyz, to_xyz);

color_from!(Rgb, Lab, to_lab);
color_from!(Srgb, Lab, to_lab);
color_from!(Hsl, Lab, to_lab);
color_from!(Hsv, Lab, to_lab);
color_from!(Yuv, Lab, to_lab);
color_from!(Xyz, Lab, to_lab);
color_from!(Lch, Lab, to_lab);

color_from!(Rgb, Lch, to_lch);
color_from!(Srgb, Lch, to_lch);
color_from!(Hsl, Lch, to_lch);
color_from!(Hsv, Lch, to_lch);
color_from!(Yuv, Lch, to_lch);
color_from!(Xyz, Lch, to_lch);
color_from!(Lab, Lch, to_lch);


// A generic implementation to format a color as a CSS alike string used to implement the Display trait
//
// TODO: Improve performance by directly writing parts to the formatter
pub(crate) fn format_color<C: Color>(
    color: &C,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
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
        color[3],
    ]
}

pub(crate) fn apply_matrix_clamped(color: &[f32; 4], matrix: &[[f32; 3]; 3]) -> [f32; 4] {
    [
        clamp(color[0] * matrix[0][0] + color[1] * matrix[0][1] + color[2] * matrix[0][2]),
        clamp(color[0] * matrix[1][0] + color[1] * matrix[1][1] + color[2] * matrix[1][2]),
        clamp(color[0] * matrix[2][0] + color[1] * matrix[2][1] + color[2] * matrix[2][2]),
        color[3],
    ]
}

mod private {
    use crate::color::Color;

    // Using a private to make it impossible for other crates to implement their own color type
    pub trait Sealed {}

    impl<T: Color> Sealed for T {}
}

#[cfg(test)]
mod conversion_tests {
    /*
     * Tests that are checking if conversion from rgb to other colorspaces and back is working.
     */

    use crate::color::illuminant::D65;
    use crate::color::observer::O2;
    use crate::color::{Color, ColorIter, ColorIterRef, Rgb};
    use rand::{thread_rng, Rng};

    const RGB: [(f32, f32, f32); 15] = [
        (0.0, 0.0, 0.0),
        (1.0, 1.0, 1.0),
        (0.5, 0.5, 0.5),
        (1.0, 0.0, 0.0),
        (0.0, 1.0, 0.0),
        (0.0, 0.0, 1.0),
        (1.0, 0.5, 0.5),
        (0.5, 1.0, 0.5),
        (0.5, 0.5, 1.0),
        (0.5, 0.0, 0.0),
        (0.0, 0.5, 0.0),
        (0.0, 0.0, 0.5),
        (0.5, 0.0, 0.5),
        (0.5, 0.5, 0.0),
        (0.0, 0.5, 0.5),
    ];

    fn get_rgb() -> Vec<Rgb> {
        let mut rgb: Vec<Rgb> = RGB
            .iter()
            .map(|rgb| Rgb::new(rgb.0, rgb.1, rgb.2))
            .collect();

        let mut rng = thread_rng();

        for _ in 0..100 {
            let r = rng.gen_range(0.0..1.0);
            let g = rng.gen_range(0.0..1.0);
            let b = rng.gen_range(0.0..1.0);
            let a = rng.gen_range(0.0..1.0);

            rgb.push(Rgb::new_with_alpha(r, g, b, a));
        }

        rgb
    }

    #[test]
    fn test_rgb_srgb() {
        for rgb in get_rgb() {
            assert_eq!(rgb, rgb.to_srgb().to_rgb())
        }
    }

    #[test]
    fn test_rgb_hsv() {
        for rgb in get_rgb() {
            assert_eq!(rgb, rgb.to_hsv().to_rgb())
        }
    }

    #[test]
    fn test_rgb_hsl() {
        for rgb in get_rgb() {
            assert_eq!(rgb, rgb.to_hsl().to_rgb())
        }
    }

    #[test]
    fn test_rgb_yuv() {
        for rgb in get_rgb() {
            assert_eq!(rgb, rgb.to_yuv().to_rgb())
        }
    }

    #[test]
    fn test_rgb_xyz() {
        for rgb in get_rgb() {
            assert_eq!(rgb, rgb.to_xyz().to_rgb())
        }
    }

    #[test]
    fn test_rgb_lab() {
        for rgb in get_rgb() {
            assert_eq!(rgb, rgb.to_lab::<D65, O2>().to_rgb())
        }
    }

    #[test]
    fn test_rgb_srgb_iter() {
        let rgb = get_rgb();
        let res: Vec<_> = rgb.iter().into_srgb().into_rgb().collect();
        assert_eq!(rgb, res);
    }

    #[test]
    fn test_rgb_hsv_iter() {
        let rgb = get_rgb();
        let res: Vec<_> = rgb.iter().into_hsv().into_rgb().collect();
        assert_eq!(rgb, res);
    }

    #[test]
    fn test_rgb_hsl_iter() {
        let rgb = get_rgb();
        let res: Vec<_> = rgb.iter().into_hsl().into_rgb().collect();
        assert_eq!(rgb, res);
    }

    #[test]
    fn test_rgb_yuv_iter() {
        let rgb = get_rgb();
        let res: Vec<_> = rgb.iter().into_yuv().into_rgb().collect();
        assert_eq!(rgb, res);
    }

    #[test]
    fn test_rgb_xyz_iter() {
        let rgb = get_rgb();
        let res: Vec<_> = rgb.iter().into_xyz().into_rgb().collect();
        assert_eq!(rgb, res);
    }

    #[test]
    fn test_rgb_lab_iter() {
        let rgb = get_rgb();
        let res: Vec<_> = rgb.iter().into_lab::<D65, O2>().into_rgb().collect();
        assert_eq!(rgb, res);
    }

    #[test]
    fn test_rgb_lch_iter() {
        let rgb = get_rgb();
        let res: Vec<_> = rgb.iter().into_lch::<D65, O2>().into_rgb().collect();
        assert_eq!(rgb, res);
    }
}
