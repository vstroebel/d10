use d10_core::color::illuminant::D65;
use d10_core::color::observer::O2;
use d10_core::color::{Color, Hsl, Hsv, Lch, Rgb};
use d10_core::errors::ParseEnumError;
use d10_core::pixelbuffer::PixelBuffer;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SaturationMode {
    Hsl,
    Hsv,
    Lch,
}

impl FromStr for SaturationMode {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use SaturationMode::*;
        match value {
            "hsl" | "default" => Ok(Hsl),
            "hsv" => Ok(Hsv),
            "lch" => Ok(Lch),
            _ => Err(ParseEnumError::new(value, "SaturationMode")),
        }
    }
}

pub fn optimize_saturation(
    buffer: &PixelBuffer<Rgb>,
    offset: f32,
    mode: SaturationMode,
) -> PixelBuffer<Rgb> {
    let avg_sat = avg_saturation(buffer, mode);

    let gamma = offset + (1.0 - avg_sat) / 1.5;

    buffer.map_colors(|c| match mode {
        SaturationMode::Hsl => saturate_hsl(c, gamma),
        SaturationMode::Hsv => saturate_hsv(c, gamma),
        SaturationMode::Lch => saturate_lch(c, gamma),
    })
}

fn get_gamma_pow(gamma: f32, brightness: f32) -> f32 {
    // Prevent dark and bright colors to get too much saturation applied
    let factor = 1.0 - ((brightness - 0.5).abs() * 2.0);

    let gamma = if gamma < 0.0 {
        let diff = 1.0 - gamma;
        1.0 - diff * factor
    } else {
        let diff = gamma - 1.0;
        1.0 + diff * factor
    };

    1.0 / gamma
}

fn avg_saturation(buffer: &PixelBuffer<Rgb>, mode: SaturationMode) -> f32 {
    let mut sum = 0.0;

    for c in buffer.data() {
        sum += match mode {
            SaturationMode::Hsl => c.to_hsl().saturation(),
            SaturationMode::Hsv => c.to_hsv().saturation(),
            SaturationMode::Lch => c.to_lch::<D65, O2>().c(),
        };
    }

    sum / buffer.data().len() as f32
}

fn saturate_hsl(c: &Rgb, gamma: f32) -> Rgb {
    let hsl = c.to_hsl();
    let pow = get_gamma_pow(gamma, hsl.lightness());

    Hsl {
        data: [
            hsl.hue(),
            hsl.saturation().clamp(0.0, 1.0).powf(pow),
            hsl.lightness(),
            c.alpha(),
        ],
    }
    .to_rgb()
}

fn saturate_hsv(c: &Rgb, gamma: f32) -> Rgb {
    let hsv = c.to_hsv();
    let pow = get_gamma_pow(gamma, hsv.value());

    Hsv {
        data: [
            hsv.hue(),
            hsv.saturation().clamp(0.0, 1.0).powf(pow),
            hsv.value(),
            c.alpha(),
        ],
    }
    .to_rgb()
}

fn saturate_lch(c: &Rgb, gamma: f32) -> Rgb {
    let lch = c.to_lch::<D65, O2>();
    let pow = get_gamma_pow(gamma, lch.l());

    Lch::<D65, O2>::new_with_alpha(
        lch.l(),
        lch.c().clamp(0.0, 1.0).powf(pow),
        lch.h(),
        c.alpha(),
    )
    .to_rgb()
}
