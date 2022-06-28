use std::str::FromStr;

use d10_core::color::{Color, Rgb};
use d10_core::errors::ParseEnumError;
use d10_core::pixelbuffer::PixelBuffer;

fn blend_color<F>(c1: Rgb, c2: Rgb, intensity: f32, func: F) -> Rgb
where
    F: Fn(f32, f32) -> f32,
{
    let intensity = intensity * c2.alpha();

    let blend_values = |v1: f32, v2: f32| -> f32 {
        let v2 = func(v1, v2);
        v1 * (1.0 - intensity) + v2 * intensity
    };

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
