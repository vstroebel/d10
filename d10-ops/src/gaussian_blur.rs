use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::Rgb;
use d10_core::kernel::Kernel;

pub fn gaussian_blur(buffer: &PixelBuffer<Rgb>, radius: u32, sigma: f32) -> PixelBuffer<Rgb> {
    let kernel = Kernel::new_gaussian(radius * 2 + 1, sigma);
    buffer.apply_kernel(&kernel)
}