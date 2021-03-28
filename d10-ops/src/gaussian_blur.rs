use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::Rgb;
use d10_core::kernel_dyn::KernelDyn;
use d10_core::kernel::Kernel;

pub fn gaussian_blur(buffer: &PixelBuffer<Rgb>, radius: u32, sigma: f32) -> PixelBuffer<Rgb> {
    /*
     * This adds several optimized versions to speed up performance for small radii
     * Because this increases binary size and differences get smaller on larger kernels
     * this only implements 1 to 3 as the most common radii that are expected
     */

    match radius {
        1 => buffer.apply_kernel(&Kernel::<3>::new_gaussian(sigma)),
        2 => buffer.apply_kernel(&Kernel::<5>::new_gaussian(sigma)),
        3 => buffer.apply_kernel(&Kernel::<7>::new_gaussian(sigma)),
        _ => buffer.apply_kernel_dyn(&KernelDyn::new_gaussian(radius * 2 + 1, sigma))
    }
}