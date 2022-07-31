use std::str::FromStr;

use d10_core::color::illuminant::D65;
use d10_core::color::observer::O2;
use d10_core::color::{Color, Rgb};
use d10_core::errors::ParseEnumError;
use d10_core::pixelbuffer::PixelBuffer;

fn apply_intensity(v1: f32, v2: f32, intensity: f32) -> f32 {
    v1 * (1.0 - intensity) + v2 * intensity
}

fn blend_color<F>(c1: Rgb, c2: Rgb, intensity: f32, func: F) -> Rgb
where
    F: Fn(f32, f32) -> f32,
{
    let intensity = intensity * c2.alpha();

    let blend_values = |v1: f32, v2: f32| -> f32 { apply_intensity(v1, func(v1, v2), intensity) };

    Rgb::new_with_alpha(
        blend_values(c1.data[0], c2.data[0]),
        blend_values(c1.data[1], c2.data[1]),
        blend_values(c1.data[2], c2.data[2]),
        c2.alpha(),
    )
}

fn blend_image_with_func<F>(
    img1: &PixelBuffer<Rgb>,
    img2: &PixelBuffer<Rgb>,
    intensity: f32,
    func: F,
) -> PixelBuffer<Rgb>
where
    F: Fn(Rgb, Rgb, f32) -> Rgb,
{
    let width = img1.width().max(img2.width());
    let height = img1.height().max(img2.height());

    PixelBuffer::new_from_func(width, height, |x, y| {
        let c1 = img1.get_pixel_optional(x as i32, y as i32);
        let c2 = img2.get_pixel_optional(x as i32, y as i32);

        match (c1, c2) {
            (Some(c1), Some(c2)) => func(*c1, *c2, intensity),
            (Some(c1), None) => *c1,
            (None, Some(c2)) => *c2,
            (None, None) => Rgb::NONE,
        }
    })
}

#[derive(Debug, PartialEq, Eq)]
pub enum BlendOp {
    Normal,
    Addition,
    Subtract,
    Darken,
    Lighten,
    HslDarken,
    HslLighten,
    LchDarken,
    LchLighten,
    LchHue,
    LchSaturation,
    LchColor,
}

impl FromStr for BlendOp {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use BlendOp::*;
        match value {
            "normal" => Ok(Normal),
            "addition" => Ok(Addition),
            "subtract" => Ok(Subtract),
            "darken" => Ok(Darken),
            "lighten" => Ok(Lighten),
            "hsl_darken" => Ok(HslDarken),
            "hsl_lighten" => Ok(HslLighten),
            "lch_darken" => Ok(LchDarken),
            "lch_lighten" => Ok(LchLighten),
            "lch_hue" => Ok(LchHue),
            "lch_saturation" => Ok(LchSaturation),
            "lch_color" => Ok(LchColor),
            _ => Err(ParseEnumError::new(value, "BlendOp")),
        }
    }
}

pub fn blend_image(
    img1: &PixelBuffer<Rgb>,
    img2: &PixelBuffer<Rgb>,
    blend_op: BlendOp,
    intensity: f32,
) -> PixelBuffer<Rgb> {
    match blend_op {
        BlendOp::Normal => blend_image_with_func(img1, img2, intensity, blend_normal),
        BlendOp::Addition => blend_image_with_func(img1, img2, intensity, blend_addition),
        BlendOp::Subtract => blend_image_with_func(img1, img2, intensity, blend_subtract),
        BlendOp::Darken => blend_image_with_func(img1, img2, intensity, blend_darken),
        BlendOp::Lighten => blend_image_with_func(img1, img2, intensity, blend_lighten),
        BlendOp::HslDarken => blend_image_with_func(img1, img2, intensity, blend_hsl_darken),
        BlendOp::HslLighten => blend_image_with_func(img1, img2, intensity, blend_hsl_lighten),
        BlendOp::LchDarken => blend_image_with_func(img1, img2, intensity, blend_lch_darken),
        BlendOp::LchLighten => blend_image_with_func(img1, img2, intensity, blend_lch_lighten),
        BlendOp::LchHue => blend_image_with_func(img1, img2, intensity, blend_lch_hue),
        BlendOp::LchSaturation => {
            blend_image_with_func(img1, img2, intensity, blend_lch_saturation)
        }
        BlendOp::LchColor => blend_image_with_func(img1, img2, intensity, blend_lch_color),
    }
}

pub fn blend_normal(c1: Rgb, c2: Rgb, intensity: f32) -> Rgb {
    c1.alpha_blend(c2.with_alpha(c2.alpha() * intensity))
}

pub fn blend_addition(c1: Rgb, c2: Rgb, intensity: f32) -> Rgb {
    blend_color(c1, c2, intensity, |v1, v2| v1 + v2)
}

pub fn blend_subtract(c1: Rgb, c2: Rgb, intensity: f32) -> Rgb {
    blend_color(c1, c2, intensity, |v1, v2| v1 - v2)
}

pub fn blend_darken(c1: Rgb, c2: Rgb, intensity: f32) -> Rgb {
    blend_color(c1, c2, intensity, |v1, v2| v1.min(v2))
}

pub fn blend_lighten(c1: Rgb, c2: Rgb, intensity: f32) -> Rgb {
    blend_color(c1, c2, intensity, |v1, v2| v1.max(v2))
}

pub fn blend_hsl_darken(c1: Rgb, c2: Rgb, intensity: f32) -> Rgb {
    let c1 = c1.to_hsl();
    let c2 = c2.to_hsl();

    let l1 = c1.lightness();
    let l2 = c1.lightness().min(c2.lightness());

    let l = apply_intensity(l1, l2, intensity);

    c1.with_lightness(l).to_rgb()
}

pub fn blend_hsl_lighten(c1: Rgb, c2: Rgb, intensity: f32) -> Rgb {
    let c1 = c1.to_hsl();
    let c2 = c2.to_hsl();

    let l1 = c1.lightness();
    let l2 = c1.lightness().max(c2.lightness());

    let l = apply_intensity(l1, l2, intensity);

    c1.with_lightness(l).to_rgb()
}

pub fn blend_lch_darken(c1: Rgb, c2: Rgb, intensity: f32) -> Rgb {
    let c1 = c1.to_lch::<D65, O2>();
    let c2 = c2.to_lch::<D65, O2>();

    let l1 = c1.l();
    let l2 = c1.l().min(c2.l());

    let l = apply_intensity(l1, l2, intensity);

    c1.with_l(l).to_rgb()
}

pub fn blend_lch_lighten(c1: Rgb, c2: Rgb, intensity: f32) -> Rgb {
    let c1 = c1.to_lch::<D65, O2>();
    let c2 = c2.to_lch::<D65, O2>();

    let l1 = c1.l();
    let l2 = c1.l().max(c2.l());

    let l = apply_intensity(l1, l2, intensity);

    c1.with_l(l).to_rgb()
}

pub fn blend_lch_hue(c1: Rgb, c2: Rgb, intensity: f32) -> Rgb {
    let c1 = c1.to_lch::<D65, O2>();
    let c2 = c2.to_lch::<D65, O2>();

    c1.with_h(apply_intensity(c1.h(), c2.h(), intensity))
        .to_rgb()
}

pub fn blend_lch_saturation(c1: Rgb, c2: Rgb, intensity: f32) -> Rgb {
    let c1 = c1.to_lch::<D65, O2>();
    let c2 = c2.to_lch::<D65, O2>();

    c1.with_c(apply_intensity(c1.c(), c2.c(), intensity))
        .to_rgb()
}

pub fn blend_lch_color(c1: Rgb, c2: Rgb, intensity: f32) -> Rgb {
    let c1 = c1.to_lch::<D65, O2>();
    let c2 = c2.to_lch::<D65, O2>();

    c1.with_c(apply_intensity(c1.c(), c2.c(), intensity))
        .with_h(apply_intensity(c1.h(), c2.h(), intensity))
        .to_rgb()
}
