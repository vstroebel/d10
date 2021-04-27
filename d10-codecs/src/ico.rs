use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::Rgb;
use d10_core::errors::ParseEnumError;

use std::io::{Write, Seek, BufRead, Read};
use std::str::FromStr;

use crate::utils::*;
use crate::{DecodedImage, EncodingError, DecodingError};

use image::codecs::ico::{IcoEncoder, IcoDecoder};
use image::{ColorType, ImageError, DynamicImage};

#[derive(Copy, Clone, Debug)]
pub enum IcoColorType {
    L8,
    La8,
    Rgb8,
    Rgba8,
}

impl FromStr for IcoColorType {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use IcoColorType::*;
        match value {
            "l8" => Ok(L8),
            "la8" => Ok(La8),
            "rgb8" => Ok(Rgb8),
            "rgba8" => Ok(Rgba8),
            _ => Err(ParseEnumError::new(value, "IcoColorType"))
        }
    }
}

pub(crate) fn encode_ico<W>(w: W,
                            buffer: &PixelBuffer<Rgb>,
                            color_type: IcoColorType) -> Result<(), EncodingError>
    where W: Write {
    let (out, color_type) = match color_type {
        IcoColorType::L8 => (to_l8_vec(buffer), ColorType::L8),
        IcoColorType::La8 => (to_la8_vec(buffer), ColorType::La8),
        IcoColorType::Rgb8 => (to_rgb8_vec(buffer), ColorType::Rgb8),
        IcoColorType::Rgba8 => (to_rgba8_vec(buffer), ColorType::Rgba8),
    };

    if let Err(err) = IcoEncoder::new(w)
        .encode(&out, buffer.width(), buffer.height(), color_type) {
        Err(match err {
            ImageError::IoError(err) => EncodingError::IoError(err),
            err => EncodingError::Encoding(err.to_string())
        })
    } else {
        Ok(())
    }
}

pub(crate) fn decode_ico<T>(reader: T) -> Result<DecodedImage, DecodingError> where T: Read + Seek + BufRead {
    let decoder = IcoDecoder::new(reader)
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