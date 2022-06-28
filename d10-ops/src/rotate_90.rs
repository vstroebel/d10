use d10_core::color::Color;
use d10_core::pixelbuffer::PixelBuffer;

/// Rotate buffer 90 degrees clockwise
pub fn rotate90<C>(buffer: &PixelBuffer<C>) -> PixelBuffer<C>
where
    C: Color,
{
    let mut result = vec![Default::default(); buffer.data().len()];

    for x in 0..buffer.width() {
        for y in 0..buffer.height() {
            let x2 = buffer.height() - y - 1;
            let y2 = x;
            result[(x2 + y2 * buffer.height()) as usize] =
                buffer.data()[(x + y * buffer.width()) as usize];
        }
    }

    PixelBuffer::new_from_raw(buffer.height(), buffer.width(), result)
}

/// Rotate buffer 180 degrees clockwise
pub fn rotate180<C>(buffer: &PixelBuffer<C>) -> PixelBuffer<C>
where
    C: Color,
{
    let mut result = vec![Default::default(); buffer.data().len()];

    for x in 0..buffer.width() {
        for y in 0..buffer.height() {
            let x2 = buffer.width() - x - 1;
            let y2 = buffer.height() - y - 1;
            result[(x2 + y2 * buffer.width()) as usize] =
                buffer.data()[(x + y * buffer.width()) as usize];
        }
    }

    PixelBuffer::new_from_raw(buffer.width(), buffer.height(), result)
}

/// Rotate buffer 270 degrees clockwise
pub fn rotate270<C>(buffer: &PixelBuffer<C>) -> PixelBuffer<C>
where
    C: Color,
{
    let mut result = vec![Default::default(); buffer.data().len()];

    for x in 0..buffer.width() {
        for y in 0..buffer.height() {
            let x2 = y;
            let y2 = buffer.width() - x - 1;
            result[(x2 + y2 * buffer.height()) as usize] =
                buffer.data()[(x + y * buffer.width()) as usize];
        }
    }

    PixelBuffer::new_from_raw(buffer.height(), buffer.width(), result)
}
