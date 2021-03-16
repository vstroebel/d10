use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;
use d10_core::kernel::Kernel;

pub fn gaussian_blur(buffer: &PixelBuffer<RGB>, radius: u32, sigma: f32) -> PixelBuffer<RGB> {
    let kernel = Kernel::new_gaussian(radius * 2 + 1, sigma);
    buffer.apply_kernel(&kernel)
}