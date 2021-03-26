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
    m.add_class::<image::Image>()?;
    m.add_class::<image::EncodingFormat>()?;

    Ok(())
}