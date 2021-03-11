use crate::{ops, RGB, PixelBuffer, D10Result};
use std::path::Path;

#[derive(Clone)]
pub struct Image {
    buffer: PixelBuffer<RGB>,
    bg_color: Option<RGB>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            buffer: PixelBuffer::new(width, height).expect("New image"),
            bg_color: None,
        }
    }

    pub fn new_with_color(width: u32, height: u32, color: &RGB) -> Image {
        Image {
            buffer: PixelBuffer::new_with_color(width, height, color).expect("New image"),
            bg_color: None,
        }
    }

    pub fn new_from_raw(width: u32, height: u32, data: Vec<RGB>) -> Image {
        Image {
            buffer: PixelBuffer::new_from_raw(width, height, data).expect("New image"),
            bg_color: None,
        }
    }

    pub fn new_from_buffer(buffer: PixelBuffer<RGB>) -> Image {
        Image {
            buffer,
            bg_color: None,
        }
    }

    pub fn new_from_raw_with_meta(orig_image: &Image, width: u32, height: u32, data: Vec<RGB>) -> Image {
        Self::new_from_buffer_with_meta(orig_image, PixelBuffer::new_from_raw(width, height, data).expect("New buffer"))
    }

    pub fn new_from_buffer_with_meta(orig_image: &Image, buffer: PixelBuffer<RGB>) -> Image {
        Image {
            buffer,
            bg_color: orig_image.bg_color,
        }
    }

    pub fn open<P>(path: P) -> D10Result<Image> where P: AsRef<Path> {
        let buffer = crate::codecs::decode_file(path)?.buffer;
        Ok(Self::new_from_buffer(buffer))
    }

    pub fn read_from_buffer(buffer: &[u8]) -> D10Result<Image> {
        let buffer = crate::codecs::decode_buffer(buffer)?.buffer;
        Ok(Self::new_from_buffer(buffer))
    }

    pub fn save(&self, path: &str) -> D10Result<()> {
        crate::codecs::save_to_file(path, &self.buffer)
    }

    pub fn width(&self) -> u32 {
        self.buffer.width()
    }

    pub fn height(&self) -> u32 {
        self.buffer.height()
    }

    pub fn data(&self) -> &[RGB] {
        self.buffer.data()
    }

    pub fn data_mut(&mut self) -> &mut [RGB] {
        self.buffer.data_mut()
    }

    pub fn buffer(&self) -> &PixelBuffer<RGB> {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut PixelBuffer<RGB> {
        &mut self.buffer
    }

    pub fn has_transparency(&self) -> bool {
        self.buffer.has_transparency()
    }

    pub fn is_grayscale(&self) -> bool {
        self.buffer.is_grayscale()
    }

    pub fn mod_colors<F: Fn(&RGB) -> RGB>(&mut self, func: F) {
        self.buffer.map_colors(func);
    }

    pub fn try_mod_colors<E, F: Fn(&RGB) -> Result<RGB, E>>(&mut self, func: F) -> Result<(), E> {
        self.buffer.try_mod_colors(func)
    }

    pub fn mod_colors_enumerated<F: Fn(u32, u32, &RGB) -> RGB>(&mut self, func: F) {
        self.buffer.mod_colors_enumerated(func)
    }

    pub fn try_mod_colors_enumerated<E, F: Fn(u32, u32, &RGB) -> Result<RGB, E>>(&mut self, func: F) -> Result<(), E> {
        self.buffer.try_mod_colors_enumerated(func)
    }

    pub fn map_colors<F: FnMut(&RGB) -> RGB>(&self, func: F) -> Image {
        Self::new_from_buffer_with_meta(self, self.buffer.map_colors(func))
    }

    pub fn try_map_colors<E, F: FnMut(&RGB) -> Result<RGB, E>>(&self, func: F) -> Result<Image, E> {
        Ok(Self::new_from_buffer_with_meta(self, self.buffer.try_map_colors(func)?))
    }

    pub fn map_colors_enumerated<F: Fn(u32, u32, &RGB) -> RGB>(&self, func: F) -> Image {
        Self::new_from_buffer_with_meta(self, self.buffer.map_colors_enumerated(func))
    }

    pub fn try_map_colors_enumerated<E, F: Fn(u32, u32, &RGB) -> Result<RGB, E>>(&self, func: F) -> Result<Image, E> {
        Ok(Self::new_from_buffer_with_meta(self, self.buffer.try_map_colors_enumerated(func)?))
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> &RGB {
        self.buffer.get_pixel(x, y)
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: &RGB) {
        self.buffer.set_pixel(x, y, color);
    }

    pub fn get_pixel_clamped(&self, x: i32, y: i32) -> &RGB {
        self.buffer.get_pixel_clamped(x, y)
    }

    pub fn get_pixel_optional(&self, x: i32, y: i32) -> Option<&RGB> {
        self.buffer.get_pixel_optional(x, y)
    }

    /// Check whether the x/y coordinates are within the image dimensions
    pub fn is_in_image(&self, x: i32, y: i32) -> bool {
        self.buffer.is_in_image(x, y)
    }

    /// Flip image horizontally
    pub fn flip_horizontal(&self) -> Image {
        Self::new_from_buffer_with_meta(self, ops::flip_horizontal(&self.buffer))
    }

    /// Flip image vertically
    pub fn flip_vertical(&self) -> Image {
        Self::new_from_buffer_with_meta(self, ops::flip_vertical(&self.buffer))
    }

    /// Rotate image 90 degrees clockwise
    pub fn rotate90(&self) -> Image {
        Self::new_from_buffer_with_meta(self, ops::rotate90(&self.buffer))
    }

    /// Rotate image 180 degrees clockwise
    pub fn rotate180(&self) -> Image {
        Self::new_from_buffer_with_meta(self, ops::rotate180(&self.buffer))
    }

    /// Rotate image 270 degrees clockwise
    pub fn rotate270(&self) -> Image {
        Self::new_from_buffer_with_meta(self, ops::rotate270(&self.buffer))
    }
}

#[cfg(test)]
mod tests {
    use super::Image;
    use crate::RGB;

    fn flip_rotate_test_image() -> Image {
        Image::new_from_raw(3, 2, vec![
            RGB::WHITE, RGB::BLACK, RGB::YELLOW,
            RGB::RED, RGB::GREEN, RGB::BLUE
        ])
    }

    #[test]
    fn flip_horizontal() {
        let img_in = flip_rotate_test_image();

        let img_out = img_in.flip_horizontal();

        assert_eq!(img_in.get_pixel(0, 0), img_out.get_pixel(2, 0));
        assert_eq!(img_in.get_pixel(1, 0), img_out.get_pixel(1, 0));
        assert_eq!(img_in.get_pixel(2, 0), img_out.get_pixel(0, 0));

        assert_eq!(img_in.get_pixel(0, 1), img_out.get_pixel(2, 1));
        assert_eq!(img_in.get_pixel(1, 1), img_out.get_pixel(1, 1));
        assert_eq!(img_in.get_pixel(2, 1), img_out.get_pixel(0, 1));
    }

    #[test]
    fn flip_vertical() {
        let img_in = flip_rotate_test_image();

        let img_out = img_in.flip_vertical();

        assert_eq!(img_in.width(), img_out.width());
        assert_eq!(img_in.height(), img_out.height());

        assert_eq!(img_in.get_pixel(0, 0), img_out.get_pixel(0, 1));
        assert_eq!(img_in.get_pixel(1, 0), img_out.get_pixel(1, 1));
        assert_eq!(img_in.get_pixel(2, 0), img_out.get_pixel(2, 1));

        assert_eq!(img_in.get_pixel(0, 1), img_out.get_pixel(0, 0));
        assert_eq!(img_in.get_pixel(1, 1), img_out.get_pixel(1, 0));
        assert_eq!(img_in.get_pixel(2, 1), img_out.get_pixel(2, 0));
    }

    #[test]
    fn rotate90() {
        let img_in = flip_rotate_test_image();

        let img_out = img_in.rotate90();

        assert_eq!(img_in.width(), img_out.height());
        assert_eq!(img_in.height(), img_out.width());

        assert_eq!(img_in.get_pixel(0, 0), img_out.get_pixel(1, 0));
        assert_eq!(img_in.get_pixel(1, 0), img_out.get_pixel(1, 1));
        assert_eq!(img_in.get_pixel(2, 0), img_out.get_pixel(1, 2));

        assert_eq!(img_in.get_pixel(0, 1), img_out.get_pixel(0, 0));
        assert_eq!(img_in.get_pixel(1, 1), img_out.get_pixel(0, 1));
        assert_eq!(img_in.get_pixel(2, 1), img_out.get_pixel(0, 2));
    }

    #[test]
    fn rotate180() {
        let img_in = flip_rotate_test_image();

        let img_out = img_in.rotate180();

        assert_eq!(img_in.width(), img_out.width());
        assert_eq!(img_in.height(), img_out.height());

        assert_eq!(img_in.get_pixel(0, 0), img_out.get_pixel(2, 1));
        assert_eq!(img_in.get_pixel(1, 0), img_out.get_pixel(1, 1));
        assert_eq!(img_in.get_pixel(2, 0), img_out.get_pixel(0, 1));

        assert_eq!(img_in.get_pixel(0, 1), img_out.get_pixel(2, 0));
        assert_eq!(img_in.get_pixel(1, 1), img_out.get_pixel(1, 0));
        assert_eq!(img_in.get_pixel(2, 1), img_out.get_pixel(0, 0));
    }

    #[test]
    fn rotate270() {
        let img_in = flip_rotate_test_image();

        let img_out = img_in.rotate270();

        assert_eq!(img_in.width(), img_out.height());
        assert_eq!(img_in.height(), img_out.width());

        assert_eq!(img_in.get_pixel(0, 0), img_out.get_pixel(0, 2));
        assert_eq!(img_in.get_pixel(1, 0), img_out.get_pixel(0, 1));
        assert_eq!(img_in.get_pixel(2, 0), img_out.get_pixel(0, 0));

        assert_eq!(img_in.get_pixel(0, 1), img_out.get_pixel(1, 2));
        assert_eq!(img_in.get_pixel(1, 1), img_out.get_pixel(1, 1));
        assert_eq!(img_in.get_pixel(2, 1), img_out.get_pixel(1, 0));
    }
}