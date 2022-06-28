use d10_core::color::Rgb;
use d10_core::kernel_dyn::KernelDyn;
use d10_core::pixelbuffer::PixelBuffer;

pub fn unsharp(
    buffer: &PixelBuffer<Rgb>,
    radius: u32,
    factor: f32,
    sigma: Option<f32>,
) -> PixelBuffer<Rgb> {
    let kernel_size = radius * 2 + 1;

    let sigma = sigma.unwrap_or_else(|| crate::gaussian_blur::get_default_sigma(kernel_size));

    let kernel = KernelDyn::new_gaussian(kernel_size, sigma);

    buffer.map_colors_enumerated(|x, y, c| {
        let c_blurred = buffer.get_kernel_value(x, y, &kernel);

        Rgb::new_with_alpha(
            c.data[0] + (c.data[0] - c_blurred.data[0]) * factor,
            c.data[1] + (c.data[1] - c_blurred.data[1]) * factor,
            c.data[2] + (c.data[2] - c_blurred.data[2]) * factor,
            c.data[3] + (c.data[3] - c_blurred.data[3]) * factor,
        )
    })
}
