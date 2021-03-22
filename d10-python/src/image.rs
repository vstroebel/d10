use pyo3::prelude::*;

use crate::IntoPyErr;
use crate::color::RGB;

use d10::{Image as D10Image, RGB as D10RGB, EncodingFormat as D10EncodingFormat, PNGColorType, PNGFilterType, PNGCompression, BMPColorType, ICOColorType};
use pyo3::types::PyFunction;
use d10::ops::FilterMode;
use std::convert::TryInto;
use pyo3::PyMappingProtocol;
use pyo3::exceptions::PyOSError;

#[pyclass]
pub struct Image {
    pub inner: D10Image
}

#[pymethods]
impl Image {
    #[new]
    fn new(width: u32, height: u32, color: Option<&RGB>) -> Image {
        match color {
            Some(color) => D10Image::new_with_color(width, height, color.inner),
            None => D10Image::new(width, height)
        }.into()
    }

    #[staticmethod]
    fn from_list(width: u32, height: u32, list: Vec<RGB>) -> Image {
        Image {
            inner: D10Image::new_from_raw(
                width,
                height,
                list.into_iter()
                    .map(|c| c.inner)
                    .collect(),
            )
        }
    }

    fn to_list(&self) -> Vec<RGB> {
        self.inner.data().iter()
            .map(|c| c.into())
            .collect()
    }

    #[getter]
    fn get_width(&self) -> u32 {
        self.inner.width()
    }

    #[getter]
    fn get_height(&self) -> u32 {
        self.inner.height()
    }

    #[staticmethod]
    fn open(path: &str) -> PyResult<Image> {
        Ok(D10Image::open(path).py_err()?.into())
    }

    fn save(&mut self, path: &str, format: Option<&EncodingFormat>) -> PyResult<()> {
        match format {
            Some(format) => self.inner.save_with_format(path, format.inner.clone()).py_err()?,
            None => self.inner.save(path).py_err()?
        }
        Ok(())
    }

    pub fn has_transparency(&self) -> bool {
        self.inner.has_transparency()
    }

    pub fn is_grayscale(&self) -> bool {
        self.inner.is_grayscale()
    }

    fn mod_colors(&mut self, func: &PyFunction) -> PyResult<()> {
        let map = |c: &D10RGB| -> PyResult<D10RGB> {
            let arg1 = RGB { inner: *c };
            let r = func.call1((arg1, ))?;
            Ok(r.extract::<RGB>()?.inner)
        };

        self.inner.try_mod_colors(map)
    }

    fn mod_colors_enumerated(&mut self, func: &PyFunction) -> PyResult<()> {
        let map = |x: u32, y: u32, c: &D10RGB| -> PyResult<D10RGB> {
            let arg1 = x as i32;
            let arg2 = y as i32;
            let arg3 = RGB { inner: *c };

            let r = func.call1((arg1, arg2, arg3))?;
            Ok(r.extract::<RGB>()?.inner)
        };

        self.inner.try_mod_colors_enumerated(map)
    }

    fn map_colors(&self, func: &PyFunction) -> PyResult<Image> {
        let map = |c: &D10RGB| -> PyResult<D10RGB> {
            let arg1 = RGB { inner: *c };
            let r = func.call1((arg1, ))?;

            Ok(r.extract::<RGB>()?.inner)
        };

        Ok(self.inner.try_map_colors(map)?.into())
    }

    fn map_colors_enumerated(&self, func: &PyFunction) -> PyResult<Image> {
        let map = |x: u32, y: u32, c: &D10RGB| -> PyResult<D10RGB> {
            let arg1 = x as i32;
            let arg2 = y as i32;
            let arg3 = RGB { inner: *c };

            let r = func.call1((arg1, arg2, arg3))?;
            Ok(r.extract::<RGB>()?.inner)
        };

        Ok(self.inner.try_map_colors_enumerated(map)?.into())
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Option<RGB> {
        self.inner.get_pixel_optional(x, y).map(|c| c.into())
    }

    pub fn get_pixel_clamped(&self, x: i32, y: i32) -> RGB {
        self.inner.get_pixel_clamped(x, y).into()
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: &RGB) {
        self.inner.put_pixel(x, y, color.inner);
    }

    pub fn is_in_image(&self, x: i32, y: i32) -> bool {
        self.inner.is_in_image(x, y)
    }

    pub fn flip_horizontal(&self) -> Image {
        self.inner.flip_horizontal().into()
    }

    pub fn flip_vertical(&self) -> Image {
        self.inner.flip_vertical().into()
    }

    pub fn rotate90(&self) -> Image {
        self.inner.rotate90().into()
    }

    pub fn rotate180(&self) -> Image {
        self.inner.rotate180().into()
    }

    pub fn rotate270(&self) -> Image {
        self.inner.rotate270().into()
    }

    pub fn resize(&self, new_width: u32, new_height: u32, filter: Option<&str>) -> PyResult<Image> {
        let filter = match filter {
            Some(filter) => filter.try_into().py_err()?,
            None => FilterMode::Bilinear
        };
        Ok(self.inner.resize(new_width, new_height, filter).into())
    }

    pub fn resize_pct(&self, pct_100: f32, filter: Option<&str>) -> PyResult<Image> {
        let filter = match filter {
            Some(filter) => filter.try_into().py_err()?,
            None => FilterMode::Bilinear
        };
        Ok(self.inner.resize_pct(pct_100, filter).into())
    }

    pub fn sobel_edge_detection(&self, normalize: Option<bool>) -> Image {
        self.inner.sobel_edge_detection(normalize.unwrap_or(false)).into()
    }

    pub fn with_jpeg_quality(&self, quality: u8, preserve_alpha: Option<bool>) -> PyResult<Image> {
        Ok(self.inner.with_jpeg_quality(quality, preserve_alpha.unwrap_or(true)).py_err()?.into())
    }

    pub fn random_noise(&self, alpha: f32) -> Image {
        self.inner.random_noise(alpha).into()
    }

    pub fn add_random_noise(&mut self, alpha: f32) {
        self.inner.add_random_noise(alpha)
    }

    pub fn salt_n_pepper_noise(&self, threshold: f32) -> Image {
        self.inner.salt_n_pepper_noise(threshold).into()
    }

    pub fn add_salt_n_pepper_noise(&mut self, threshold: f32) {
        self.inner.add_salt_n_pepper_noise(threshold);
    }

    pub fn gaussian_noise(&self, alpha: f32) -> Image {
        self.inner.gaussian_noise(alpha).into()
    }

    pub fn add_gaussian_noise(&mut self, alpha: f32) {
        self.inner.add_gaussian_noise(alpha);
    }

    pub fn gaussian_blur(&self, radius: u32, sigma: Option<f32>) -> Image {
        self.inner.gaussian_blur(radius, sigma.unwrap_or(1.0)).into()
    }

    pub fn unsharp(&self, radius: u32, sigma: Option<f32>, factor: Option<f32>) -> Image {
        self.inner.unsharp(radius, sigma.unwrap_or(1.0), factor.unwrap_or(1.0)).into()
    }
}

impl From<D10Image> for Image {
    fn from(image: D10Image) -> Image {
        Image {
            inner: image
        }
    }
}

#[pyproto]
impl PyMappingProtocol for Image {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.inner.data().len())
    }

    fn __getitem__(&self, key: (i32, i32)) -> PyResult<RGB> {
        let x = key.0;
        let y = key.1;

        self.get_pixel(x, y).ok_or_else(|| {
            PyOSError::new_err(format!("Array not within range: {}x{}", y, x))
        })
    }

    fn __setitem__(&mut self, key: (u32, u32), value: RGB) {
        let x = key.0;
        let y = key.1;

        self.put_pixel(x, y, &value);
    }
}


#[pyclass]
pub struct EncodingFormat {
    pub inner: D10EncodingFormat
}

#[pymethods]
impl EncodingFormat {
    #[staticmethod]
    fn jpeg(quality: Option<u8>) -> EncodingFormat {
        EncodingFormat {
            inner: D10EncodingFormat::JPEG {
                quality: quality.unwrap_or(85)
            }
        }
    }

    #[staticmethod]
    fn png(color_type: Option<&str>, compression: Option<&str>, filter: Option<&str>) -> PyResult<EncodingFormat> {
        let color_type = match color_type {
            Some(v) => v.try_into().py_err()?,
            None => PNGColorType::RGBA8,
        };
        let compression = match compression {
            Some(v) => v.try_into().py_err()?,
            None => PNGCompression::Default
        };

        let filter = match filter {
            Some(v) => v.try_into().py_err()?,
            None => PNGFilterType::Sub,
        };

        Ok(EncodingFormat {
            inner: D10EncodingFormat::PNG {
                color_type,
                compression,
                filter,
            }
        })
    }

    #[staticmethod]
    fn gif() -> EncodingFormat {
        EncodingFormat {
            inner: D10EncodingFormat::GIF
        }
    }

    #[staticmethod]
    fn bmp(color_type: Option<&str>) -> PyResult<EncodingFormat> {
        let color_type = match color_type {
            Some(v) => v.try_into().py_err()?,
            None => BMPColorType::RGBA8,
        };

        Ok(EncodingFormat {
            inner: D10EncodingFormat::BMP {
                color_type
            }
        })
    }

    #[staticmethod]
    fn ico(color_type: Option<&str>) -> PyResult<EncodingFormat> {
        let color_type = match color_type {
            Some(v) => v.try_into().py_err()?,
            None => ICOColorType::RGBA8,
        };

        Ok(EncodingFormat {
            inner: D10EncodingFormat::ICO {
                color_type,
            }
        })
    }
}