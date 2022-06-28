use d10_core::color::Color;
use d10_core::pixelbuffer::PixelBuffer;
use rand::prelude::*;

pub fn random_noise<C>(buffer: &PixelBuffer<C>, alpha: f32) -> PixelBuffer<C>
where
    C: Color,
{
    let mut rng = rand::thread_rng();

    buffer.map_colors(|c| {
        c.map_color_channels(|v| {
            let noise = rng.gen_range(-1.0..=1.0);
            noise * alpha + (1.0 - alpha) * v
        })
    })
}

pub fn add_random_noise<C>(buffer: &mut PixelBuffer<C>, alpha: f32)
where
    C: Color,
{
    let mut rng = rand::thread_rng();

    buffer.mod_colors(|c| {
        c.map_color_channels(|v| {
            let noise = rng.gen_range(-1.0..=1.0);
            noise * alpha + (1.0 - alpha) * v
        })
    });
}
