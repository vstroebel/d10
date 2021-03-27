use pyo3::prelude::*;
use pyo3::PyObjectProtocol;
use pyo3::types::PyFunction;
use pyo3::basic::CompareOp;

use d10::{Color,
          Rgb as D10Rgb,
          Srgb as D10Srgb,
          Hsl as D10Hsl,
          Hsv as D10Hsv,
          Yuv as D10Yuv};

use crate::IntoPyErr;


#[pyclass]
#[derive(Clone)]
pub struct Rgb {
    pub inner: D10Rgb
}

#[pymethods]
impl Rgb {
    #[new]
    pub fn new(red: f32, green: f32, blue: f32, alpha: Option<f32>) -> Self {
        Self {
            inner: D10Rgb::new_with_alpha(red, green, blue, alpha.unwrap_or(1.0))
        }
    }

    #[getter]
    fn get_red(&self) -> f32 {
        self.inner.red()
    }

    #[getter]
    fn get_green(&self) -> f32 {
        self.inner.green()
    }

    #[getter]
    fn get_blue(&self) -> f32 {
        self.inner.blue()
    }

    #[getter]
    fn get_alpha(&self) -> f32 {
        self.inner.alpha()
    }

    fn is_grayscale(&self) -> bool {
        self.inner.is_grayscale()
    }

    fn has_transparency(&self) -> bool {
        self.inner.has_transparency()
    }

    fn with_red(&self, red: f32) -> Rgb {
        self.inner.with_red(red).into()
    }

    fn with_green(&self, green: f32) -> Rgb {
        self.inner.with_green(green).into()
    }

    fn with_blue(&self, blue: f32) -> Rgb {
        self.inner.with_blue(blue).into()
    }

    fn with_alpha(&self, alpha: f32) -> Rgb {
        self.inner.with_alpha(alpha).into()
    }

    fn to_gray(&self, intensity: Option<&str>) -> PyResult<Rgb> {
        Ok(if let Some(intensity) = intensity {
            self.inner.to_gray_with_intensity(intensity.parse().py_err()?)
        } else {
            self.inner.to_gray()
        }.into())
    }

    fn invert(&self) -> Rgb {
        self.inner.invert().into()
    }

    fn difference(&self, color: &Rgb) -> Rgb {
        self.inner.difference(&color.inner).into()
    }

    fn with_gamma(&self, gamma: f32) -> Rgb {
        self.inner.with_gamma(gamma).into()
    }

    fn with_level(&self, black_point: f32, white_point: f32, gamma: f32) -> Rgb {
        self.inner.with_level(black_point, white_point, gamma).into()
    }

    fn with_brightness(&self, factor: f32) -> Rgb {
        self.inner.with_brightness(factor).into()
    }

    fn with_saturation(&self, factor: f32) -> Rgb {
        self.inner.with_saturation(factor).into()
    }

    fn stretch_saturation(&self, factor: f32) -> Rgb {
        self.inner.stretch_saturation(factor).into()
    }

    fn with_lightness(&self, factor: f32) -> Rgb {
        self.inner.with_lightness(factor).into()
    }

    fn with_hue_rotate(&self, radians: f32) -> Rgb {
        self.inner.with_hue_rotate(radians).into()
    }

    fn with_contrast(&self, factor: f32) -> Rgb {
        self.inner.with_contrast(factor).into()
    }

    fn with_brightness_contrast(&self, brightness: f32, contrast: f32) -> Rgb {
        self.inner.with_brightness_contrast(brightness, contrast).into()
    }


    fn alpha_blend(&self, color: &Rgb) -> Rgb {
        self.inner.alpha_blend(color.inner).into()
    }

    fn with_vibrance(&self, factor: f32) -> Rgb {
        self.inner.with_vibrance(factor).into()
    }

    fn with_sepia(&self) -> Rgb {
        self.inner.with_sepia().into()
    }

    fn max(&self) -> f32 {
        self.inner.max()
    }

    fn min(&self) -> f32 {
        self.inner.min()
    }

    fn modulate(&self, hue: f32, saturation: f32, lightness: f32) -> Rgb {
        self.inner.modulate(hue, saturation, lightness).into()
    }

    fn map_color_channels(&self, func: &PyFunction) -> PyResult<Rgb> {
        let map = |v: f32| -> PyResult<f32> {
            let r = func.call1((v, ))?;
            r.extract::<f32>()
        };
        Ok(self.inner.try_map_color_channels(map)?.into())
    }

    fn to_srgb(&self) -> Srgb {
        self.inner.to_srgb().into()
    }

    fn to_hsl(&self) -> Hsl {
        self.inner.to_hsl().into()
    }

    fn to_hsv(&self) -> Hsv {
        self.inner.to_hsv().into()
    }

    fn to_yuv(&self) -> Yuv {
        self.inner.to_yuv().into()
    }

    #[getter]
    fn type_name(&self) -> &str {
        self.inner.type_name()
    }
}

impl From<D10Rgb> for Rgb {
    fn from(color: D10Rgb) -> Rgb {
        Rgb {
            inner: color
        }
    }
}

impl From<&D10Rgb> for Rgb {
    fn from(color: &D10Rgb) -> Rgb {
        Rgb {
            inner: *color
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Rgb {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    fn __richcmp__(&self, other: PyRef<Self>, op: CompareOp) -> PyResult<PyObject> {
        match op {
            CompareOp::Eq => Ok(self.inner.eq(&other.inner).into_py(other.py())),
            _ => Ok(other.py().NotImplemented()),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Srgb {
    pub inner: D10Srgb
}

#[pymethods]
impl Srgb {
    #[new]
    fn new(red: f32, green: f32, blue: f32, alpha: Option<f32>) -> Srgb {
        Srgb {
            inner: D10Srgb::new_with_alpha(red, green, blue, alpha.unwrap_or(1.0))
        }
    }

    #[getter]
    fn get_red(&self) -> f32 {
        self.inner.red()
    }

    #[getter]
    fn get_green(&self) -> f32 {
        self.inner.green()
    }

    #[getter]
    fn get_blue(&self) -> f32 {
        self.inner.blue()
    }

    #[getter]
    fn get_alpha(&self) -> f32 {
        self.inner.alpha()
    }

    fn with_red(&self, red: f32) -> Srgb {
        self.inner.with_red(red).into()
    }

    fn with_green(&self, green: f32) -> Srgb {
        self.inner.with_green(green).into()
    }

    fn with_blue(&self, blue: f32) -> Srgb {
        self.inner.with_blue(blue).into()
    }

    fn with_alpha(&self, alpha: f32) -> Srgb {
        self.inner.with_alpha(alpha).into()
    }

    fn to_rgb(&self) -> Rgb {
        self.inner.to_rgb().into()
    }

    fn to_hsl(&self) -> Hsl {
        self.inner.to_hsl().into()
    }

    fn to_hsv(&self) -> Hsv {
        self.inner.to_hsv().into()
    }

    fn to_yuv(&self) -> Yuv {
        self.inner.to_yuv().into()
    }

    #[getter]
    fn type_name(&self) -> &str {
        self.inner.type_name()
    }
}

impl From<D10Srgb> for Srgb {
    fn from(color: D10Srgb) -> Srgb {
        Srgb {
            inner: color
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Srgb {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    fn __richcmp__(&self, other: PyRef<Self>, op: CompareOp) -> PyResult<PyObject> {
        match op {
            CompareOp::Eq => Ok(self.inner.eq(&other.inner).into_py(other.py())),
            _ => Ok(other.py().NotImplemented()),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Hsl {
    pub inner: D10Hsl
}

#[pymethods]
impl Hsl {
    #[new]
    fn new(h: f32, s: f32, l: f32, alpha: Option<f32>) -> Hsl {
        D10Hsl::new_with_alpha(h, s, l, alpha.unwrap_or(1.0)).into()
    }

    #[getter]
    fn get_hue(&self) -> f32 {
        self.inner.hue()
    }

    #[getter]
    fn get_saturation(&self) -> f32 {
        self.inner.saturation()
    }

    #[getter]
    fn get_lightness(&self) -> f32 {
        self.inner.lightness()
    }

    #[getter]
    fn get_alpha(&self) -> f32 {
        self.inner.alpha()
    }

    fn has_transparency(&self) -> bool {
        self.inner.has_transparency()
    }

    fn with_hue(&self, hue: f32) -> Hsl {
        self.inner.with_hue(hue).into()
    }

    fn with_saturation(&self, saturation: f32) -> Hsl {
        self.inner.with_saturation(saturation).into()
    }

    fn with_lightness(&self, lightness: f32) -> Hsl {
        self.inner.with_lightness(lightness).into()
    }

    fn with_alpha(&self, alpha: f32) -> Hsl {
        self.inner.with_alpha(alpha).into()
    }

    fn to_srgb(&self) -> Srgb {
        self.inner.to_srgb().into()
    }

    fn to_rgb(&self) -> Rgb {
        self.inner.to_rgb().into()
    }

    fn to_hsv(&self) -> Hsv {
        self.inner.to_hsv().into()
    }

    fn to_yuv(&self) -> Yuv {
        self.inner.to_yuv().into()
    }

    #[getter]
    fn type_name(&self) -> &str {
        self.inner.type_name()
    }
}

impl From<D10Hsl> for Hsl {
    fn from(hsl: D10Hsl) -> Hsl {
        Hsl {
            inner: hsl
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Hsl {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    fn __richcmp__(&self, other: PyRef<Self>, op: CompareOp) -> PyResult<PyObject> {
        match op {
            CompareOp::Eq => Ok(self.inner.eq(&other.inner).into_py(other.py())),
            _ => Ok(other.py().NotImplemented()),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Hsv {
    pub inner: D10Hsv
}


#[pymethods]
impl Hsv {
    #[new]
    fn new(h: f32, s: f32, v: f32, alpha: Option<f32>) -> Hsv {
        D10Hsv::new_with_alpha(h, s, v, alpha.unwrap_or(1.0)).into()
    }

    #[getter]
    fn get_hue(&self) -> f32 {
        self.inner.hue()
    }

    #[getter]
    fn get_saturation(&self) -> f32 {
        self.inner.saturation()
    }

    #[getter]
    fn get_value(&self) -> f32 {
        self.inner.value()
    }

    #[getter]
    fn get_alpha(&self) -> f32 {
        self.inner.alpha()
    }

    fn has_transparency(&self) -> bool {
        self.inner.has_transparency()
    }

    fn with_hue(&self, hue: f32) -> Hsv {
        self.inner.with_hue(hue).into()
    }

    fn with_saturation(&self, saturation: f32) -> Hsv {
        self.inner.with_saturation(saturation).into()
    }

    fn with_value(&self, value: f32) -> Hsv {
        self.inner.with_value(value).into()
    }

    fn with_alpha(&self, alpha: f32) -> Hsv {
        self.inner.with_alpha(alpha).into()
    }

    fn to_srgb(&self) -> Srgb {
        self.inner.to_srgb().into()
    }

    fn to_rgb(&self) -> Rgb {
        self.inner.to_rgb().into()
    }

    fn to_hsl(&self) -> Hsl {
        self.inner.to_hsl().into()
    }

    fn to_yuv(&self) -> Yuv {
        self.inner.to_yuv().into()
    }

    #[getter]
    fn type_name(&self) -> &str {
        self.inner.type_name()
    }
}

impl From<D10Hsv> for Hsv {
    fn from(hsv: D10Hsv) -> Hsv {
        Hsv {
            inner: hsv
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Hsv {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    fn __richcmp__(&self, other: PyRef<Self>, op: CompareOp) -> PyResult<PyObject> {
        match op {
            CompareOp::Eq => Ok(self.inner.eq(&other.inner).into_py(other.py())),
            _ => Ok(other.py().NotImplemented()),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Yuv {
    pub inner: D10Yuv
}


#[pymethods]
impl Yuv {
    #[new]
    fn new(y: f32, u: f32, v: f32, alpha: Option<f32>) -> Yuv {
        D10Yuv::new_with_alpha(y, u, v, alpha.unwrap_or(1.0)).into()
    }

    #[getter]
    fn get_y(&self) -> f32 {
        self.inner.y()
    }

    #[getter]
    fn get_u(&self) -> f32 {
        self.inner.u()
    }

    #[getter]
    fn get_v(&self) -> f32 {
        self.inner.v()
    }

    #[getter]
    fn get_alpha(&self) -> f32 {
        self.inner.alpha()
    }

    fn has_transparency(&self) -> bool {
        self.inner.has_transparency()
    }

    fn with_y(&self, y: f32) -> Yuv {
        self.inner.with_y(y).into()
    }

    fn with_u(&self, u: f32) -> Yuv {
        self.inner.with_u(u).into()
    }

    fn with_v(&self, v: f32) -> Yuv {
        self.inner.with_v(v).into()
    }

    fn with_alpha(&self, alpha: f32) -> Yuv {
        self.inner.with_alpha(alpha).into()
    }

    fn to_srgb(&self) -> Srgb {
        self.inner.to_srgb().into()
    }

    fn to_rgb(&self) -> Rgb {
        self.inner.to_rgb().into()
    }

    fn to_hsl(&self) -> Hsl {
        self.inner.to_hsl().into()
    }

    fn to_hsv(&self) -> Hsv {
        self.inner.to_hsv().into()
    }

    #[getter]
    fn type_name(&self) -> &str {
        self.inner.type_name()
    }
}

impl From<D10Yuv> for Yuv {
    fn from(yuv: D10Yuv) -> Yuv {
        Yuv {
            inner: yuv
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Yuv {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    fn __richcmp__(&self, other: PyRef<Self>, op: CompareOp) -> PyResult<PyObject> {
        match op {
            CompareOp::Eq => Ok(self.inner.eq(&other.inner).into_py(other.py())),
            _ => Ok(other.py().NotImplemented()),
        }
    }
}