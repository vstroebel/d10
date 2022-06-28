use d10_core::color::Rgb;
use d10_core::pixelbuffer::PixelBuffer;
use rand::Rng;

fn salt_n_pepper<R>(c: &Rgb, threshold: f32, rng: &mut R) -> Rgb
where
    R: Rng,
{
    let value = rng.gen_range(0.0..=1.0);

    if value < threshold {
        Rgb::BLACK
    } else if value > 1.0 - threshold {
        Rgb::WHITE
    } else {
        *c
    }
}

pub fn salt_n_pepper_noise(buffer: &PixelBuffer<Rgb>, threshold: f32) -> PixelBuffer<Rgb> {
    let mut rng = rand::thread_rng();

    buffer.map_colors(|c| salt_n_pepper(c, threshold, &mut rng))
}

pub fn add_salt_n_pepper_noise(buffer: &mut PixelBuffer<Rgb>, threshold: f32) {
    let mut rng = rand::thread_rng();

    buffer.mod_colors(|c| salt_n_pepper(c, threshold, &mut rng));
}
