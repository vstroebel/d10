mod color;
mod image;

use pyo3::prelude::*;
use pyo3::exceptions::PyOSError;

use std::error::Error;

/// Helper trait used to convert d10 based results into PyResult
trait IntoPyErr<T> {
    fn py_err(self) -> PyResult<T>;
}

impl<T, E> IntoPyErr<T> for Result<T, E> where E: Error {
    fn py_err(self) -> PyResult<T> {
        self.map_err(|err| PyOSError::new_err(err.to_string()))
    }
}


#[pymodule]
fn d10(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<color::Rgb>()?;
    m.add_class::<color::Srgb>()?;
    m.add_class::<color::Hsl>()?;
    m.add_class::<color::Hsv>()?;
    m.add_class::<color::Yuv>()?;
    m.add_class::<color::Xyz>()?;

    m.add_class::<color::LabD65O2>()?;
    m.add_class::<color::LabD65O10>()?;
    m.add_class::<color::LabD50O2>()?;
    m.add_class::<color::LabD50O10>()?;
    m.add_class::<color::LabEO2>()?;
    m.add_class::<color::LabEO10>()?;

    m.add_class::<color::LchD65O2>()?;
    m.add_class::<color::LchD65O10>()?;
    m.add_class::<color::LchD50O2>()?;
    m.add_class::<color::LchD50O10>()?;
    m.add_class::<color::LchEO2>()?;
    m.add_class::<color::LchEO10>()?;

    m.add_class::<image::Image>()?;
    m.add_class::<image::EncodingFormat>()?;

    #[pyfn(m, "Lab")]
    fn lab(py: Python, l: f32, a: f32, b: f32, alpha: Option<f32>, illuminant: Option<&str>, observer: Option<&str>) -> PyResult<Py<PyAny>> {
        use pyo3::conversion::IntoPy;
        use crate::color::{LabD65O10, LabD65O2, LabD50O10, LabEO2, LabEO10, LabD50O2};

        let illuminant = illuminant.unwrap_or("D65");
        let observer = observer.unwrap_or("2");

        match (illuminant, observer) {
            ("D65", "2") => Ok(LabD65O2::new(l, a, b, alpha).into_py(py)),
            ("D65", "10") => Ok(LabD65O10::new(l, a, b, alpha).into_py(py)),
            ("D50", "2") => Ok(LabD50O2::new(l, a, b, alpha).into_py(py)),
            ("D50", "10") => Ok(LabD50O10::new(l, a, b, alpha).into_py(py)),
            ("E", "2") => Ok(LabEO2::new(l, a, b, alpha).into_py(py)),
            ("E", "10") => Ok(LabEO10::new(l, a, b, alpha).into_py(py)),
            _ => Err(PyOSError::new_err(format!("Unsupported Lab type: {} {}", illuminant, observer))),
        }
    }

    #[pyfn(m, "Lch")]
    fn lch(py: Python, l: f32, c: f32, h: f32, alpha: Option<f32>, illuminant: Option<&str>, observer: Option<&str>) -> PyResult<Py<PyAny>> {
        use pyo3::conversion::IntoPy;
        use crate::color::{LchD65O10, LchD65O2, LchD50O10, LchEO2, LchEO10, LchD50O2};

        let illuminant = illuminant.unwrap_or("D65");
        let observer = observer.unwrap_or("2");

        match (illuminant, observer) {
            ("D65", "2") => Ok(LchD65O2::new(l, c, h, alpha).into_py(py)),
            ("D65", "10") => Ok(LchD65O10::new(l, c, h, alpha).into_py(py)),
            ("D50", "2") => Ok(LchD50O2::new(l, c, h, alpha).into_py(py)),
            ("D50", "10") => Ok(LchD50O10::new(l, c, h, alpha).into_py(py)),
            ("E", "2") => Ok(LchEO2::new(l, c, h, alpha).into_py(py)),
            ("E", "10") => Ok(LchEO10::new(l, c, h, alpha).into_py(py)),
            _ => Err(PyOSError::new_err(format!("Unsupported Lch type: {} {}", illuminant, observer))),
        }
    }

    Ok(())
}