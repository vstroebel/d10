use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::Color;

/// Flip buffer horizontally
pub fn flip_horizontal<C>(buffer: &PixelBuffer<C>) -> PixelBuffer<C> where C: Color {
    let mut result = vec![Default::default(); buffer.data().len()];

    for x in 0..buffer.width() {
        for y in 0..buffer.height() {
            result[((buffer.width() - x - 1) + y * buffer.width()) as usize] = buffer.data()[(x + y * buffer.width()) as usize];
        }
    }

    PixelBuffer::new_from_raw(buffer.width(), buffer.height(), result).unwrap()
}

/// Flip buffer vertically
pub fn flip_vertical<C>(buffer: &PixelBuffer<C>) -> PixelBuffer<C> where C: Color {
    let mut result = vec![Default::default(); buffer.data().len()];

    for x in 0..buffer.width() {
        for y in 0..buffer.height() {
            result[(x + (buffer.height() - y - 1) * buffer.width()) as usize] = buffer.data()[(x + y * buffer.width()) as usize];
        }
    }

    PixelBuffer::new_from_raw(buffer.width(), buffer.height(), result).unwrap()
}