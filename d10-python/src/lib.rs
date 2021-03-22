mod color;
mod image;

use pyo3::prelude::*;
use pyo3::exceptions::PyOSError;

use d10::D10Result;

/// Helper trait used to convert D10Results into
trait IntoPyErr<T> {
    fn py_err(self) -> PyResult<T>;
}

impl<T> IntoPyErr<T> for D10Result<T> {
    fn py_err(self) -> PyResult<T> {
        self.map_err(|err| PyOSError::new_err(err.to_string()))
    }
}

#[pymodule]
fn d10(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<color::RGB>()?;
    m.add_class::<color::SRGB>()?;
    m.add_class::<color::HSL>()?;
    m.add_class::<color::HSV>()?;
    m.add_class::<color::YUV>()?;
    m.add_class::<image::Image>()?;
    m.add_class::<image::EncodingFormat>()?;

    Ok(())
}