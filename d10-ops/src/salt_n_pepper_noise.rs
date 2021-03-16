use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;
use rand::Rng;

fn salt_n_pepper<R>(c: &RGB, threshold: f32, rng: &mut R) -> RGB
    where R: Rng
{
    let value = rng.gen_range(0.0..=1.0);

    if value < threshold {
        RGB::BLACK
    } else if value > 1.0 - threshold {
        RGB::WHITE
    } else {
        *c
    }
}

pub fn salt_n_pepper_noise(buffer: &PixelBuffer<RGB>, threshold: f32) -> PixelBuffer<RGB> {
    let mut rng = rand::thread_rng();

    buffer.map_colors(|c| salt_n_pepper(c, threshold, &mut rng))
}

pub fn add_salt_n_pepper_noise(buffer: &mut PixelBuffer<RGB>, threshold: f32) {
    let mut rng = rand::thread_rng();

    buffer.mod_colors(|c| salt_n_pepper(c, threshold, &mut rng));
}