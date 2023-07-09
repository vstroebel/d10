use d10_core::color::{Color, Rgb};
use d10_core::kernel_dyn::KernelDyn;
use d10_core::pixelbuffer::PixelBuffer;

use crate::FilterMode;

/// Resize buffer
fn resize_with_fn<F>(
    buffer: &PixelBuffer<Rgb>,
    new_width: u32,
    new_height: u32,
    func: F,
) -> PixelBuffer<Rgb>
where
    F: Fn(&PixelBuffer<Rgb>, u32, u32, f32, f32) -> Rgb,
{
    let scale_x = (new_width as f32) / (buffer.width() as f32);
    let scale_y = (new_height as f32) / (buffer.height() as f32);

    PixelBuffer::new_from_func(new_width, new_height, |x, y| {
        func(buffer, x, y, scale_x, scale_y)
    })
}

fn resize_pixel_nearest(
    buffer: &PixelBuffer<Rgb>,
    x: u32,
    y: u32,
    scale_x: f32,
    scale_y: f32,
) -> Rgb {
    let x2 = (x as f32 / scale_x + 0.5).floor() as i32;
    let y2 = (y as f32 / scale_y + 0.5).floor() as i32;
    *buffer.get_pixel_clamped(x2, y2)
}

fn resize_pixel_bilinear(
    buffer: &PixelBuffer<Rgb>,
    x: u32,
    y: u32,
    scale_x: f32,
    scale_y: f32,
) -> Rgb {
    let gx = (x as f32 + 0.5) / scale_x - 0.5;
    let gy = (y as f32 + 0.5) / scale_y - 0.5;
    crate::filters::get_pixel_bilinear(buffer, gx, gy)
}

fn resize_pixel_bicubic(
    buffer: &PixelBuffer<Rgb>,
    x: u32,
    y: u32,
    scale_x: f32,
    scale_y: f32,
) -> Rgb {
    let gx = (x as f32 + 0.5) / scale_x - 0.5;
    let gy = (y as f32 + 0.5) / scale_y - 0.5;
    crate::filters::get_pixel_bicubic(buffer, gx, gy)
}

fn resize_pixel_lanczos3(
    buffer: &PixelBuffer<Rgb>,
    x: u32,
    y: u32,
    scale_x: f32,
    scale_y: f32,
) -> Rgb {
    let gx = (x as f32 + 0.5) / scale_x - 0.5;
    let gy = (y as f32 + 0.5) / scale_y - 0.5;
    crate::filters::get_pixel_lanczos3(buffer, gx, gy)
}

fn resize_auto(
    buffer: &PixelBuffer<Rgb>,
    new_width: u32,
    new_height: u32,
) -> PixelBuffer<Rgb>
{
    let scale_x = (new_width as f32) / (buffer.width() as f32);
    let scale_y = (new_height as f32) / (buffer.height() as f32);

    let base_kernel_size = ((1.0 / scale_x.min(scale_y)) * 1.75).max(3.0);

    let kernel_size = base_kernel_size.ceil() as u32;
    let kernel_size2 = (base_kernel_size * 1.5).ceil() as u32;

    let kernel = KernelDyn::new_gaussian(kernel_size, 2.0);
    let kernel2 = KernelDyn::new_gaussian(kernel_size2, 4.0);

    let factor = 0.5;

    PixelBuffer::new_from_func(new_width, new_height, |x, y| {
        let gx = ((x as f32 + 0.5) / scale_x - 0.5) as i32;
        let gy = ((y as f32 + 0.5) / scale_y - 0.5) as i32;

        let gx = gx.max(0).min(buffer.width() as i32 -1) as u32;
        let gy = gy.max(0).min(buffer.height() as i32 -1) as u32;

        let c1 = buffer.get_kernel_value(gx, gy, &kernel);
        let c2 = buffer.get_kernel_value(gx, gy, &kernel2);

        Rgb::new_with_alpha(
            c1.red() + (c1.red() - c2.red()) * factor,
            c1.green() + (c1.green() - c2.green()) * factor,
            c1.blue() + (c1.blue() - c2.blue()) * factor,
            c1.alpha() + (c1.alpha() - c2.alpha()) * factor
        )
    })
}

pub fn resize(
    buffer: &PixelBuffer<Rgb>,
    new_width: u32,
    new_height: u32,
    filter: FilterMode,
) -> PixelBuffer<Rgb> {
    if buffer.width() == new_width && buffer.height() == new_height {
        return buffer.clone();
    }

    match filter {
        FilterMode::Nearest => resize_with_fn(buffer, new_width, new_height, resize_pixel_nearest),
        FilterMode::Bilinear => {
            resize_with_fn(buffer, new_width, new_height, resize_pixel_bilinear)
        }
        FilterMode::Bicubic => resize_with_fn(buffer, new_width, new_height, resize_pixel_bicubic),
        FilterMode::Lanczos3 => {
            resize_with_fn(buffer, new_width, new_height, resize_pixel_lanczos3)
        }
        FilterMode::Auto => {
            if buffer.width() <= new_width || buffer.height() <= new_height {
                resize_with_fn(buffer, new_width, new_height, resize_pixel_bicubic)
            } else{
                resize_auto(buffer, new_width, new_height)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //NOTE: All of this test only check if images with only one color don't have any visual corruptions
    //      Testing the algorithms itself should be done in external integration tests

    fn check_color(buffer: &PixelBuffer<Rgb>, color: Rgb) {
        for (x, y, c) in buffer.enumerate() {
            assert_eq!(
                c, color,
                "Bad color at position {}x{}: Expected {:?} got {:?}",
                x, y, color, c
            )
        }
    }

    fn check_resize(color: Rgb, filter: FilterMode) {
        let img_in = PixelBuffer::new_with_color(100, 100, color);
        let img_out = resize(&img_in, 133, 166, filter);
        check_color(&img_out, color);
        assert_eq!(img_out.width(), 133);
        assert_eq!(img_out.height(), 166);

        let img_in = PixelBuffer::new_with_color(100, 100, color);
        let img_out = resize(&img_in, 66, 33, filter);
        check_color(&img_out, color);
        assert_eq!(img_out.width(), 66);
        assert_eq!(img_out.height(), 33);

        let img_in = PixelBuffer::new_with_color(100, 100, color);
        let img_out = resize(&img_in, 9, 8, filter);
        check_color(&img_out, color);
        assert_eq!(img_out.width(), 9);
        assert_eq!(img_out.height(), 8);
    }

    fn check_resize_colors(filter: FilterMode) {
        check_resize(Rgb::BLACK, filter);
        check_resize(Rgb::WHITE, filter);
        check_resize(Rgb::RED, filter);
        check_resize(Rgb::GREEN, filter);
        check_resize(Rgb::BLUE, filter);
        check_resize(Rgb::CYAN, filter);
        check_resize(Rgb::MAGENTA, filter);
        check_resize(Rgb::YELLOW, filter);
        check_resize(Rgb::new(0.5, 0.5, 0.5), filter);
        check_resize(Rgb::new(1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0), filter);
        check_resize(Rgb::new(2.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0), filter);
    }

    #[test]
    fn test_nearest() {
        check_resize_colors(FilterMode::Nearest);
    }

    #[test]
    fn test_bilinear() {
        check_resize_colors(FilterMode::Bilinear);
    }

    #[test]
    fn test_bicubic() {
        check_resize_colors(FilterMode::Bicubic);
    }
}
