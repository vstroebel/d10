use d10_core::color::{Color, Rgb};
use d10_core::pixelbuffer::PixelBuffer;
pub fn optimize_lightness(buffer: &PixelBuffer<Rgb>, factor: f32) -> PixelBuffer<Rgb> {
    let mut sum = 0.0f64;

    for c in buffer.data() {
        sum += c.to_hsl().lightness() as f64;
    }

    let avg = sum / buffer.data().len() as f64;

    let gamma = 1.0 - (avg as f32 - 0.5) * factor;

    buffer.map_colors(|c| c.with_gamma(gamma))
}
