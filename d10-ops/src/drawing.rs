use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::{Rgb, Intensity, Color};
use crate::{compose, gaussian_blur, unsharp, despeckle};

use std::str::FromStr;
use d10_core::errors::ParseEnumError;

#[derive(Debug, Copy, Clone)]
pub enum DrawingMode {
    Gray,
    Colored,
    ReducedColors,
}

impl FromStr for DrawingMode {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<DrawingMode, Self::Err> {
        match value {
            "gray" => Ok(DrawingMode::Gray),
            "colored" | "default" => Ok(DrawingMode::Colored),
            "reduced_colors" => Ok(DrawingMode::ReducedColors),
            _ => Err(ParseEnumError::new(value, "DrawingMode")),
        }
    }
}

pub fn drawing(buffer: &PixelBuffer<Rgb>, radius: u32, mode: DrawingMode) -> PixelBuffer<Rgb> {
    let orig = buffer;

    let lightened = orig.map_colors(|c| {
        c.map_channels(|v| {
            v * 0.8 + 0.2
        })
    });

    let b1 = {
        let b = gaussian_blur(&lightened, 1, None);
        let b = unsharp(&b, 4, 4.0, Some(5.0));

        b.map_colors(|c| c.with_contrast(1.05))
    };

    let b2 = {
        let b = gaussian_blur(&lightened, radius, None);
        b.map_colors(|c| c.invert())
    };

    let drawing = compose([&b1, &b2], Rgb::NONE, |_, _, [c1, c2]| {
        let mut v1 = vec![];
        let mut v2 = vec![];

        let g1 = c1.to_gray_with_intensity(Intensity::Average).red();
        let g2 = c2.to_gray_with_intensity(Intensity::Average).red();

        if g1 > 0.05 || g2 < 0.95 {
            v1.push(g1);
            v2.push(g2);
        }

        for i in 0..3 {
            if c1.data[i] > 0.8 || c2.data[i] < 0.2 {
                v1.push(c1.data[i]);
                v2.push(c2.data[i]);
            }
        }

        let c = |v1: f32, v2: f32| {
            let v1 = v1; // + 1.0;
            let v2 = v2; // + 1.0;

            (v1 / (1.0 - v2 + 0.01).min(1.0)).abs()
        };

        let diff = v1.iter()
            .zip(v2.iter())
            .map(|(v1, v2)| c(*v1, *v2))
            .fold(1.0f32, |a, b| a.min(b));

        let diff = if diff > 0.8 {
            1.0
        } else {
            ((diff - 0.5) * 1.2).max(0.0)
        };

        Rgb::new(diff, diff, diff)
    });

    let drawing = unsharp(&drawing, 2, 1.0, None);

    let drawing = despeckle(&drawing, 0.15, 1);

    match mode {
        DrawingMode::Gray => drawing,
        DrawingMode::Colored => merge_color(drawing, &orig),
        DrawingMode::ReducedColors => merge_color_reduced(drawing, &orig),
    }
}

fn merge_color(drawing: PixelBuffer<Rgb>, orig: &PixelBuffer<Rgb>) -> PixelBuffer<Rgb> {
    compose([&drawing, &orig], Rgb::NONE, |_, _, [d, o]| {
        let d = d.to_hsl();
        let o = o.to_hsl();

        o.with_lightness(o.lightness().min(d.lightness())).to_rgb()
    })
}

fn merge_color_reduced(drawing: PixelBuffer<Rgb>, orig: &PixelBuffer<Rgb>) -> PixelBuffer<Rgb> {
    let reduced_color = orig.map_colors(|c| {
        let c = c.to_hsv();

        let c = c.with_saturation((c.saturation() * 6.0) / 12.0);
        let c = c.with_value(((c.value() * 6.5 + 0.2) / 4.5).min(1.0));

        c.to_rgb()
    });

    let out1 = compose([&drawing, &gaussian_blur(&reduced_color, 2, None)], Rgb::NONE, |_, _, [c1, c2]| {
        c1.to_hsv()
            .with_saturation(c2.to_hsv().saturation())
            .to_rgb()
    });

    let out1 = compose([&out1, &gaussian_blur(&orig, 3, None)], Rgb::NONE, |_, _, [c1, c2]| {
        c1.to_hsv()
            .with_hue(c2.to_hsv().hue())
            .to_rgb()
    });

    let out1 = compose([&out1, &gaussian_blur(&orig, 4, None)], Rgb::NONE, |_, _, [c1, c2]| {
        let c = c1.to_hsv();

        c.with_value(c.value() * c2.to_hsv().value())
            .to_rgb()
    });

    unsharp(&out1, 3, 1.5, None)
        .map_colors(|x|
            x.with_saturation(1.3)
                .with_gamma(1.1)
                .with_vibrance(0.3)
        )
}
