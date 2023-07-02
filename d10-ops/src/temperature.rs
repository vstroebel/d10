use d10_core::color::{Color, Rgb, Xyz};
use d10_core::pixelbuffer::PixelBuffer;
use rand_distr::num_traits::Pow;

pub fn change_color_temperature(
    buffer: &PixelBuffer<Rgb>,
    orig_temp: f32,
    new_temp: f32,
    tint_correction: f32,
) -> PixelBuffer<Rgb> {
    let factors = calculate_factors(orig_temp, new_temp);

    let tint_pow = if tint_correction > 0.0 {
        get_green_tint_pow(buffer, tint_correction).max(0.0)
    } else {
        0.0
    };

    let mut res = if tint_pow > 0.0 {
        buffer.map_colors(|c| {
            Rgb::new_with_alpha(
                c.red() * factors[0],
                c.green().pow(tint_pow) * factors[1],
                c.blue() * factors[2],
                c.alpha(),
            )
        })
    } else {
        buffer.map_colors(|c| {
            Rgb::new_with_alpha(
                c.red() * factors[0],
                c.green() * factors[1],
                c.blue() * factors[2],
                c.alpha(),
            )
        })
    };

    let red_pow = get_red_tint_pow(buffer, tint_correction);
    let blue_pow = get_blue_tint_pow(buffer, tint_correction);

    if red_pow > 0.0 {
        res.mod_colors(|c| c.with_red(c.red().powf(red_pow)));
    }

    if blue_pow > 0.0 {
        res.mod_colors(|c| c.with_blue(c.blue().powf(blue_pow)));
    }

    res
}

fn calculate_factors(orig_temp: f32, new_temp: f32) -> [f32; 3] {
    let orig = convert_kelvin_to_rgb(orig_temp);
    let new = convert_kelvin_to_rgb(new_temp);

    [
        orig.data()[0] / new.data()[0],
        orig.data()[1] / new.data()[1],
        orig.data()[2] / new.data()[2],
    ]
}

fn convert_kelvin_to_rgb(temperature: f32) -> Rgb {
    let temperature = temperature as usize;
    let temperature = temperature.clamp(1000, 12000);

    let i = temperature / 1000;

    let (mut x, mut y) = TEMPERATURE_TABLE[i - 1];

    let floor_temp = (i * 1000) as f64;
    let temperature = temperature  as f64;

    if (temperature - floor_temp).abs() > 1.0 {
        let (x2, y2) = TEMPERATURE_TABLE[i];
        let f = (temperature - floor_temp) / 1000.0;

        x = x * (1.0 - f) + x2 * f;
        y = y * (1.0 - f) + y2 * f;
    }

    let z = 1.0 - x - y;
    let s = 1.0 / y;

    Xyz::new((x * s) as f32, (y * s) as f32, (z * s) as f32).to_rgb()
}

//CIE x,y values for temperatures between 1000 and 12000
const TEMPERATURE_TABLE: [(f64, f64); 12] = [
    (0.652750055750174, 0.344462227197370),
    (0.526676280311873, 0.413297274507630),
    (0.436929833678155, 0.404073616886221),
    (0.380438429420364, 0.376746069841299),
    (0.345100160725069, 0.351607005318840),
    (0.322082269887888, 0.331752126277376),
    (0.306372718718652, 0.316511125739794),
    (0.295186142428596, 0.304763622626521),
    (0.286924765725065, 0.295581717193809),
    (0.280632719756407, 0.288286029784579),
    (0.275714062105148, 0.282393589935205),
    (0.271782994107569, 0.277561259748537),
];

fn get_green_tint_pow(buffer: &PixelBuffer<Rgb>, tint_correction: f32) -> f32 {
    if tint_correction <= 0.0 {
        return 0.0;
    }

    let mut sum = 0.0;

    for c in buffer.data() {
        sum += c.green() - c.red() + c.green() - c.blue();
    }

    let avg = sum * tint_correction / buffer.data().len() as f32;

    if avg > 0.0 {
        1.0 / (1.0 - avg.tanh())
    } else {
        0.0
    }
}

fn get_red_tint_pow(buffer: &PixelBuffer<Rgb>, tint_correction: f32) -> f32 {
    if tint_correction <= 0.0 {
        return 0.0;
    }

    let mut sum = 0.0;

    for c in buffer.data() {
        sum += c.red() - c.green() + c.red() - c.blue();
    }

    let avg = 0.25 * sum * tint_correction / buffer.data().len() as f32;

    if avg > 0.0 {
        1.0 / (1.0 - avg.tanh())
    } else {
        0.0
    }
}

fn get_blue_tint_pow(buffer: &PixelBuffer<Rgb>, tint_correction: f32) -> f32 {
    if tint_correction <= 0.0 {
        return 0.0;
    }

    let mut sum = 0.0;

    for c in buffer.data() {
        sum += c.blue() - c.green() + c.blue() - c.red();
    }

    let avg = 0.33 * sum * tint_correction / buffer.data().len() as f32;

    if avg > 0.0 {
        1.0 / (1.0 - avg.tanh())
    } else {
        0.0
    }
}

pub fn optimize_color_temperature(
    buffer: &PixelBuffer<Rgb>,
    factor: f32,
    tint_correction: f32,
) -> PixelBuffer<Rgb> {
    let mut sum = 0.0f64;

    for c in buffer.data() {
        sum += c.red() as f64 - c.blue() as f64;
    }

    let avg = sum / buffer.data().len() as f64;
    let new_temp = ((6500.0) - avg.tanh() * (factor as f64 * 4000.0)) as f32;

    change_color_temperature(buffer, 6500.0, new_temp, tint_correction)
}
