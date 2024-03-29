use crate::color::*;
use crate::kernel::Kernel;
use crate::kernel_dyn::KernelDyn;
use std::any::type_name;
use std::fmt::{Debug, Formatter};

pub const MAX_BUFFER_SIZE: u64 = (i32::MAX as u64) / 2;

pub fn is_valid_buffer_size(width: u32, height: u32) -> bool {
    if width == 0 {
        height == 0
    } else if height == 0 {
        width == 0
    } else {
        (width as u64) * (height as u64) <= MAX_BUFFER_SIZE
    }
}

fn validate_size(width: u32, height: u32) {
    if !is_valid_buffer_size(width, height) {
        panic!("Invalid buffer size: {}x{}", width, height)
    }
}

/// A storage for raw image data
///
///
///
#[derive(Clone)]
pub struct PixelBuffer<T: Color> {
    width: u32,
    height: u32,
    data: Vec<T>,
}

impl<T: Color> PixelBuffer<T> {
    /// Creates a new buffer with the default color (i.e. transparent black for RGBA)
    ///
    /// # Panics
    ///
    /// Creating the buffer panics if the number of Pixels exceeds MAX_BUFFER_SIZE
    pub fn new(width: u32, height: u32) -> PixelBuffer<T> {
        Self::new_with_color(width, height, T::default())
    }

    pub fn new_with_color(width: u32, height: u32, color: T) -> PixelBuffer<T> {
        validate_size(width, height);

        PixelBuffer {
            width,
            height,
            data: vec![color; (width * height) as usize],
        }
    }

    pub fn new_from_raw(width: u32, height: u32, data: Vec<T>) -> PixelBuffer<T> {
        let required_len = width as u64 * height as u64;

        if required_len > usize::MAX as u64 || required_len as usize != data.len() {
            panic!(
                "Data has wrong length: {}x{}={} data has {}",
                width,
                height,
                required_len,
                data.len()
            )
        } else {
            validate_size(width, height);

            Self {
                width,
                height,
                data,
            }
        }
    }

    pub fn try_new_from_func<E, F>(
        width: u32,
        height: u32,
        mut func: F,
    ) -> Result<PixelBuffer<T>, E>
    where
        F: FnMut(u32, u32) -> Result<T, E>,
    {
        validate_size(width, height);

        let data: Result<Vec<T>, E> = (0..(width * height))
            .map(|i| func(i % width, i / width))
            .collect();

        data.map(|data| Self {
            width,
            height,
            data,
        })
    }

    pub fn new_from_func<F>(width: u32, height: u32, mut func: F) -> PixelBuffer<T>
    where
        F: FnMut(u32, u32) -> T,
    {
        validate_size(width, height);

        let data = (0..(width * height))
            .map(|i| func(i % width, i / width))
            .collect();

        Self {
            width,
            height,
            data,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 && self.height == 0
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (u32, u32, T)> + '_ {
        let width = self.width;

        self.data
            .iter()
            .enumerate()
            .map(move |(i, v)| (i as u32 % width, i as u32 / width, *v))
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (u32, u32, &mut T)> + '_ {
        let width = self.width;

        self.data
            .iter_mut()
            .enumerate()
            .map(move |(i, v)| (i as u32 % width, i as u32 / width, v))
    }

    pub fn mod_colors<F: FnMut(&T) -> T>(&mut self, mut func: F) {
        for pixel in self.data.iter_mut() {
            let new_color = func(pixel);

            *pixel = new_color;
        }
    }

    pub fn try_mod_colors<E, F: FnMut(&T) -> Result<T, E>>(
        &mut self,
        mut func: F,
    ) -> Result<(), E> {
        for pixel in self.data.iter_mut() {
            let new_color = func(pixel)?;

            *pixel = new_color;
        }

        Ok(())
    }

    pub fn mod_colors_enumerated<F: Fn(u32, u32, &T) -> T>(&mut self, func: F) {
        for (x, y, pixel) in self.enumerate_mut() {
            let new_color = func(x, y, pixel);

            *pixel = new_color;
        }
    }

    pub fn try_mod_colors_enumerated<E, F: Fn(u32, u32, &T) -> Result<T, E>>(
        &mut self,
        func: F,
    ) -> Result<(), E> {
        for (x, y, pixel) in self.enumerate_mut() {
            let new_color = func(x, y, pixel)?;

            *pixel = new_color;
        }

        Ok(())
    }

    pub fn map_colors<F: FnMut(&T) -> R, R: Color>(&self, func: F) -> PixelBuffer<R> {
        let data = self.data.iter().map(func).collect();
        PixelBuffer {
            width: self.width,
            height: self.height,
            data,
        }
    }

    pub fn try_map_colors<E, F: FnMut(&T) -> Result<R, E>, R: Color>(
        &self,
        func: F,
    ) -> Result<PixelBuffer<R>, E> {
        let data = self.data.iter().map(func).collect::<Result<Vec<R>, E>>()?;
        Ok(PixelBuffer {
            width: self.width,
            height: self.height,
            data,
        })
    }

    pub fn map_colors_enumerated<F: Fn(u32, u32, &T) -> T>(&self, func: F) -> PixelBuffer<T> {
        let data = self.enumerate().map(|(x, y, c)| func(x, y, &c)).collect();
        PixelBuffer {
            width: self.width,
            height: self.height,
            data,
        }
    }

    pub fn try_map_colors_enumerated<E, F: Fn(u32, u32, &T) -> Result<T, E>>(
        &self,
        func: F,
    ) -> Result<PixelBuffer<T>, E> {
        let data = self
            .enumerate()
            .map(|(x, y, c)| func(x, y, &c))
            .collect::<Result<Vec<T>, E>>()?;
        Ok(PixelBuffer {
            width: self.width,
            height: self.height,
            data,
        })
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> &T {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.data[(x + y * self.width) as usize]
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: T) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.data[(x + y * self.width) as usize] = color;
    }

    pub fn get_pixel_clamped(&self, x: i32, y: i32) -> &T {
        self.get_pixel(
            x.clamp(0, (self.width - 1) as i32) as u32,
            y.clamp(0, (self.height - 1) as i32) as u32,
        )
    }

    pub fn get_pixel_optional(&self, x: i32, y: i32) -> Option<&T> {
        if self.is_in_image(x, y) {
            Some(&self.data[(x as u32 + y as u32 * self.width) as usize])
        } else {
            None
        }
    }

    /// Check whether the x/y coordinates are within the buffer dimensions
    pub fn is_in_image(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    pub fn get_kernel<const N: usize>(&self, x: i32, y: i32) -> [[T; N]; N] {
        let mut values = [[T::default(); N]; N];

        let offset = N as i32 / 2;

        let start_x = x - offset;
        let start_y = y - offset;

        if self.is_in_image(start_x, start_y) && self.is_in_image(x + offset, y + offset) {
            // Fast path that can be used if we know the kernel is always inside the image

            for (ty, row) in values.iter_mut().enumerate() {
                let start = self.width as usize * (ty + start_y as usize) + start_x as usize;
                let end = start + N;
                (*row).copy_from_slice(&self.data[start..end]);
            }
        } else {
            for (ty, row) in values.iter_mut().enumerate() {
                for (tx, cell) in row.iter_mut().enumerate() {
                    *cell = *self.get_pixel_clamped(tx as i32 + x - offset, ty as i32 + y - offset)
                }
            }
        }

        values
    }

    pub fn get_kernel_dyn(&self, x: i32, y: i32, size: usize) -> Vec<Vec<T>> {

        let mut values = vec![vec![T::default(); size]; size];

        let offset = size as i32 / 2;

        let start_x = x - offset;
        let start_y = y - offset;

        if self.is_in_image(start_x, start_y) && self.is_in_image(x + offset, y + offset) {
            // Fast path that can be used if we know the kernel is always inside the image

            for (ty, row) in values.iter_mut().enumerate() {
                let start = self.width as usize * (ty + start_y as usize) + start_x as usize;
                let end = start + size;
                (*row).copy_from_slice(&self.data[start..end]);
            }
        } else {
            for (ty, row) in values.iter_mut().enumerate() {
                for (tx, cell) in row.iter_mut().enumerate() {
                    *cell = *self.get_pixel_clamped(tx as i32 + x - offset, ty as i32 + y - offset)
                }
            }
        }

        values
    }

    pub fn has_transparency(&self) -> bool {
        self.data.iter().any(Color::has_transparency)
    }

    pub fn to_rgb(&self) -> PixelBuffer<Rgb> {
        PixelBuffer {
            width: self.width,
            height: self.height,
            data: self.data.iter().into_rgb().collect(),
        }
    }

    pub fn to_srgb(&self) -> PixelBuffer<Srgb> {
        PixelBuffer {
            width: self.width,
            height: self.height,
            data: self.data.iter().into_srgb().collect(),
        }
    }

    pub fn to_hsl(&self) -> PixelBuffer<Hsl> {
        PixelBuffer {
            width: self.width,
            height: self.height,
            data: self.data.iter().into_hsl().collect(),
        }
    }

    pub fn to_hsv(&self) -> PixelBuffer<Hsv> {
        PixelBuffer {
            width: self.width,
            height: self.height,
            data: self.data.iter().into_hsv().collect(),
        }
    }

    pub fn to_yuv(&self) -> PixelBuffer<Yuv> {
        PixelBuffer {
            width: self.width,
            height: self.height,
            data: self.data.iter().into_yuv().collect(),
        }
    }

    pub fn to_xyz(&self) -> PixelBuffer<Xyz> {
        PixelBuffer {
            width: self.width,
            height: self.height,
            data: self.data.iter().into_xyz().collect(),
        }
    }

    pub fn to_lab<I: Illuminant, O: Observer>(&self) -> PixelBuffer<Lab<I, O>> {
        PixelBuffer {
            width: self.width,
            height: self.height,
            data: self.data.iter().into_lab().collect(),
        }
    }

    pub fn to_lch<I: Illuminant, O: Observer>(&self) -> PixelBuffer<Lch<I, O>> {
        PixelBuffer {
            width: self.width,
            height: self.height,
            data: self.data.iter().into_lch().collect(),
        }
    }
}

impl PixelBuffer<Rgb> {
    pub fn is_grayscale(&self) -> bool {
        self.data.iter().all(Rgb::is_grayscale)
    }

    pub fn apply_kernel<const N: usize>(&self, kernel: &Kernel<N>) -> PixelBuffer<Rgb> {
        self.map_colors_enumerated(|x, y, _| {
            let buffer_k = self.get_kernel::<N>(x as i32, y as i32);

            Rgb {
                data: kernel.apply_kernel(&buffer_k, |c, i| c.data[i]),
            }
        })
    }

    pub fn apply_kernel_dyn(&self, kernel: &KernelDyn) -> PixelBuffer<Rgb> {
        self.map_colors_enumerated(|x, y, _| self.get_kernel_value(x, y, kernel))
    }

    pub fn get_kernel_value(&self, image_x: u32, image_y: u32, kernel: &KernelDyn) -> Rgb {
        let offset_x = kernel.get_offset_x();
        let offset_y = kernel.get_offset_y();

        let mut data = [0.0; 4];

        for (x, y, kernel_value) in kernel.enumerate() {
            let color = self.get_pixel_clamped(
                (image_x + x) as i32 - offset_x,
                (image_y + y) as i32 - offset_y,
            );

            for (value, color_value) in data.iter_mut().zip(color.data.iter()) {
                *value += color_value * kernel_value;
            }
        }

        Rgb { data }
    }
}

impl<C: Color> Debug for PixelBuffer<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PixelBuffer({}, {}x{})",
            type_name::<C>(),
            self.width,
            self.height
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Rgb;
    use crate::pixelbuffer::PixelBuffer;

    #[test]
    fn new() {
        let buffer: PixelBuffer<Rgb> = PixelBuffer::new(13, 7);

        assert_eq!(buffer.width(), 13);
        assert_eq!(buffer.height(), 7);

        for c in buffer.data() {
            assert_eq!(*c, Rgb::default());
        }
    }

    #[test]
    fn new_with_color() {
        let buffer: PixelBuffer<Rgb> = PixelBuffer::new_with_color(7, 13, Rgb::RED);

        assert_eq!(buffer.width(), 7);
        assert_eq!(buffer.height(), 13);

        for c in buffer.data() {
            assert_eq!(*c, Rgb::RED);
        }
    }

    #[test]
    fn new_from_raw() {
        let raw = vec![Rgb::BLUE; 7 * 13];

        let buffer: PixelBuffer<Rgb> = PixelBuffer::new_from_raw(7, 13, raw);

        assert_eq!(buffer.width(), 7);
        assert_eq!(buffer.height(), 13);

        for c in buffer.data() {
            assert_eq!(*c, Rgb::BLUE);
        }
    }

    #[test]
    fn data() {
        let buffer: PixelBuffer<Rgb> = PixelBuffer::new_with_color(7, 13, Rgb::RED);
        let data = buffer.data();

        assert_eq!(data.len(), 13 * 7);

        for c in data {
            assert_eq!(*c, Rgb::RED);
        }
    }

    #[test]
    fn data_mut() {
        let mut buffer: PixelBuffer<Rgb> = PixelBuffer::new_with_color(13, 7, Rgb::RED);

        assert_eq!(buffer.data_mut().len(), 7 * 13);

        for c in buffer.data_mut() {
            assert_eq!(*c, Rgb::RED);
        }

        for c in buffer.data_mut() {
            *c = Rgb::BLUE;
        }

        for c in buffer.data_mut() {
            assert_eq!(*c, Rgb::BLUE);
        }
    }

    #[test]
    fn enumerate() {
        let buffer: PixelBuffer<Rgb> = PixelBuffer::new_with_color(13, 7, Rgb::RED);

        let mut i = 0u32;

        for (x, y, c) in buffer.enumerate() {
            let test_x = i % 13;
            let test_y = i / 13;

            assert_eq!(
                x, test_x,
                "Expected x value of {} got {} at index {}",
                test_x, x, i
            );
            assert_eq!(
                y, test_y,
                "Expected y value of {} got {} at index {}",
                test_y, y, i
            );
            assert!(i < 13 * 7);
            assert_eq!(c, Rgb::RED);

            i = i + 1;
        }
    }

    #[test]
    fn enumerate_mut() {
        let mut buffer: PixelBuffer<Rgb> = PixelBuffer::new_with_color(32, 64, Rgb::RED);

        let mut i = 0u32;

        for (x, y, c) in buffer.enumerate_mut() {
            let test_x = i % 32;
            let test_y = i / 32;

            assert_eq!(
                x, test_x,
                "Expected x value of {} got {} at index {}",
                test_x, x, i
            );
            assert_eq!(
                y, test_y,
                "Expected y value of {} got {} at index {}",
                test_y, y, i
            );
            assert!(i < 32 * 64);
            assert_eq!(*c, Rgb::RED);

            i = i + 1;

            *c = Rgb::new(1.0 / (x as f32), 1.0 / (y as f32), 1.0);
        }

        let mut i = 0u32;

        for (x, y, c) in buffer.enumerate() {
            let test_x = i % 32;
            let test_y = i / 32;

            assert_eq!(
                x, test_x,
                "Expected x value of {} got {} at index {}",
                test_x, x, i
            );
            assert_eq!(
                y, test_y,
                "Expected y value of {} got {} at index {}",
                test_y, y, i
            );
            assert!(i < 32 * 64);
            assert_eq!(c, Rgb::new(1.0 / (x as f32), 1.0 / (y as f32), 1.0));

            i = i + 1;
        }
    }

    #[test]
    fn test_is_grayscale() {
        let mut buffer = PixelBuffer::new_with_color(13, 7, Rgb::new(0.5, 0.5, 0.5));

        assert!(buffer.is_grayscale());
        buffer.put_pixel(0, 0, Rgb::new(1.0, 0.5, 0.5));
        assert!(!buffer.is_grayscale());
    }

    #[test]
    fn test_new_from_func() {
        let buffer =
            PixelBuffer::new_from_func(2, 3, |x, y| Rgb::new(x as f32 / 4.0, y as f32 / 4.0, 1.0));

        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 3);

        assert_eq!(buffer.get_pixel(0, 0), &Rgb::new(0.0 / 4.0, 0.0 / 4.0, 1.0));
        assert_eq!(buffer.get_pixel(1, 0), &Rgb::new(1.0 / 4.0, 0.0 / 4.0, 1.0));
        assert_eq!(buffer.get_pixel(0, 1), &Rgb::new(0.0 / 4.0, 1.0 / 4.0, 1.0));
        assert_eq!(buffer.get_pixel(1, 1), &Rgb::new(1.0 / 4.0, 1.0 / 4.0, 1.0));
        assert_eq!(buffer.get_pixel(0, 2), &Rgb::new(0.0 / 4.0, 2.0 / 4.0, 1.0));
        assert_eq!(buffer.get_pixel(1, 2), &Rgb::new(1.0 / 4.0, 2.0 / 4.0, 1.0));
    }

    #[test]
    fn test_try_new_from_func() {
        let buffer = PixelBuffer::try_new_from_func::<(), _>(2, 3, |x, y| {
            Ok(Rgb::new(x as f32 / 4.0, y as f32 / 4.0, 1.0))
        })
        .unwrap();

        assert_eq!(buffer.width, 2);
        assert_eq!(buffer.height, 3);

        assert_eq!(buffer.get_pixel(0, 0), &Rgb::new(0.0 / 4.0, 0.0 / 4.0, 1.0));
        assert_eq!(buffer.get_pixel(1, 0), &Rgb::new(1.0 / 4.0, 0.0 / 4.0, 1.0));
        assert_eq!(buffer.get_pixel(0, 1), &Rgb::new(0.0 / 4.0, 1.0 / 4.0, 1.0));
        assert_eq!(buffer.get_pixel(1, 1), &Rgb::new(1.0 / 4.0, 1.0 / 4.0, 1.0));
        assert_eq!(buffer.get_pixel(0, 2), &Rgb::new(0.0 / 4.0, 2.0 / 4.0, 1.0));
        assert_eq!(buffer.get_pixel(1, 2), &Rgb::new(1.0 / 4.0, 2.0 / 4.0, 1.0));

        let res =
            PixelBuffer::try_new_from_func(
                2,
                3,
                |x, _y| {
                    if x == 0 {
                        Ok(Rgb::WHITE)
                    } else {
                        Err(())
                    }
                },
            );

        assert!(res.is_err());
    }
}
