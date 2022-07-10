use d10_core::color::Rgb;
use d10_core::pixelbuffer::PixelBuffer;
use rand::prelude::*;

fn rgb_noise_for_pixel<R>(c: &Rgb, threshold: f32, rng: &mut R) -> Rgb
where
    R: Rng,
{
    let value = rng.gen_range(0.0..=1.0);

    if value < threshold {
        match rng.gen_range(0..3) {
            0 => c.with_red(1.0),
            1 => c.with_green(1.0),
            _ => c.with_blue(1.0),
        }
    } else {
        *c
    }
}

pub fn rgb_noise(buffer: &PixelBuffer<Rgb>, threshold: f32) -> PixelBuffer<Rgb> {
    let mut rng = rand::thread_rng();

    buffer.map_colors(|c| rgb_noise_for_pixel(c, threshold, &mut rng))
}

pub fn add_rgb_noise(buffer: &mut PixelBuffer<Rgb>, threshold: f32) {
    let mut rng = rand::thread_rng();

    buffer.mod_colors(|c| rgb_noise_for_pixel(c, threshold, &mut rng));
}
