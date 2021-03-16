use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;

use crate::FilterMode;

/// Resize buffer with nearest neighbor filter
pub fn resize_nearest(buffer: &PixelBuffer<RGB>, new_width: u32, new_height: u32) -> PixelBuffer<RGB> {
    let scale_x = new_width as f32 / buffer.width() as f32;
    let scale_y = new_height as f32 / buffer.height() as f32;

    let result = (0..new_width * new_height)
        .map(|i| (i % new_width, i / new_width))
        .map(|(x, y)| {
            let x2 = (x as f32 / scale_x + 0.5).floor() as i32;
            let y2 = (y as f32 / scale_y + 0.5).floor() as i32;

            *buffer.get_pixel_clamped(x2, y2)
        }).collect();

    PixelBuffer::new_from_raw(new_width, new_height, result).expect("New buffer")
}

/// Resize buffer with bilinear filter
pub fn resize_bilinear(buffer: &PixelBuffer<RGB>, new_width: u32, new_height: u32) -> PixelBuffer<RGB> {
    let scale_x = (new_width as f32) / (buffer.width() as f32);
    let scale_y = (new_height as f32) / (buffer.height() as f32);

    let result = (0..new_width * new_height)
        .map(|i| (i % new_width, i / new_width))
        .map(|(x, y)| {
            let gx = (x as f32 + 0.5) / scale_x - 0.5;
            let gy = (y as f32 + 0.5) / scale_y - 0.5;

            crate::filters::get_pixel_bilinear(buffer, gx, gy)
        }).collect();

    PixelBuffer::new_from_raw(new_width, new_height, result).expect("New buffer")
}

/// Resize buffer with bicubic filter
pub fn resize_bicubic(buffer: &PixelBuffer<RGB>, new_width: u32, new_height: u32) -> PixelBuffer<RGB> {
    let scale_x = (new_width as f32) / (buffer.width() as f32);
    let scale_y = (new_height as f32) / (buffer.height() as f32);

    let result = (0..new_width * new_height)
        .map(|i| (i % new_width, i / new_width))
        .map(|(x, y)| {
            let gx = (x as f32 + 0.5) / scale_x - 0.5;
            let gy = (y as f32 + 0.5) / scale_y - 0.5;

            crate::filters::get_pixel_bicubic(buffer, gx, gy)
        }).collect();

    PixelBuffer::new_from_raw(new_width, new_height, result).expect("New buffer")
}

pub fn resize(buffer: &PixelBuffer<RGB>, new_width: u32, new_height: u32, filter: FilterMode) -> PixelBuffer<RGB> {
    match filter {
        FilterMode::Nearest => resize_nearest(buffer, new_width, new_height),
        FilterMode::Bilinear => resize_bilinear(buffer, new_width, new_height),
        FilterMode::Bicubic => resize_bicubic(buffer, new_width, new_height),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //NOTE: All of this test only check if images with only one color don't have any visual corruptions
    //      Testing the algorithms itself should be done in external integration tests

    fn check_color(buffer: &PixelBuffer<RGB>, color: RGB) {
        for (x, y, c) in buffer.enumerate() {
            assert_eq!(c, color, "Bad color at position {}x{}: Expected {:?} got {:?}", x, y, color, c)
        }
    }

    fn check_resize(color: RGB, filter: FilterMode) {
        let img_in = PixelBuffer::new_with_color(100, 100, color).unwrap();
        let img_out = resize(&img_in, 133, 166, filter);
        check_color(&img_out, color);
        assert_eq!(img_out.width(), 133);
        assert_eq!(img_out.height(), 166);

        let img_in = PixelBuffer::new_with_color(100, 100, color).unwrap();
        let img_out = resize(&img_in, 66, 33, filter);
        check_color(&img_out, color);
        assert_eq!(img_out.width(), 66);
        assert_eq!(img_out.height(), 33);

        let img_in = PixelBuffer::new_with_color(100, 100, color).unwrap();
        let img_out = resize(&img_in, 9, 8, filter);
        check_color(&img_out, color);
        assert_eq!(img_out.width(), 9);
        assert_eq!(img_out.height(), 8);
    }

    fn check_resize_colors(filter: FilterMode) {
        check_resize(RGB::BLACK, filter);
        check_resize(RGB::WHITE, filter);
        check_resize(RGB::RED, filter);
        check_resize(RGB::GREEN, filter);
        check_resize(RGB::BLUE, filter);
        check_resize(RGB::CYAN, filter);
        check_resize(RGB::MAGENTA, filter);
        check_resize(RGB::YELLOW, filter);
        check_resize(RGB::new(0.5, 0.5, 0.5), filter);
        check_resize(RGB::new(1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0), filter);
        check_resize(RGB::new(2.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0), filter);
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