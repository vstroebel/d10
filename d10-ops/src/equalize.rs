use d10_core::color::{Color, Hsl, Rgb, Srgb};
use d10_core::errors::ParseEnumError;
use d10_core::pixelbuffer::PixelBuffer;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EqualizeMode {
    Rgb,
    Srgb,
    Saturation,
    Lightness,
    SaturationLightness,
}

impl FromStr for EqualizeMode {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use EqualizeMode::*;
        match value {
            "rgb" => Ok(Rgb),
            "srgb" => Ok(Srgb),
            "saturation" => Ok(Saturation),
            "lightness" => Ok(Lightness),
            "saturation_lightness" => Ok(SaturationLightness),
            _ => Err(ParseEnumError::new(value, "EqualizeMode")),
        }
    }
}

fn channel_histogram<C: Color, const NUM_CHANNELS: usize>(
    buffer: &PixelBuffer<C>,
    channel_offset: usize,
) -> [[f32; 256]; NUM_CHANNELS] {
    let mut histogram = [[0.0; 256]; NUM_CHANNELS];

    for (i, h) in histogram.iter_mut().enumerate() {
        for c in buffer.data() {
            let v = c.data()[channel_offset + i] * 255.0;
            let index = v.round().clamp(0., 255.0) as usize;
            h[index] += 1.0;
        }
    }

    let pixels = (buffer.width() * buffer.height()) as f32;

    let mut sum = [0.0; NUM_CHANNELS];

    for (i, h) in histogram.iter_mut().enumerate() {
        for v in h.iter_mut() {
            sum[i] += *v;
            *v = sum[i] / pixels;
        }
    }

    let mut max = [0.0f32; NUM_CHANNELS];
    let mut min = [f32::MAX; NUM_CHANNELS];

    for (i, h) in histogram.iter_mut().enumerate() {
        for &v in h.iter() {
            max[i] = max[i].max(v);
            min[i] = min[i].min(v);
        }
    }

    for (i, h) in histogram.iter_mut().enumerate() {
        for v in h.iter_mut() {
            *v = (*v - min[i]) / (max[i] - min[i]);
        }
    }

    histogram
}

fn pick_value(histogram: &[f32; 256], value: f32) -> f32 {
    let r = (value * 255.0).round() as u8;
    histogram[r as usize]
}

fn equalize_rgb(buffer: &PixelBuffer<Rgb>) -> PixelBuffer<Rgb> {
    let histogram = channel_histogram::<_, 3>(buffer, 0);

    buffer.map_colors(|c| {
        Rgb::new_with_alpha(
            pick_value(&histogram[0], c.red()),
            pick_value(&histogram[1], c.green()),
            pick_value(&histogram[2], c.blue()),
            c.alpha(),
        )
    })
}

fn equalize_srgb(buffer: &PixelBuffer<Rgb>) -> PixelBuffer<Rgb> {
    let buffer = buffer.to_srgb();
    let histogram = channel_histogram::<_, 3>(&buffer, 0);

    buffer.map_colors(|c| {
        Srgb::new_with_alpha(
            pick_value(&histogram[0], c.red()),
            pick_value(&histogram[1], c.green()),
            pick_value(&histogram[2], c.blue()),
            c.alpha(),
        )
        .to_rgb()
    })
}

fn equalize_saturation(buffer: &PixelBuffer<Rgb>) -> PixelBuffer<Rgb> {
    let buffer = buffer.to_hsl();
    let histogram = channel_histogram::<_, 1>(&buffer, 1);

    buffer.map_colors(|c| {
        Hsl::new_with_alpha(
            c.hue(),
            pick_value(&histogram[0], c.saturation()),
            c.lightness(),
            c.alpha(),
        )
        .to_rgb()
    })
}

fn equalize_lightness(buffer: &PixelBuffer<Rgb>) -> PixelBuffer<Rgb> {
    let buffer = buffer.to_hsl();
    let histogram = channel_histogram::<_, 1>(&buffer, 2);

    buffer.map_colors(|c| {
        Hsl::new_with_alpha(
            c.hue(),
            c.saturation(),
            pick_value(&histogram[0], c.lightness()),
            c.alpha(),
        )
        .to_rgb()
    })
}

fn equalize_saturation_lightness(buffer: &PixelBuffer<Rgb>) -> PixelBuffer<Rgb> {
    let buffer = buffer.to_hsl();
    let histogram = channel_histogram::<_, 2>(&buffer, 1);

    buffer.map_colors(|c| {
        Hsl::new_with_alpha(
            c.hue(),
            pick_value(&histogram[0], c.saturation()),
            pick_value(&histogram[1], c.lightness()),
            c.alpha(),
        )
        .to_rgb()
    })
}

pub fn equalize(buffer: &PixelBuffer<Rgb>, mode: EqualizeMode) -> PixelBuffer<Rgb> {
    match mode {
        EqualizeMode::Rgb => equalize_rgb(buffer),
        EqualizeMode::Srgb => equalize_srgb(buffer),
        EqualizeMode::Saturation => equalize_saturation(buffer),
        EqualizeMode::Lightness => equalize_lightness(buffer),
        EqualizeMode::SaturationLightness => equalize_saturation_lightness(buffer),
    }
}
