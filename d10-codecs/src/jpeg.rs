use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::Rgb;
use std::io::{Write, Read, Seek, BufRead};

use image::codecs::jpeg::{JpegEncoder, JpegDecoder};
use image::{ColorType, ImageError, DynamicImage};

use crate::utils::{to_rgb8_vec, read_into_buffer, to_l8_vec};
use crate::{DecodedImage, EncodingError, DecodingError};

pub(crate) fn encode_jpeg<W>(mut w: W, buffer: &PixelBuffer<Rgb>, quality: u8, grayscale: bool) -> Result<(), EncodingError> where W: Write {
    let width = buffer.width();
    let height = buffer.height();

    if width > u16::MAX as u32 || height > u16::MAX as u32 {
        return Err(EncodingError::BadDimensions {
            format: "jpeg",
            width,
            height,
        });
    }

    let (out, color_type) = if grayscale {
        (to_l8_vec(buffer), ColorType::L8)
    } else {
        (to_rgb8_vec(buffer), ColorType::Rgb8)
    };

    // Ensure quality is always in the valid range.
    let quality = quality.clamp(1, 100);

    if let Err(err) = JpegEncoder::new_with_quality(&mut w, quality).encode(
        &out,
        width,
        height,
        color_type) {
        Err(match err {
            ImageError::IoError(err) => EncodingError::IoError(err),
            err => EncodingError::Encoding(err.to_string())
        })
    } else {
        Ok(())
    }
}

pub(crate) fn decode_jpeg<T>(reader: T) -> Result<DecodedImage, DecodingError> where T: Read + Seek + BufRead {
    let decoder = JpegDecoder::new(reader)
        .map_err(|err| match err {
            ImageError::IoError(err) => DecodingError::IoError(err),
            err => DecodingError::Decoding(err.to_string())
        })?;

    let img = DynamicImage::from_decoder(decoder)
        .map_err(|err| match err {
            ImageError::IoError(err) => DecodingError::IoError(err),
            err => DecodingError::Decoding(err.to_string())
        })?;

    read_into_buffer(img).map(|buffer| DecodedImage {
        buffer
    })
}