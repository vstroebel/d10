use d10_core::color::{Color, Hsl, Hsv, Lch, Rgb, Srgb};
use d10_core::errors::ParseEnumError;
use d10_core::pixelbuffer::PixelBuffer;
use std::str::FromStr;

fn get_color_values<C: Color + From<Rgb>, const START: usize, const NUM_CHANNELS: usize>(
    buffer: &PixelBuffer<Rgb>,
) -> [[f32; 256]; NUM_CHANNELS] {
    let mut result = [[0.0; 256]; NUM_CHANNELS];

    let len = buffer.data().len() as f32;

    for c in buffer.data() {
        let c: C = (*c).into();
        for (channel, res) in result.iter_mut().enumerate() {
            let v = (c.data()[START + channel] * 255.0).clamp(0.0, 255.0) as u8;
            res[v as usize] += 1.0 / len;
        }
    }

    result
}

pub fn get_min_value(values: &[f32; 256], threshold: f32) -> f32 {
    let mut min_value = 1.0;
    let mut agg = 0.0;

    for (i, v) in values.iter().enumerate() {
        agg += *v;
        if agg > threshold {
            min_value = i as f32 / 255.0;
            break;
        }
    }

    min_value
}

pub fn get_max_value(values: &[f32; 256], threshold: f32) -> f32 {
    let mut max_value = 1.0;
    let mut agg = 0.0;

    for (i, v) in values.iter().enumerate().rev() {
        agg += *v;
        if agg > threshold {
            max_value = i as f32 / 255.0;
            break;
        }
    }

    max_value
}

fn level_channel(value: f32, black_point: f32, white_point: f32) -> f32 {
    let diff = white_point - black_point;

    // Prevent channels with little differences to change too much
    let scale = (diff.abs() * 10.0).tanh();

    if scale <= f32::EPSILON {
        value
    } else {
        let factor = 1.0 / diff;
        (value - black_point * scale) * factor * scale
    }
}

fn balance_buffer<C: Color + From<Rgb>, const START: usize, const NUM_CHANNELS: usize>(
    buffer: &PixelBuffer<Rgb>,
    threshold: f32,
) -> PixelBuffer<Rgb> {
    let threshold = threshold / 1000.0;

    let values = get_color_values::<C, START, NUM_CHANNELS>(buffer);

    let mut min = [0.0; NUM_CHANNELS];
    let mut max = [0.0; NUM_CHANNELS];

    for i in 0..NUM_CHANNELS {
        min[i] = get_min_value(&values[i], threshold);
        max[i] = get_max_value(&values[i], threshold);
    }

    buffer.map_colors(|c| {
        let mut c: C = (*c).into();

        for i in 0..NUM_CHANNELS {
            c.data_mut()[START + i] = level_channel(c.data()[START + i], min[i], max[i])
        }

        c.to_rgb()
    })
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BalanceMode {
    Rgb,
    Srgb,
    Hsv,
    Hsl,
    Lch,
}

impl FromStr for BalanceMode {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use BalanceMode::*;
        match value {
            "rgb" => Ok(Rgb),
            "srgb" => Ok(Srgb),
            "hsv" => Ok(Hsv),
            "hsl" => Ok(Hsl),
            "lch" => Ok(Lch),
            _ => Err(ParseEnumError::new(value, "BalanceMode")),
        }
    }
}

pub fn balance(buffer: &PixelBuffer<Rgb>, mode: BalanceMode, threshold: f32) -> PixelBuffer<Rgb> {
    match mode {
        BalanceMode::Rgb => balance_buffer::<Rgb, 0, 3>(buffer, threshold),
        BalanceMode::Srgb => balance_buffer::<Srgb, 0, 3>(buffer, threshold),
        BalanceMode::Hsv => balance_buffer::<Hsv, 1, 2>(buffer, threshold),
        BalanceMode::Hsl => balance_buffer::<Hsl, 1, 2>(buffer, threshold),
        BalanceMode::Lch => balance_buffer::<Lch, 0, 2>(buffer, threshold),
    }
}
