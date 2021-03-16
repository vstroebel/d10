use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::Color;
use rand::Rng;
use rand_distr::StandardNormal;

pub fn gaussian_noise<C>(buffer: &PixelBuffer<C>, alpha: f32) -> PixelBuffer<C>
    where C: Color {
    let mut rng = rand::thread_rng();

    buffer.map_colors(|c| c.map_color_channels(|v| {
        let noise: f32 = rng.sample(StandardNormal);
        noise * alpha + (1.0 - alpha) * v
    }))
}

pub fn add_gaussian_noise<C>(buffer: &mut PixelBuffer<C>, alpha: f32)
    where C: Color {
    let mut rng = rand::thread_rng();

    buffer.mod_colors(|c| c.map_color_channels(|v| {
        let noise: f32 = rng.sample(StandardNormal);
        noise * alpha + (1.0 - alpha) * v
    }));
}