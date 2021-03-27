use pyo3::prelude::*;
use pyo3::types::PyFunction;
use pyo3::PyMappingProtocol;
use pyo3::exceptions::PyOSError;

use crate::IntoPyErr;
use crate::color::Rgb;

use d10::{Image as D10Image,
          Rgb as D10Rgb,
          EncodingFormat as D10EncodingFormat,
          PngColorType, PngFilterType,
          PngCompression,
          BmpColorType,
          IcoColorType,
          FilterMode};

#[cfg(feature = "numpy")]
use {
    numpy_helper::*,
    numpy::{PyArray, DataType},
    d10::{Color,
          Srgb as D10Srgb,
          Hsl as D10Hsl,
          Hsv as D10Hsv,
          Yuv as D10Yuv},
};

#[pyclass]
pub struct Image {
    pub inner: D10Image
}

#[pymethods]
impl Image {
    #[new]
    fn new(width: u32, height: u32, color: Option<&Rgb>) -> Image {
        match color {
            Some(color) => D10Image::new_with_color(width, height, color.inner),
            None => D10Image::new(width, height)
        }.into()
    }

    #[staticmethod]
    fn from_list(width: u32, height: u32, list: Vec<Rgb>) -> Image {
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

    fn to_list(&self) -> Vec<Rgb> {
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
        let map = |c: &D10Rgb| -> PyResult<D10Rgb> {
            let arg1 = Rgb { inner: *c };
            let r = func.call1((arg1, ))?;
            Ok(r.extract::<Rgb>()?.inner)
        };

        self.inner.try_mod_colors(map)
    }

    fn mod_colors_enumerated(&mut self, func: &PyFunction) -> PyResult<()> {
        let map = |x: u32, y: u32, c: &D10Rgb| -> PyResult<D10Rgb> {
            let arg1 = x as i32;
            let arg2 = y as i32;
            let arg3 = Rgb { inner: *c };

            let r = func.call1((arg1, arg2, arg3))?;
            Ok(r.extract::<Rgb>()?.inner)
        };

        self.inner.try_mod_colors_enumerated(map)
    }

    fn map_colors(&self, func: &PyFunction) -> PyResult<Image> {
        let map = |c: &D10Rgb| -> PyResult<D10Rgb> {
            let arg1 = Rgb { inner: *c };
            let r = func.call1((arg1, ))?;

            Ok(r.extract::<Rgb>()?.inner)
        };

        Ok(self.inner.try_map_colors(map)?.into())
    }

    fn map_colors_enumerated(&self, func: &PyFunction) -> PyResult<Image> {
        let map = |x: u32, y: u32, c: &D10Rgb| -> PyResult<D10Rgb> {
            let arg1 = x as i32;
            let arg2 = y as i32;
            let arg3 = Rgb { inner: *c };

            let r = func.call1((arg1, arg2, arg3))?;
            Ok(r.extract::<Rgb>()?.inner)
        };

        Ok(self.inner.try_map_colors_enumerated(map)?.into())
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Option<Rgb> {
        self.inner.get_pixel_optional(x, y).map(|c| c.into())
    }

    pub fn get_pixel_clamped(&self, x: i32, y: i32) -> Rgb {
        self.inner.get_pixel_clamped(x, y).into()
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: &Rgb) {
        self.inner.put_pixel(x, y, color.inner);
    }

    pub fn is_in_image(&self, x: i32, y: i32) -> bool {
        self.inner.is_in_image(x, y)
    }

    pub fn crop(&self, offset_x: u32, offset_y: u32, width: u32, height: u32) -> Image {
        self.inner.crop(offset_x, offset_y, width, height).into()
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

    pub fn rotate(&self, radians: f32, filter: Option<&str>) -> PyResult<Image> {
        let filter = match filter {
            Some(filter) => filter.parse().py_err()?,
            None => FilterMode::Bilinear
        };
        Ok(self.inner.rotate(radians, filter).into())
    }

    pub fn resize(&self, new_width: u32, new_height: u32, filter: Option<&str>) -> PyResult<Image> {
        let filter = match filter {
            Some(filter) => filter.parse().py_err()?,
            None => FilterMode::Bilinear
        };
        Ok(self.inner.resize(new_width, new_height, filter).into())
    }

    pub fn resize_pct(&self, pct_100: f32, filter: Option<&str>) -> PyResult<Image> {
        let filter = match filter {
            Some(filter) => filter.parse().py_err()?,
            None => FilterMode::Bilinear
        };
        Ok(self.inner.resize_pct(pct_100, filter).into())
    }

    pub fn sobel_edge_detection(&self, normalize: Option<bool>) -> Image {
        self.inner.sobel_edge_detection(normalize.unwrap_or(false)).into()
    }

    pub fn with_jpeg_quality(&self, quality: u8, preserve_alpha: Option<bool>) -> Image {
        self.inner.with_jpeg_quality(quality, preserve_alpha.unwrap_or(true)).into()
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


    #[cfg(feature = "numpy")]
    fn to_np_array(&self, py: Python, colorspace: Option<&str>, data_type: Option<&PyAny>) -> PyResult<Py<PyAny>> {
        let data_type = numpy_helper::extract_data_type(data_type)?;

        let colorspace = colorspace.unwrap_or("rgba");

        let (data, depth): (Vec<f32>, usize) = match colorspace {
            "hsl" => (self.inner.buffer().data()
                          .iter()
                          .flat_map(|c| color_iter::<3>(&c.to_hsl().data))
                          .collect(), 3),
            "hsla" => (self.inner.buffer().data()
                           .iter()
                           .flat_map(|c| color_iter::<4>(&c.to_hsl().data))
                           .collect(), 4),
            "hsv" => (self.inner.buffer().data()
                          .iter()
                          .flat_map(|c| color_iter::<3>(&c.to_hsv().data))
                          .collect(), 3),
            "hsva" => (self.inner.buffer().data()
                           .iter()
                           .flat_map(|c| color_iter::<4>(&c.to_hsv().data))
                           .collect(), 4),
            "yuv" => (self.inner.buffer().data()
                          .iter()
                          .flat_map(|c| color_iter::<3>(&c.to_yuv().data))
                          .collect(), 3),
            "yuva" => (self.inner.buffer().data()
                           .iter()
                           .flat_map(|c| color_iter::<4>(&c.to_yuv().data))
                           .collect(), 4),
            "rgb" => (self.inner.buffer().data()
                          .iter()
                          .flat_map(|c| c.data[0..=2].iter())
                          .copied()
                          .collect(), 3),
            "rgba" => (self.inner.buffer().data()
                           .iter()
                           .flat_map(|c| c.data.iter())
                           .copied()
                           .collect(), 4),
            "srgb" => (self.inner.buffer().data()
                           .iter()
                           .flat_map(|c| color_iter::<3>(&c.to_srgb().data))
                           .collect(), 3),
            "srgba" => (self.inner.buffer().data()
                            .iter()
                            .flat_map(|c| color_iter::<4>(&c.to_srgb().data))
                            .collect(), 4),
            "gray" => (self.inner.buffer().data()
                           .iter()
                           .map(|c| c.to_gray().red())
                           .collect(), 1),
            _ => return Err(PyOSError::new_err(format!("Unknown colorspace: {}", colorspace)))
        };

        Ok(match data_type {
            DataType::Float32 => PyArray::from_vec(py, data).reshape([self.inner.height() as usize, self.inner.width() as usize, depth])?.into(),
            DataType::Float64 => {
                let arr = PyArray::from_iter(py, data.iter().map(|v| *v as f64));
                arr.reshape([self.inner.height() as usize, self.inner.width() as usize, depth])?.into()
            }
            DataType::Uint8 => {
                let arr = PyArray::from_iter(py, data.iter().map(|v| (v * 255.0) as u8));
                arr.reshape([self.inner.height() as usize, self.inner.width() as usize, depth])?.into()
            }

            DataType::Uint16 => {
                let arr = PyArray::from_iter(py, data.iter().map(|v| (v * 65535.0) as u16));
                arr.reshape([self.inner.height() as usize, self.inner.width() as usize, depth])?.into()
            }

            DataType::Uint32 => {
                let arr = PyArray::from_iter(py, data.iter().map(|v| (v * 4294967295.0) as u32));
                arr.reshape([self.inner.height() as usize, self.inner.width() as usize, depth])?.into()
            }
            DataType::Bool => {
                let arr = PyArray::from_iter(py, data.iter().map(|v| *v >= 0.5));
                arr.reshape([self.inner.height() as usize, self.inner.width() as usize, depth])?.into()
            }
            _ => return Err(PyOSError::new_err(format!("Unsupported data type: {:?}", data_type))),
        })
    }

    #[cfg(feature = "numpy")]
    #[staticmethod]
    pub fn from_np_array(array: &PyAny, colorspace: Option<&str>) -> PyResult<Image> {
        let (ndims, dims, mut iter) = numpy_helper::into_f32_array(array)?;

        let width = dims[1];
        let height = dims[0];

        let colorspace = colorspace.unwrap_or("auto");

        let data: Vec<D10Rgb> = if ndims == 3 {
            if dims[2] == 4 {
                match colorspace {
                    "rgba" | "rgb" | "auto" => chunked::<4>(&mut iter)
                        .into_iter()
                        .map(|chunk| D10Rgb::new_with_alpha(chunk[0], chunk[1], chunk[2], chunk[3]))
                        .collect(),
                    "srgba" | "srgb" => chunked::<4>(&mut iter)
                        .into_iter()
                        .map(|chunk| D10Srgb::new_with_alpha(chunk[0], chunk[1], chunk[2], chunk[3]).to_rgb())
                        .collect(),
                    "hsla" | "hsl" => chunked::<4>(&mut iter)
                        .into_iter()
                        .map(|chunk| D10Hsl::new_with_alpha(chunk[0], chunk[1], chunk[2], chunk[3]).to_rgb())
                        .collect(),
                    "hsva" | "hsv" => chunked::<4>(&mut iter)
                        .into_iter()
                        .map(|chunk| D10Hsv::new_with_alpha(chunk[0], chunk[1], chunk[2], chunk[3]).to_rgb())
                        .collect(),
                    "yuva" | "yuv" => chunked::<4>(&mut iter)
                        .into_iter()
                        .map(|chunk| D10Yuv::new_with_alpha(chunk[0], chunk[1], chunk[2], chunk[3]).to_rgb())
                        .collect(),
                    _ => return Err(PyOSError::new_err(format!("Bad colorspace {} for dimensions: {}", colorspace, ndims)))
                }
            } else if dims[2] == 3 {
                match colorspace {
                    "rgb" | "auto" => chunked::<3>(&mut iter)
                        .into_iter()
                        .map(|chunk| D10Rgb::new(chunk[0], chunk[1], chunk[2]))
                        .collect(),
                    "srgb" => chunked::<3>(&mut iter)
                        .into_iter()
                        .map(|chunk| D10Srgb::new(chunk[0], chunk[1], chunk[2]).to_rgb())
                        .collect(),
                    "hsl" => chunked::<3>(&mut iter)
                        .into_iter()
                        .map(|chunk| D10Hsl::new(chunk[0], chunk[1], chunk[2]).to_rgb())
                        .collect(),
                    "hsv" => chunked::<3>(&mut iter)
                        .into_iter()
                        .map(|chunk| D10Hsv::new(chunk[0], chunk[1], chunk[2]).to_rgb())
                        .collect(),
                    "yuv" => chunked::<3>(&mut iter)
                        .into_iter()
                        .map(|chunk| D10Yuv::new(chunk[0], chunk[1], chunk[2]).to_rgb())
                        .collect(),
                    _ => return Err(PyOSError::new_err(format!("Bad colorspace {} for dimensions: {}", colorspace, ndims)))
                }
            } else if dims[2] == 1 {
                if colorspace != "gray" && colorspace != "auto" {
                    return Err(PyOSError::new_err(format!("Bad colorspace {} for dimensions: {}", colorspace, ndims)));
                }
                iter.map(|value| D10Rgb::new_with_alpha(value, value, value, 1.0)).collect()
            } else {
                return Err(PyOSError::new_err(format!("Bad color dimensions: {}", dims[2])));
            }
        } else if ndims == 2 {
            if colorspace != "gray" && colorspace != "auto" {
                return Err(PyOSError::new_err(format!("Bad colorspace {} for dimensions: {}", colorspace, ndims)));
            }
            iter.map(|value| D10Rgb::new_with_alpha(value, value, value, 1.0)).collect()
        } else {
            return Err(PyOSError::new_err(format!("Bad number of dimensions: {}", ndims)));
        };

        assert!(width * height == data.len());

        Ok(D10Image::new_from_raw(width as u32, height as u32, data).into())
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

    fn __getitem__(&self, key: (i32, i32)) -> PyResult<Rgb> {
        let x = key.0;
        let y = key.1;

        self.get_pixel(x, y).ok_or_else(|| {
            PyOSError::new_err(format!("Array not within range: {}x{}", y, x))
        })
    }

    fn __setitem__(&mut self, key: (u32, u32), value: Rgb) {
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
    fn jpeg(quality: Option<u8>, grayscale: Option<bool>) -> EncodingFormat {
        EncodingFormat {
            inner: D10EncodingFormat::Jpeg {
                quality: quality.unwrap_or(85),
                grayscale: grayscale.unwrap_or(false),
            }
        }
    }

    #[staticmethod]
    fn png(color_type: Option<&str>, compression: Option<&str>, filter: Option<&str>) -> PyResult<EncodingFormat> {
        let color_type = match color_type {
            Some(v) => v.parse().py_err()?,
            None => PngColorType::Rgba8,
        };
        let compression = match compression {
            Some(v) => v.parse().py_err()?,
            None => PngCompression::Default
        };

        let filter = match filter {
            Some(v) => v.parse().py_err()?,
            None => PngFilterType::Sub,
        };

        Ok(EncodingFormat {
            inner: D10EncodingFormat::Png {
                color_type,
                compression,
                filter,
            }
        })
    }

    #[staticmethod]
    fn gif() -> EncodingFormat {
        EncodingFormat {
            inner: D10EncodingFormat::Gif
        }
    }

    #[staticmethod]
    fn bmp(color_type: Option<&str>) -> PyResult<EncodingFormat> {
        let color_type = match color_type {
            Some(v) => v.parse().py_err()?,
            None => BmpColorType::Rgba8,
        };

        Ok(EncodingFormat {
            inner: D10EncodingFormat::Bmp {
                color_type
            }
        })
    }

    #[staticmethod]
    fn ico(color_type: Option<&str>) -> PyResult<EncodingFormat> {
        let color_type = match color_type {
            Some(v) => v.parse().py_err()?,
            None => IcoColorType::Rgba8,
        };

        Ok(EncodingFormat {
            inner: D10EncodingFormat::Ico {
                color_type,
            }
        })
    }
}

#[cfg(feature = "numpy")]
mod numpy_helper {
    use pyo3::{PyAny, PyResult};
    use pyo3::exceptions::PyOSError;
    use numpy::{PyArrayDyn, IxDyn, DataType};

    pub fn extract_data_type(data_type: Option<&PyAny>) -> PyResult<DataType> {
        let data_type = match data_type {
            Some(data_type) => data_type,
            None => return Ok(DataType::Float32),
        };

        if let Ok(str) = data_type.extract::<String>() {
            return match str.as_str() {
                "float32" => Ok(DataType::Float32),
                "float64" => Ok(DataType::Float64),
                "uint8" => Ok(DataType::Uint8),
                "uint16" => Ok(DataType::Uint16),
                "uint32" => Ok(DataType::Uint32),
                "bool" => Ok(DataType::Bool),
                _ => Err(PyOSError::new_err(format!("Unsupported data type: {:?}", str)))
            };
        }

        let data_type = data_type.to_string();

        return match data_type.as_str() {
            "<class 'numpy.float32'>" => Ok(DataType::Float32),
            "<class 'numpy.float64'>" => Ok(DataType::Float64),
            "<class 'numpy.uint8'>" => Ok(DataType::Uint8),
            "<class 'numpy.uint16'>" => Ok(DataType::Uint16),
            "<class 'numpy.uint32'>" => Ok(DataType::Uint32),
            "<class 'numpy.bool'>" | "<class 'numpy.bool_'>" | "<class 'bool'>" => Ok(DataType::Bool),
            _ => Err(PyOSError::new_err(format!("Unsupported data type: {}", data_type)))
        };
    }

    pub fn into_f32_array<'a>(array: &'a PyAny) -> PyResult<(usize, IxDyn, Box<dyn Iterator<Item=f32> + 'a>)> {

        //WARNING: In order to find out what data type this numpy array has, we
        //         blindly cast it into an f32 one, which might result into an
        //         array with broken data. This seems to be "save" on the rust side
        //         but might still causes undefined behavior...
        //
        //TODO: Find a way to not use this hackish way to do it

        let py_array: &PyArrayDyn<f32> = array.cast_as()?;

        let data_type = py_array.dtype().get_datatype().ok_or_else(|| PyOSError::new_err("Bad data type for array".to_owned()))?;

        let ndims = py_array.ndim();
        let dims = py_array.dims();

        use numpy::DataType::*;

        let iter: Box<dyn Iterator<Item=f32> + 'a> = match data_type {
            Float32 => Box::new(py_array.readonly().iter()?.copied()),
            Float64 => {
                let py_array: &PyArrayDyn<f64> = array.cast_as()?;
                Box::new(py_array.readonly().iter()?.map(|v| (*v) as f32))
            }
            Bool => {
                let py_array: &PyArrayDyn<bool> = array.cast_as()?;
                Box::new(py_array.readonly().iter()?.map(|v| if *v { 0.0f32 } else { 1.0f32 }))
            }
            Uint8 => {
                let py_array: &PyArrayDyn<u8> = array.cast_as()?;
                Box::new(py_array.readonly().iter()?.map(|v| (*v as f32) / 255.0))
            }
            Uint16 => {
                let py_array: &PyArrayDyn<u16> = array.cast_as()?;
                Box::new(py_array.readonly().iter()?.map(|v| (*v as f32) / 65535.0))
            }
            Uint32 => {
                let py_array: &PyArrayDyn<u32> = array.cast_as()?;
                Box::new(py_array.readonly().iter()?.map(|v| (*v as f32) / 4294967295.0))
            }
            _ => return Err(PyOSError::new_err(format!("Unsupported data type for numpy array: {:?}", data_type)))
        };

        Ok((ndims, dims, iter))
    }

    pub struct ColorIter<const N: usize> {
        data: [f32; 4],
        index: usize,
    }

    pub fn color_iter<const N: usize>(data: &[f32; 4]) -> ColorIter<N> {
        ColorIter {
            data: data.to_owned(),
            index: 0,
        }
    }

    impl<const N: usize> Iterator for ColorIter<N> {
        type Item = f32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < N {
                let r = self.data[self.index];
                self.index += 1;
                Some(r)
            } else {
                None
            }
        }
    }

    pub struct ChunkedIter<'a, const N: usize> {
        iter: &'a mut dyn Iterator<Item=f32>
    }

    pub fn chunked<const N: usize>(iter: &mut dyn Iterator<Item=f32>) -> ChunkedIter<N> {
        ChunkedIter {
            iter
        }
    }

    impl<'a, const N: usize> Iterator for ChunkedIter<'a, N> {
        type Item = [f32; N];

        fn next(&mut self) -> Option<Self::Item> {
            let mut data = [0f32; N];


            for d in data.iter_mut().take(N) {
                if let Some(v) = self.iter.next() {
                    *d = v;
                } else {
                    return None;
                }
            }

            Some(data)
        }
    }
}
