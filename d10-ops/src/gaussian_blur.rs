use d10_core::color::Rgb;
use d10_core::kernel::Kernel;
use d10_core::kernel_dyn::KernelDyn;
use d10_core::pixelbuffer::PixelBuffer;

pub fn gaussian_blur(
    buffer: &PixelBuffer<Rgb>,
    radius: u32,
    sigma: Option<f32>,
) -> PixelBuffer<Rgb> {
    /*
     * This adds several optimized versions to speed up performance for small radii
     * Because this increases binary size and differences get smaller on larger kernels
     * this only implements 1 to 3 as the most common radii that are expected
     */

    let kernel_size = radius * 2 + 1;
    let sigma = sigma.unwrap_or_else(|| get_default_sigma(kernel_size));

    match kernel_size {
        3 => buffer.apply_kernel(&Kernel::<3>::new_gaussian(sigma)),
        5 => buffer.apply_kernel(&Kernel::<5>::new_gaussian(sigma)),
        7 => buffer.apply_kernel(&Kernel::<7>::new_gaussian(sigma)),
        _ => buffer.apply_kernel_dyn(&KernelDyn::new_gaussian(kernel_size, sigma)),
    }
}

pub(crate) fn get_default_sigma(kernel_size: u32) -> f32 {
    (kernel_size as f32 - 1.0) / 4.0
}
