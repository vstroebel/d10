use crate::color::*;
use crate::errors::{D10Result, D10Error};
use crate::kernel::Kernel;

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

fn validate_size(width: u32, height: u32) -> D10Result<()> {
    //TODO: Add build time configuration for this
    const MAX: u32 = i32::MAX as u32;
    if width >= MAX || height >= MAX || width >= MAX / height {
        Err(D10Error::Limits(format!("Image size exceeds limits: {}x{}", width, height)))
    } else {
        Ok(())
    }
}

impl<T: Color> PixelBuffer<T> {
    /// Creates a new buffer with the default color (i.e. transparent black for RGBA)
    ///
    /// # Warning
    ///
    /// As of now this will abort the application if there is an Out-Of-Memory error
    /// during buffer allocation.
    pub fn new(width: u32, height: u32) -> D10Result<PixelBuffer<T>> {
        Self::new_with_color(width, height, &T::default())
    }

    pub fn new_with_color(width: u32, height: u32, color: &T) -> D10Result<PixelBuffer<T>> {
        validate_size(width, height)?;

        Ok(PixelBuffer {
            width,
            height,
            data: vec![*color; (width * height) as usize],
        })
    }

    pub fn new_from_raw(width: u32, height: u32, data: Vec<T>) -> D10Result<PixelBuffer<T>> {
        validate_size(width, height)?;

        let required_len = width as usize * height as usize;

        if required_len != data.len() {
            Err(D10Error::BadArgument(format!("Data has wrong length: {}x{}={} data has {}", width, height, required_len, data.len())))
        } else {
            Ok(Self {
                width,
                height,
                data,
            })
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn enumerate(&self) -> impl Iterator<Item=(u32, u32, T)> + '_ {
        let width = self.width;

        self.data.iter()
            .enumerate()
            .map(move |(i, v)| (i as u32 % width, i as u32 / width, *v))
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item=(u32, u32, &mut T)> + '_ {
        let width = self.width;

        self.data.iter_mut()
            .enumerate()
            .map(move |(i, v)| (i as u32 % width, i as u32 / width, v))
    }

    pub fn mod_colors<F: FnMut(&T) -> T>(&mut self, mut func: F) {
        for pixel in self.data.iter_mut() {
            let new_color = func(pixel);

            *pixel = new_color;
        }
    }

    pub fn try_mod_colors<E, F: FnMut(&T) -> Result<T, E>>(&mut self, mut func: F) -> Result<(), E> {
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

    pub fn try_mod_colors_enumerated<E, F: Fn(u32, u32, &T) -> Result<T, E>>(&mut self, func: F) -> Result<(), E> {
        for (x, y, pixel) in self.enumerate_mut() {
            let new_color = func(x, y, pixel)?;

            *pixel = new_color;
        }

        Ok(())
    }

    pub fn map_colors<F: FnMut(&T) -> T>(&self, func: F) -> PixelBuffer<T> {
        let data = self.data.iter().map(func).collect();
        PixelBuffer {
            width: self.width,
            height: self.height,
            data,
        }
    }

    pub fn try_map_colors<E, F: FnMut(&T) -> Result<T, E>>(&self, func: F) -> Result<PixelBuffer<T>, E> {
        let data = self.data.iter().map(func).collect::<Result<Vec<T>, E>>()?;
        Ok(PixelBuffer {
            width: self.width,
            height: self.height,
            data,
        })
    }

    pub fn map_colors_enumerated<F: Fn(u32, u32, &T) -> T>(&self, func: F) -> PixelBuffer<T> {
        let data = self.enumerate()
            .map(|(x, y, c)| func(x, y, &c))
            .collect();
        PixelBuffer {
            width: self.width,
            height: self.height,
            data,
        }
    }

    pub fn try_map_colors_enumerated<E, F: Fn(u32, u32, &T) -> Result<T, E>>(&self, func: F) -> Result<PixelBuffer<T>, E> {
        let data = self.enumerate()
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

    pub fn set_pixel(&mut self, x: u32, y: u32, color: &T) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.data[(x + y * self.width) as usize] = *color;
    }

    pub fn get_pixel_clamped(&self, x: i32, y: i32) -> &T {
        self.get_pixel(x.max(0).min((self.width - 1) as i32) as u32,
                       y.max(0).min((self.height - 1) as i32) as u32,
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

    pub fn get_kernel_3x3(&self, x: i32, y: i32) -> [[&T; 3]; 3] {
        [
            [
                self.get_pixel_clamped(x - 1, y - 1),
                self.get_pixel_clamped(x, y - 1),
                self.get_pixel_clamped(x + 1, y - 1)
            ],
            [
                self.get_pixel_clamped(x - 1, y),
                self.get_pixel_clamped(x, y),
                self.get_pixel_clamped(x + 1, y)
            ],
            [
                self.get_pixel_clamped(x - 1, y + 1),
                self.get_pixel_clamped(x, y + 1),
                self.get_pixel_clamped(x + 1, y + 1)
            ]
        ]
    }

    pub fn get_kernel_5x5(&self, x: i32, y: i32) -> [[&T; 5]; 5] {
        [
            [
                self.get_pixel_clamped(x - 2, y - 2),
                self.get_pixel_clamped(x - 1, y - 2),
                self.get_pixel_clamped(x, y - 2),
                self.get_pixel_clamped(x + 1, y - 2),
                self.get_pixel_clamped(x + 2, y - 2),
            ],
            [
                self.get_pixel_clamped(x - 2, y - 1),
                self.get_pixel_clamped(x - 1, y - 1),
                self.get_pixel_clamped(x, y - 1),
                self.get_pixel_clamped(x + 1, y - 1),
                self.get_pixel_clamped(x + 2, y - 1),
            ],
            [
                self.get_pixel_clamped(x - 2, y),
                self.get_pixel_clamped(x - 1, y),
                self.get_pixel_clamped(x, y - 1),
                self.get_pixel_clamped(x + 1, y),
                self.get_pixel_clamped(x + 2, y),
            ],
            [
                self.get_pixel_clamped(x - 2, y + 1),
                self.get_pixel_clamped(x - 1, y + 1),
                self.get_pixel_clamped(x, y + 1),
                self.get_pixel_clamped(x + 1, y + 1),
                self.get_pixel_clamped(x + 2, y + 1),
            ],
            [
                self.get_pixel_clamped(x - 2, y + 2),
                self.get_pixel_clamped(x - 1, y + 2),
                self.get_pixel_clamped(x, y - 1),
                self.get_pixel_clamped(x + 1, y + 2),
                self.get_pixel_clamped(x + 2, y + 2),
            ]]
    }

    pub fn has_transparency(&self) -> bool {
        self.data.iter().any(Color::has_transparency)
    }

    pub fn to_rgb(&self) -> PixelBuffer<RGB> {
        PixelBuffer {
            width: self.width,
            height: self.height,
            data: self.data.iter().into_rgb().collect(),
        }
    }

    pub fn to_srgb(&self) -> PixelBuffer<SRGB> {
        PixelBuffer {
            width: self.width,
            height: self.height,
            data: self.data.iter().into_srgb().collect(),
        }
    }

    pub fn to_hsl(&self) -> PixelBuffer<HSL> {
        PixelBuffer {
            width: self.width,
            height: self.height,
            data: self.data.iter().into_hsl().collect(),
        }
    }

    pub fn to_hsv(&self) -> PixelBuffer<HSV> {
        PixelBuffer {
            width: self.width,
            height: self.height,
            data: self.data.iter().into_hsv().collect(),
        }
    }

    pub fn to_yuv(&self) -> PixelBuffer<YUV> {
        PixelBuffer {
            width: self.width,
            height: self.height,
            data: self.data.iter().into_yuv().collect(),
        }
    }
}

impl PixelBuffer<RGB> {
    pub fn is_grayscale(&self) -> bool {
        self.data.iter().any(RGB::is_grayscale)
    }

    pub fn apply_kernel(&self, kernel: &Kernel) -> PixelBuffer<RGB> {
        self.map_colors_enumerated(|x, y, _| self.get_kernel_value(x, y, kernel))
    }

    pub fn get_kernel_value(&self, image_x: u32, image_y: u32, kernel: &Kernel) -> RGB {
        let offset_x = kernel.get_offset_x();
        let offset_y = kernel.get_offset_y();

        let mut data = [0.0; 4];

        for (x, y, kernel_value) in kernel.enumerate() {
            let color = self.get_pixel_clamped((image_x + x) as i32 - offset_x, (image_y + y) as i32 - offset_y);

            for (value, color_value) in data.iter_mut().zip(color.data.iter()) {
                *value += color_value * kernel_value;
            }
        }

        RGB {
            data
        }
    }
}
