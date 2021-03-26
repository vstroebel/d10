use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::Color;

pub fn crop<C>(buffer: &PixelBuffer<C>, offset_x: u32, offset_y: u32, width: u32, height: u32) -> PixelBuffer<C>
    where C: Color
{
    if buffer.is_empty() {
        buffer.clone()
    } else {
        let offset_x = offset_x.min(buffer.width());
        let offset_y = offset_y.min(buffer.height());

        let width = width.min(buffer.width() - offset_x);
        let height = height.min(buffer.height() - offset_y);

        if width == 0 || height == 0 {
            return PixelBuffer::new(0, 0);
        }

        let mut data = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            let offset = ((offset_y + y) * buffer.width() + offset_x) as usize;

            let row = &buffer.data()[offset..offset + width as usize];

            data.extend_from_slice(&row);
        }

        PixelBuffer::new_from_raw(width, height, data)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use d10_core::pixelbuffer::PixelBuffer;
    use d10_core::color::Rgb;

    #[test]
    fn test_crop() {
        let buffer: PixelBuffer<Rgb> = PixelBuffer::new(100, 200);

        let cropped = crop(&buffer, 0, 0, 10, 20);
        assert_eq!(cropped.width(), 10);
        assert_eq!(cropped.height(), 20);

        let cropped = crop(&buffer, 200, 0, 10, 20);
        assert_eq!(cropped.width(), 0);
        assert_eq!(cropped.height(), 0);

        let cropped = crop(&buffer, 50, 50, 100, 200);
        assert_eq!(cropped.width(), 50);
        assert_eq!(cropped.height(), 150);
    }
}