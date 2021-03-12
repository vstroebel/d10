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

pub fn resize(buffer: &PixelBuffer<RGB>, new_width: u32, new_height: u32, filter: FilterMode) -> PixelBuffer<RGB> {
    match filter {
        FilterMode::Nearest => resize_nearest(buffer, new_width, new_height),
        FilterMode::Bilinear => resize_bilinear(buffer, new_width, new_height)
    }
}