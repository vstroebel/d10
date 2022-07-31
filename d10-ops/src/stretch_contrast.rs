use d10_core::color::{Intensity, Rgb};
use d10_core::pixelbuffer::PixelBuffer;

fn get_color_value(c: &Rgb) -> u8 {
    let v = c.to_gray_with_intensity(Intensity::Average).red() * 255.0;
    v.clamp(0.0, 255.0) as u8
}

fn get_color_values(buffer: &PixelBuffer<Rgb>) -> [f32; 256] {
    let mut result = [0.0; 256];

    let len = buffer.data().len() as f32;

    for c in buffer.data() {
        let v = get_color_value(c);
        result[v as usize] += 1.0 / len;
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

pub fn stretch_contrast(buffer: &PixelBuffer<Rgb>, threshold: f32) -> PixelBuffer<Rgb> {
    let threshold = threshold / 1000.0;

    let values = get_color_values(buffer);

    let min_value = get_min_value(&values, threshold);
    let max_value = get_max_value(&values, threshold);

    if min_value > 0.0 || max_value < 1.0 {
        buffer.map_colors(|c| c.with_level(min_value, max_value, 1.0))
    } else {
        buffer.clone()
    }
}
