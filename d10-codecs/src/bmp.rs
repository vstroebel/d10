use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;
use d10_core::errors::{D10Result, D10Error, ParseEnumError};

use std::io::{Write, Read, Seek, BufRead};
use std::str::FromStr;

use image::{ColorType, ImageError, DynamicImage};
use image::codecs::bmp::{BmpEncoder, BmpDecoder};

use crate::utils::{to_rgb8_vec, read_into_buffer, to_la8_vec, to_l8_vec, to_rgba8_vec};
use crate::DecodedImage;

#[derive(Copy, Clone, Debug)]
pub enum BMPColorType {
    L8,
    LA8,
    RGB8,
    RGBA8,
}

impl FromStr for BMPColorType {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use BMPColorType::*;
        match value {
            "l8" => Ok(L8),
            "la8" => Ok(LA8),
            "rgb8" => Ok(RGB8),
            "rgba8" => Ok(RGBA8),
            _ => Err(ParseEnumError::new(value, "BMPColorType"))
        }
    }
}

pub(crate) fn encode_bmp<W>(w: &mut W, buffer: &PixelBuffer<RGB>, color_type: BMPColorType) -> D10Result<()> where W: Write {
    let (out, color_type) = match color_type {
        BMPColorType::L8 => (to_l8_vec(buffer), ColorType::L8),
        BMPColorType::LA8 => (to_la8_vec(buffer), ColorType::La8),
        BMPColorType::RGB8 => (to_rgb8_vec(buffer), ColorType::Rgb8),
        BMPColorType::RGBA8 => (to_rgba8_vec(buffer), ColorType::Rgba8),
    };

    if let Err(err) = BmpEncoder::new(w).encode(
        &out,
        buffer.width(),
        buffer.height(),
        color_type) {
        Err(D10Error::SaveError(format!("Save error: {:?}", err)))
    } else {
        Ok(())
    }
}

pub(crate) fn decode_bmp<T>(reader: T) -> D10Result<DecodedImage> where T: Read + Seek + BufRead {
    let decoder = BmpDecoder::new(reader)
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