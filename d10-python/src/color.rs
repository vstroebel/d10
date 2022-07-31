#![allow(clippy::upper_case_acronyms)]

use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::PyFunction;

use d10::{
    illuminant, observer, Color, Hsl as D10Hsl, Hsv as D10Hsv, Lab as D10Lab, Lch as D10Lch,
    Rgb as D10Rgb, Srgb as D10Srgb, Xyz as D10Xyz, Yuv as D10Yuv,
};

use crate::IntoPyErr;

macro_rules! color_type {
    ($type_name:ident,
    $d10_type_name:ident,
    $value_1:ident,
    $value_2:ident,
    $value_3:ident,
    $get_value_1:ident,
    $get_value_2:ident,
    $get_value_3:ident,
    $set_value_1:ident,
    $set_value_2:ident,
    $set_value_3:ident,
    $with_value_1:ident,
    $with_value_2:ident,
    $with_value_3:ident
    $($function:item)*
    ) => {
        #[pyclass]
        #[derive(Clone)]
        pub struct $type_name {
            pub inner: $d10_type_name
        }

        #[pymethods]
        impl $type_name {
            #[new]
            pub fn new($value_1: f32, $value_2: f32, $value_3: f32, alpha: Option<f32>) -> $type_name {
                $d10_type_name::new_with_alpha($value_1, $value_2, $value_3, alpha.unwrap_or(1.0)).into()
            }

            #[getter]
            fn $get_value_1(&self) -> f32 {
                self.inner.$value_1()
            }

            #[setter]
            fn $set_value_1(&mut self, $value_1: f32) {
                self.inner.$set_value_1($value_1);
            }

            #[getter]
            fn $get_value_2(&self) -> f32 {
                self.inner.$value_2()
            }

            #[setter]
            fn $set_value_2(&mut self, $value_2: f32) {
                self.inner.$set_value_2($value_2);
            }

            #[getter]
            fn $get_value_3(&self) -> f32 {
                self.inner.$value_3()
            }

            #[setter]
            fn $set_value_3(&mut self, $value_3: f32) {
                self.inner.$set_value_3($value_3);
            }

            #[getter]
            fn get_alpha(&self) -> f32 {
                self.inner.alpha()
            }

            #[setter]
            fn set_alpha(&mut self, alpha: f32) {
                self.inner.set_alpha(alpha);
            }

            fn has_transparency(&self) -> bool {
                self.inner.has_transparency()
            }

            fn $with_value_1(&self, $value_1: f32) -> $type_name {
                self.inner.$with_value_1($value_1).into()
            }

            fn $with_value_2(&self, $value_2: f32) -> $type_name {
                self.inner.$with_value_2($value_2).into()
            }

            fn $with_value_3(&self, $value_3: f32) -> $type_name {
                self.inner.$with_value_3($value_3).into()
            }

            fn with_alpha(&self, alpha: f32) -> $type_name {
                self.inner.with_alpha(alpha).into()
            }

            fn to_rgb(&self) -> Rgb {
                self.inner.to_rgb().into()
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

            fn to_xyz(&self) -> Xyz {
                self.inner.to_xyz().into()
            }

            fn to_lab(&self, py: Python, illuminant: Option<&str>, observer: Option<&str>) -> PyResult<Py<PyAny>> {
                use pyo3::conversion::IntoPy;
                use pyo3::exceptions::PyOSError;

                let illuminant = illuminant.unwrap_or("D65");
                let observer = observer.unwrap_or("2");

                match (illuminant, observer) {
                    ("D65", "2") => Ok(LabD65O2 {inner : self.inner.to_lab()}.into_py(py)),
                    ("D65", "10") => Ok(LabD65O10 {inner : self.inner.to_lab()}.into_py(py)),
                    ("D50", "2") => Ok(LabD50O2 {inner : self.inner.to_lab()}.into_py(py)),
                    ("D50", "10") => Ok(LabD50O10 {inner : self.inner.to_lab()}.into_py(py)),
                    ("E", "2") => Ok(LabEO2 {inner : self.inner.to_lab()}.into_py(py)),
                    ("E", "10") => Ok(LabEO10 {inner : self.inner.to_lab()}.into_py(py)),
                    _ => Err(PyOSError::new_err(format!("Unsupported Lab type: {} {}", illuminant, observer))),
                }
            }

            fn to_lch(&self, py: Python, illuminant: Option<&str>, observer: Option<&str>) -> PyResult<Py<PyAny>> {
                use pyo3::conversion::IntoPy;
                use pyo3::exceptions::PyOSError;

                let illuminant = illuminant.unwrap_or("D65");
                let observer = observer.unwrap_or("2");

                match (illuminant, observer) {
                    ("D65", "2") => Ok(LchD65O2 {inner : self.inner.to_lch()}.into_py(py)),
                    ("D65", "10") => Ok(LchD65O10 {inner : self.inner.to_lch()}.into_py(py)),
                    ("D50", "2") => Ok(LchD50O2 {inner : self.inner.to_lch()}.into_py(py)),
                    ("D50", "10") => Ok(LchD50O10 {inner : self.inner.to_lch()}.into_py(py)),
                    ("E", "2") => Ok(LchEO2 {inner : self.inner.to_lch()}.into_py(py)),
                    ("E", "10") => Ok(LchEO10 {inner : self.inner.to_lch()}.into_py(py)),
                    _ => Err(PyOSError::new_err(format!("Unsupported Lch type: {} {}", illuminant, observer))),
                }
            }

            fn map_color_channels(&self, func: &PyFunction) -> PyResult<$type_name> {
                let map = |v: f32| -> PyResult<f32> {
                    let r = func.call1((v, ))?;
                    r.extract::<f32>()
                };
                Ok(self.inner.try_map_color_channels(map)?.into())
            }

            #[getter]
            fn type_name(&self) -> &str {
                self.inner.type_name()
            }

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

            $($function)*
        }

        impl From<$d10_type_name> for $type_name {
            fn from(color: $d10_type_name) -> $type_name  {
                $type_name  {
                    inner: color
                }
            }
        }

        impl From<& $d10_type_name> for $type_name  {
            fn from(color: &$d10_type_name) -> $type_name  {
                $type_name  {
                    inner: *color
                }
            }
        }
    };
}

color_type!(Rgb, D10Rgb, red, green, blue, get_red, get_green, get_blue, set_red, set_green, set_blue, with_red, with_green, with_blue
fn is_grayscale(&self) -> bool {
    self.inner.is_grayscale()
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

fn with_gamma_saturation(&self, gamma: f32) -> Rgb {
    self.inner.with_gamma_saturation(gamma).into()
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
});
color_type!(
    Srgb, D10Srgb, red, green, blue, get_red, get_green, get_blue, set_red, set_green, set_blue,
    with_red, with_green, with_blue
);
color_type!(
    Hsl,
    D10Hsl,
    hue,
    saturation,
    lightness,
    get_hue,
    get_saturation,
    get_lightness,
    set_hue,
    set_saturation,
    set_lightness,
    with_hue,
    with_saturation,
    with_lightness
);
color_type!(
    Hsv,
    D10Hsv,
    hue,
    saturation,
    value,
    get_hue,
    get_saturation,
    get_value,
    set_hue,
    set_saturation,
    set_value,
    with_hue,
    with_saturation,
    with_value
);
color_type!(Yuv, D10Yuv, y, u, v, get_y, get_u, get_v, set_y, set_u, set_v, with_y, with_u, with_v);
color_type!(Xyz, D10Xyz, x, y, z, get_x, get_y, get_z, set_x, set_y, set_z, with_x, with_y, with_z);

pub type D10LabD65O2 = D10Lab<illuminant::D65, observer::O2>;
pub type D10LabD65O10 = D10Lab<illuminant::D65, observer::O10>;
pub type D10LabD50O2 = D10Lab<illuminant::D50, observer::O2>;
pub type D10LabD50O10 = D10Lab<illuminant::D50, observer::O10>;
pub type D10LabEO2 = D10Lab<illuminant::E, observer::O2>;
pub type D10LabEO10 = D10Lab<illuminant::E, observer::O10>;

color_type!(
    LabD65O2,
    D10LabD65O2,
    l,
    a,
    b,
    get_l,
    get_a,
    get_b,
    set_l,
    set_a,
    set_b,
    with_l,
    with_a,
    with_b
);
color_type!(
    LabD65O10,
    D10LabD65O10,
    l,
    a,
    b,
    get_l,
    get_a,
    get_b,
    set_l,
    set_a,
    set_b,
    with_l,
    with_a,
    with_b
);
color_type!(
    LabD50O2,
    D10LabD50O2,
    l,
    a,
    b,
    get_l,
    get_a,
    get_b,
    set_l,
    set_a,
    set_b,
    with_l,
    with_a,
    with_b
);
color_type!(
    LabD50O10,
    D10LabD50O10,
    l,
    a,
    b,
    get_l,
    get_a,
    get_b,
    set_l,
    set_a,
    set_b,
    with_l,
    with_a,
    with_b
);
color_type!(
    LabEO2, D10LabEO2, l, a, b, get_l, get_a, get_b, set_l, set_a, set_b, with_l, with_a, with_b
);
color_type!(
    LabEO10, D10LabEO10, l, a, b, get_l, get_a, get_b, set_l, set_a, set_b, with_l, with_a, with_b
);

pub type D10LchD65O2 = D10Lch<illuminant::D65, observer::O2>;
pub type D10LchD65O10 = D10Lch<illuminant::D65, observer::O10>;
pub type D10LchD50O2 = D10Lch<illuminant::D50, observer::O2>;
pub type D10LchD50O10 = D10Lch<illuminant::D50, observer::O10>;
pub type D10LchEO2 = D10Lch<illuminant::E, observer::O2>;
pub type D10LchEO10 = D10Lch<illuminant::E, observer::O10>;

color_type!(
    LchD65O2,
    D10LchD65O2,
    l,
    c,
    h,
    get_l,
    get_c,
    get_h,
    set_l,
    set_c,
    set_h,
    with_l,
    with_c,
    with_h
);
color_type!(
    LchD65O10,
    D10LchD65O10,
    l,
    c,
    h,
    get_l,
    get_c,
    get_h,
    set_l,
    set_c,
    set_h,
    with_l,
    with_c,
    with_h
);
color_type!(
    LchD50O2,
    D10LchD50O2,
    l,
    c,
    h,
    get_l,
    get_c,
    get_h,
    set_l,
    set_c,
    set_h,
    with_l,
    with_c,
    with_h
);
color_type!(
    LchD50O10,
    D10LchD50O10,
    l,
    c,
    h,
    get_l,
    get_c,
    get_h,
    set_l,
    set_c,
    set_h,
    with_l,
    with_c,
    with_h
);
color_type!(
    LchEO2, D10LchEO2, l, c, h, get_l, get_c, get_h, set_l, set_c, set_h, with_l, with_c, with_h
);
color_type!(
    LchEO10, D10LchEO10, l, c, h, get_l, get_c, get_h, set_l, set_c, set_h, with_l, with_c, with_h
);
