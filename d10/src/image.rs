use crate::{ops, RGB, PixelBuffer, D10Result};
use std::path::Path;
use d10_ops::FilterMode;
use d10_codecs::EncodingFormat;
use std::io::Write;

#[derive(Clone)]
pub struct Image {
    buffer: PixelBuffer<RGB>,
    bg_color: Option<RGB>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            buffer: PixelBuffer::new(width, height),
            bg_color: None,
        }
    }

    pub fn new_with_color(width: u32, height: u32, color: RGB) -> Image {
        Image {
            buffer: PixelBuffer::new_with_color(width, height, color),
            bg_color: None,
        }
    }

    pub fn new_from_raw(width: u32, height: u32, data: Vec<RGB>) -> Image {
        Image {
            buffer: PixelBuffer::new_from_raw(width, height, data),
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
        Self::new_from_buffer_with_meta(orig_image, PixelBuffer::new_from_raw(width, height, data))
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

    pub fn save<P>(&self, path: P) -> D10Result<()>
        where P: AsRef<Path>
    {
        crate::codecs::encode_to_file(path, &self.buffer, None)
    }

    pub fn save_with_format<P>(&self, path: P, format: EncodingFormat) -> D10Result<()>
        where P: AsRef<Path>
    {
        crate::codecs::encode_to_file(path, &self.buffer, Some(format))
    }

    pub fn save_to_writer<W>(&self, w: &mut W, format: EncodingFormat) -> D10Result<()>
        where W: Write
    {
        crate::codecs::encode(w, &self.buffer, format)
    }

    pub fn save_to_buffer(&self, format: EncodingFormat) -> D10Result<Vec<u8>> {
        let mut out = vec![];
        crate::codecs::encode(&mut out, &self.buffer, format)?;
        Ok(out)
    }

    pub fn width(&self) -> u32 {
        self.buffer.width()
    }

    pub fn height(&self) -> u32 {
        self.buffer.height()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
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
        self.buffer.mod_colors(func);
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

    pub fn put_pixel(&mut self, x: u32, y: u32, color: RGB) {
        self.buffer.put_pixel(x, y, color);
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

    /// Return cropped image
    pub fn crop(&self, offset_x: u32, offset_y: u32, width: u32, height: u32) -> Image {
        Self::new_from_buffer_with_meta(self, ops::crop(&self.buffer, offset_x, offset_y, width, height))
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

    /// Rotate image clockwise with the given filter
    pub fn rotate(&self, radians: f32, filter: FilterMode) -> Self {
        Self::new_from_buffer_with_meta(self, ops::rotate(&self.buffer, radians, self.bg_color.unwrap_or(RGB::NONE), filter))
    }

    /// Detect edges in the image with a sobel kernel
    ///
    /// If `normalize` is true the resulting color channel values will be between 0.0 and 1.0
    pub fn sobel_edge_detection(&self, normalize: bool) -> Image {
        Self::new_from_buffer_with_meta(self, ops::sobel_edge_detection(&self.buffer, normalize))
    }

    /// Resize image
    pub fn resize(&self, new_width: u32, new_height: u32, filter: FilterMode) -> Image {
        Self::new_from_buffer_with_meta(self, ops::resize(&self.buffer, new_width, new_height, filter))
    }

    /// Resize image using the given percentage
    pub fn resize_pct(&self, pct_100: f32, filter: FilterMode) -> Image {
        let factor = pct_100 / 100.0;

        let width = ((self.width() as f32) * factor).round() as u32;
        let height = ((self.height() as f32) * factor).round() as u32;

        self.resize(width.max(1), height.max(1), filter)
    }

    /// Returns a new image with a simulated jpeg quality
    ///
    /// If `preserve_alpha` is not set all alpha values will be set to 1.0
    pub fn with_jpeg_quality(&self, quality: u8, preserve_alpha: bool) -> D10Result<Image> {
        Ok(Self::new_from_buffer_with_meta(self, ops::jpeg_quality(&self.buffer, quality, preserve_alpha)?))
    }

    /// Add random noise to the image
    pub fn random_noise(&self, alpha: f32) -> Image {
        Self::new_from_buffer_with_meta(self, ops::random_noise(&self.buffer, alpha))
    }

    /// Add random noise to the image
    pub fn add_random_noise(&mut self, alpha: f32) {
        ops::add_random_noise(&mut self.buffer, alpha);
    }

    /// Return a new image with salt n pepper noise
    pub fn salt_n_pepper_noise(&self, threshold: f32) -> Image {
        Self::new_from_buffer_with_meta(self, ops::salt_n_pepper_noise(&self.buffer, threshold))
    }

    /// Add salt n pepper noise to the image
    pub fn add_salt_n_pepper_noise(&mut self, threshold: f32) {
        ops::add_salt_n_pepper_noise(&mut self.buffer, threshold);
    }

    /// Return a new image with gaussian noise
    pub fn gaussian_noise(&self, alpha: f32) -> Image {
        Self::new_from_buffer_with_meta(self, ops::gaussian_noise(&self.buffer, alpha))
    }

    /// Add gaussian noise to the image
    pub fn add_gaussian_noise(&mut self, alpha: f32) {
        ops::add_gaussian_noise(&mut self.buffer, alpha);
    }

    /// Return a new image with gaussian blur
    pub fn gaussian_blur(&self, radius: u32, sigma: f32) -> Image {
        Self::new_from_buffer_with_meta(self, ops::gaussian_blur(&self.buffer, radius, sigma))
    }

    /// Return a new image with unsharp mask applied
    pub fn unsharp(&self, radius: u32, sigma: f32, factor: f32) -> Image {
        Self::new_from_buffer_with_meta(self, ops::unsharp(&self.buffer, radius, sigma, factor))
    }
}

#[cfg(test)]
mod tests {
    use super::Image;
    use crate::RGB;
    use d10_ops::FilterMode;

    fn test_image_3_2() -> Image {
        Image::new_from_raw(3, 2, vec![
            RGB::WHITE, RGB::BLACK, RGB::YELLOW,
            RGB::RED, RGB::GREEN, RGB::BLUE
        ])
    }

    fn test_image_4_2() -> Image {
        Image::new_from_raw(4, 2, vec![
            RGB::WHITE, RGB::BLACK, RGB::YELLOW, RGB::MAGENTA,
            RGB::RED, RGB::GREEN, RGB::BLUE, RGB::CYAN
        ])
    }

    #[test]
    fn flip_horizontal() {
        let img_in = test_image_3_2();

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
        let img_in = test_image_3_2();

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
        let img_in = test_image_3_2();

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
        let img_in = test_image_3_2();

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
        let img_in = test_image_3_2();

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

    #[test]
    fn rotate() {
        let img_in = test_image_4_2();

        let img_out = img_in.rotate(180.0, FilterMode::Nearest);

        assert_eq!(img_in.width(), img_out.width());
        assert_eq!(img_in.height(), img_out.height());

        assert_eq!(img_in.get_pixel(0, 0), img_out.get_pixel(3, 1));
        assert_eq!(img_in.get_pixel(1, 0), img_out.get_pixel(2, 1));
        assert_eq!(img_in.get_pixel(2, 0), img_out.get_pixel(1, 1));
        assert_eq!(img_in.get_pixel(3, 0), img_out.get_pixel(0, 1));

        assert_eq!(img_in.get_pixel(0, 1), img_out.get_pixel(3, 0));
        assert_eq!(img_in.get_pixel(1, 1), img_out.get_pixel(2, 0));
        assert_eq!(img_in.get_pixel(2, 1), img_out.get_pixel(1, 0));
        assert_eq!(img_in.get_pixel(3, 1), img_out.get_pixel(0, 0));
    }

    #[test]
    fn resize() {
        let img_in = test_image_3_2();

        let img_out = img_in.resize(30, 21, FilterMode::Nearest);
        assert_eq!(img_out.width(), 30);
        assert_eq!(img_out.height(), 21);

        let img_out = img_in.resize(30, 21, FilterMode::Bilinear);
        assert_eq!(img_out.width(), 30);
        assert_eq!(img_out.height(), 21);

        let img_out = img_in.resize(30, 21, FilterMode::Bicubic);
        assert_eq!(img_out.width(), 30);
        assert_eq!(img_out.height(), 21);
    }

    #[test]
    fn resize_pct() {
        let img_in = test_image_3_2();

        let img_out = img_in.resize_pct(200.0, FilterMode::Nearest);
        assert_eq!(img_out.width(), 3 * 2);
        assert_eq!(img_out.height(), 2 * 2);

        let img_out = img_in.resize_pct(33.3, FilterMode::Nearest);
        assert_eq!(img_out.width(), 1);
        assert_eq!(img_out.height(), 1);

        let img_out = img_in.resize_pct(1.0, FilterMode::Nearest);
        assert_eq!(img_out.width(), 1);
        assert_eq!(img_out.height(), 1);
    }

    #[test]
    fn with_jpeg_quality() {
        let img_in = test_image_3_2();

        let img_out = img_in.with_jpeg_quality(100, true).expect("New image");

        assert_eq!(img_in.width(), img_out.width());
        assert_eq!(img_in.height(), img_out.height());

        for (c1, c2) in img_in.data().iter().zip(img_out.data().iter()) {
            for i in 0..=2 {
                let diff = (c1.data[i] - c2.data[i]).abs();

                // Not exact but if it fails it's definitely not a quality of 100
                assert!(diff < 0.1)
            }
        }
    }

    #[test]
    fn random_noise() {
        //TODO:  Add real test that checks if there is actually a noise added

        let img_in = test_image_3_2();

        let img_out = img_in.random_noise(0.5);

        assert_eq!(img_in.width(), img_out.width());
        assert_eq!(img_in.height(), img_out.height());

        let mut img_in = test_image_3_2();

        img_in.add_random_noise(0.5);
    }

    #[test]
    fn salt_n_pepper_noise() {
        //TODO:  Add real test that checks if there is actually a noise added

        let img_in = test_image_3_2();

        let img_out = img_in.random_noise(0.5);

        assert_eq!(img_in.width(), img_out.width());
        assert_eq!(img_in.height(), img_out.height());

        let mut img_in = test_image_3_2();

        img_in.add_random_noise(0.5);
    }

    #[test]
    fn gaussian_noise() {
        //TODO:  Add real test that checks if there is actually a noise added

        let img_in = test_image_3_2();

        let img_out = img_in.gaussian_noise(0.5);

        assert_eq!(img_in.width(), img_out.width());
        assert_eq!(img_in.height(), img_out.height());

        let mut img_in = test_image_3_2();

        img_in.add_gaussian_noise(0.5);
    }

    #[test]
    fn gaussian_blur() {
        //TODO:  Add test if image is blurry

        let img_in = test_image_3_2();

        let img_out = img_in.gaussian_blur(1, 0.5);

        assert_eq!(img_in.width(), img_out.width());
        assert_eq!(img_in.height(), img_out.height());
    }

    #[test]
    fn unsharp() {
        //TODO:  Add test if image is sharpened

        let img_in = test_image_3_2();

        let img_out = img_in.unsharp(1, 0.5, 0.5);

        assert_eq!(img_in.width(), img_out.width());
        assert_eq!(img_in.height(), img_out.height());
    }

    #[test]
    fn test_crop() {
        let image: Image = Image::new(100, 200);

        let cropped = image.crop(0, 0, 10, 20);
        assert_eq!(cropped.width(), 10);
        assert_eq!(cropped.height(), 20);

        let cropped = image.crop(200, 0, 10, 20);
        assert_eq!(cropped.width(), 0);
        assert_eq!(cropped.height(), 0);

        let cropped = image.crop(50, 50, 100, 200);
        assert_eq!(cropped.width(), 50);
        assert_eq!(cropped.height(), 150);
    }
}