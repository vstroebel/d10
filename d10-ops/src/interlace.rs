use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::Color;

pub fn interlace<C: Color>(buffer: &PixelBuffer<C>, offset: u32) -> PixelBuffer<C> {
    let offset = offset as i32;

    PixelBuffer::new_from_func(buffer.width(), buffer.height(), |x, y| {
        *if y % 2 == 0 {
            buffer.get_pixel_clamped(x as i32 - offset, y as i32)
        } else {
            buffer.get_pixel_clamped(x as i32 + offset, y as i32)
        }
    })
}