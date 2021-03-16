use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;
use d10_core::kernel::Kernel;

pub fn unsharp(buffer: &PixelBuffer<RGB>, radius: u32, sigma: f32, factor: f32) -> PixelBuffer<RGB> {
    let kernel = Kernel::new_gaussian(radius * 2 + 1, sigma);

    buffer.map_colors_enumerated(|x, y, c| {
        let c_blurred = buffer.get_kernel_value(x, y, &kernel);

        RGB::new_with_alpha(
            c.data[0] + (c.data[0] - c_blurred.data[0]) * factor,
            c.data[1] + (c.data[1] - c_blurred.data[1]) * factor,
            c.data[2] + (c.data[2] - c_blurred.data[2]) * factor,
            c.data[3] + (c.data[3] - c_blurred.data[3]) * factor,
        )
    })
}