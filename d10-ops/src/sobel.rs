use d10_core::color::RGB;
use d10_core::pixelbuffer::PixelBuffer;
use d10_core::kernel::Kernel;

fn apply_kernel_2(buffer: &PixelBuffer<RGB>, normalize: bool, kernel_x: &Kernel, kernel_y: &Kernel) -> PixelBuffer<RGB> {
    let ix = buffer.apply_kernel(kernel_x);
    let iy = buffer.apply_kernel(kernel_y);

    let mut max = f32::MIN;
    let mut min = f32::MAX;

    let mut g: Vec<f32> = ix.data().iter().zip(iy.data().iter()).map(|(c1, c2)| {
        let v1 = c1.to_gray().red();
        let v2 = c2.to_gray().red();

        let v = (v1 * v1 + v2 * v2).sqrt();
        if v > max {
            max = v;
        }
        if v < min {
            min = v;
        }
        v
    }).collect();

    if normalize {
        let diff = max - min;

        for v in g.iter_mut() {
            *v = (*v - min) / diff;
        }
    }

    let data = g.iter().map(|v| RGB::new(*v, *v, *v)).collect();

    PixelBuffer::new_from_raw(buffer.width(), buffer.height(), data)
}

/// Detect edges in the buffer with a sobel kernel
///
/// If `normalize` is true the resulting color channel values will be between 0.0 and 1.0
pub fn sobel_edge_detection(buffer: &PixelBuffer<RGB>, normalize: bool) -> PixelBuffer<RGB> {
    apply_kernel_2(buffer, normalize, &Kernel::new_sobel_x(), &Kernel::new_sobel_y())
}