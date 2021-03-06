use d10_core::color::Rgb;
use d10_core::errors::ParseEnumError;
use d10_core::pixelbuffer::PixelBuffer;

use std::io::{BufRead, Read, Seek, Write};
use std::str::FromStr;

use image::codecs::bmp::{BmpDecoder, BmpEncoder};
use image::{ColorType, DynamicImage, ImageError};

use crate::utils::{read_into_buffer, to_l8_vec, to_la8_vec, to_rgb8_vec, to_rgba8_vec};
use crate::{DecodedImage, DecodingError, EncodingError};

#[derive(Copy, Clone, Debug)]
pub enum BmpColorType {
    L8,
    La8,
    Rgb8,
    Rgba8,
}

impl FromStr for BmpColorType {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use BmpColorType::*;
        match value {
            "l8" => Ok(L8),
            "la8" => Ok(La8),
            "rgb8" => Ok(Rgb8),
            "rgba8" => Ok(Rgba8),
            _ => Err(ParseEnumError::new(value, "BmpColorType")),
        }
    }
}

pub(crate) fn encode_bmp<W>(
    mut w: W,
    buffer: &PixelBuffer<Rgb>,
    color_type: BmpColorType,
) -> Result<(), EncodingError>
where
    W: Write,
{
    let (out, color_type) = match color_type {
        BmpColorType::L8 => (to_l8_vec(buffer), ColorType::L8),
        BmpColorType::La8 => (to_la8_vec(buffer), ColorType::La8),
        BmpColorType::Rgb8 => (to_rgb8_vec(buffer), ColorType::Rgb8),
        BmpColorType::Rgba8 => (to_rgba8_vec(buffer), ColorType::Rgba8),
    };

    if let Err(err) =
        BmpEncoder::new(&mut w).encode(&out, buffer.width(), buffer.height(), color_type)
    {
        Err(match err {
            ImageError::IoError(err) => EncodingError::IoError(err),
            err => EncodingError::Encoding(err.to_string()),
        })
    } else {
        Ok(())
    }
}

pub(crate) fn decode_bmp<T>(reader: T) -> Result<DecodedImage, DecodingError>
where
    T: Read + Seek + BufRead,
{
    let decoder = BmpDecoder::new(reader).map_err(|err| match err {
        ImageError::IoError(err) => DecodingError::IoError(err),
        err => DecodingError::Decoding(err.to_string()),
    })?;

    let img = DynamicImage::from_decoder(decoder).map_err(|err| match err {
        ImageError::IoError(err) => DecodingError::IoError(err),
        err => DecodingError::Decoding(err.to_string()),
    })?;

    read_into_buffer(img).map(|buffer| DecodedImage { buffer })
}
