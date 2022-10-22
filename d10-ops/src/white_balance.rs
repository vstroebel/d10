use d10_core::color::Rgb;
use d10_core::pixelbuffer::PixelBuffer;

fn get_color_values(buffer: &PixelBuffer<Rgb>) -> [[f32; 256]; 3] {
    let mut result = [[0.0; 256]; 3];

    let len = buffer.data().len() as f32;

    for c in buffer.data() {
        for channel in 0..3 {
            let v = (c.data[channel] * 255.0).clamp(0.0, 255.0) as u8;
            result[channel][v as usize] += 1.0 / len;
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
    let factor = if diff.abs() < f32::EPSILON {
        1.0 / f32::EPSILON
    } else {
        1.0 / diff
    };

   (value - black_point) * factor
}

pub fn white_balance(buffer: &PixelBuffer<Rgb>, threshold: f32) -> PixelBuffer<Rgb> {
    let threshold = threshold / 1000.0;

    let values = get_color_values(buffer);

    let min = [
        get_min_value(&values[0], threshold),
        get_min_value(&values[1], threshold),
        get_min_value(&values[2], threshold),
    ];

    let max = [
        get_max_value(&values[0], threshold),
        get_max_value(&values[1], threshold),
        get_max_value(&values[2], threshold),
    ];

    buffer.map_colors(|c| {
        c.with_red(level_channel(c.red(), min[0], max[0]))
            .with_green(level_channel(c.green(), min[1], max[1]))
            .with_blue(level_channel(c.blue(), min[2], max[2]))
    })
}
