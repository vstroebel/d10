use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;
use d10_core::errors::{D10Result, D10Error};
use std::io::Write;

use image::codecs::jpeg::JpegEncoder;
use image::ColorType;

use crate::utils::to_rgb8_vec;

pub(crate) fn save_jpeg<W>(w: &mut W, buffer: &PixelBuffer<RGB>, quality: u8) -> D10Result<()> where W: Write {
    let out = to_rgb8_vec(buffer);

    if let Err(err) = JpegEncoder::new_with_quality(w, quality).encode(
        &out,
        buffer.width(),
        buffer.height(),
        ColorType::Rgb8) {
        Err(D10Error::SaveError(format!("Save error: {:?}", err)))
    } else {
        Ok(())
    }
}