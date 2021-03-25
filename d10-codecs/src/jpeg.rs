use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;
use d10_core::errors::{D10Result, D10Error};
use std::io::{Write, Read, Seek, BufRead};

use image::codecs::jpeg::{JpegEncoder, JpegDecoder};
use image::{ColorType, ImageError, DynamicImage};

use crate::utils::{to_rgb8_vec, read_into_buffer};
use crate::DecodedImage;

pub(crate) fn encode_jpeg<W>(w: &mut W, buffer: &PixelBuffer<RGB>, quality: u8) -> D10Result<()> where W: Write {
    let out = to_rgb8_vec(buffer);

    // Ensure quality is always in the valid range.
    let quality = quality.clamp(1, 100);

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

pub(crate) fn decode_jpeg<T>(reader: T) -> D10Result<DecodedImage> where T: Read + Seek + BufRead {
    let decoder = JpegDecoder::new(reader)
        .map_err(|err| match err {
            ImageError::IoError(err) => D10Error::IOError(err),
            ImageError::Limits(l) => D10Error::Limits(format!("{:?}", l)),
            err => D10Error::OpenError(format!("Open error: {:?}", err))
        })?;

    let img = DynamicImage::from_decoder(decoder)
        .map_err(|err| match err {
            ImageError::IoError(err) => D10Error::IOError(err),
            ImageError::Limits(l) => D10Error::Limits(format!("{:?}", l)),
            err => D10Error::OpenError(format!("Decode error: {:?}", err))
        })?;

    read_into_buffer(img).map(|buffer| DecodedImage {
        buffer
    })
}