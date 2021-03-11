use crate::{RGB, PixelBuffer, D10Result};
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

    pub fn open<P>(path: P) -> D10Result<Image> where P: AsRef<Path> {
        let buffer = crate::codecs::decode_file(path)?.buffer;
        Ok(Image::new_from_buffer(buffer))
    }

    pub fn read_from_buffer(buffer: &[u8]) -> D10Result<Image> {
        let buffer = crate::codecs::decode_buffer(buffer)?.buffer;
        Ok(Image::new_from_buffer(buffer))
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
        Image::new_from_buffer_with_meta(self, self.buffer.map_colors(func))
    }

    pub fn try_map_colors<E, F: FnMut(&RGB) -> Result<RGB, E>>(&self, func: F) -> Result<Image, E> {
        Ok(Image::new_from_buffer_with_meta(self, self.buffer.try_map_colors(func)?))
    }

    pub fn map_colors_enumerated<F: Fn(u32, u32, &RGB) -> RGB>(&self, func: F) -> Image {
        Image::new_from_buffer_with_meta(self, self.buffer.map_colors_enumerated(func))
    }

    pub fn try_map_colors_enumerated<E, F: Fn(u32, u32, &RGB) -> Result<RGB, E>>(&self, func: F) -> Result<Image, E> {
        Ok(Image::new_from_buffer_with_meta(self, self.buffer.try_map_colors_enumerated(func)?))
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

    pub fn is_in_image(&self, x: i32, y: i32) -> bool {
        self.buffer.is_in_image(x, y)
    }
}