use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;
use d10_core::errors::ParseEnumError;

use std::io::{Write, Seek, BufRead, Read};
use std::str::FromStr;

use crate::utils::*;
use crate::{DecodedImage, EncodingError, DecodingError};

use image::codecs::ico::{IcoEncoder, IcoDecoder};
use image::{ColorType, ImageError, DynamicImage};

#[derive(Copy, Clone, Debug)]
pub enum ICOColorType {
    L8,
    LA8,
    RGB8,
    RGBA8,
}

impl FromStr for ICOColorType {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use ICOColorType::*;
        match value {
            "l8" => Ok(L8),
            "la8" => Ok(LA8),
            "rgb8" => Ok(RGB8),
            "rgba8" => Ok(RGBA8),
            _ => Err(ParseEnumError::new(value, "ICOColorType"))
        }
    }
}

pub(crate) fn encode_ico<W>(w: &mut W,
                            buffer: &PixelBuffer<RGB>,
                            color_type: ICOColorType) -> Result<(), EncodingError>
    where W: Write {
    let (out, color_type) = match color_type {
        ICOColorType::L8 => (to_l8_vec(buffer), ColorType::L8),
        ICOColorType::LA8 => (to_la8_vec(buffer), ColorType::La8),
        ICOColorType::RGB8 => (to_rgb8_vec(buffer), ColorType::Rgb8),
        ICOColorType::RGBA8 => (to_rgba8_vec(buffer), ColorType::Rgba8),
    };

    if let Err(err) = IcoEncoder::new(w)
        .encode(&out, buffer.width(), buffer.height(), color_type) {
        Err(match err {
            ImageError::IoError(err) => EncodingError::IOError(err),
            err => EncodingError::Encoding(err.to_string())
        })
    } else {
        Ok(())
    }
}

pub(crate) fn decode_ico<T>(reader: T) -> Result<DecodedImage, DecodingError> where T: Read + Seek + BufRead {
    let decoder = IcoDecoder::new(reader)
        .map_err(|err| match err {
            ImageError::IoError(err) => DecodingError::IOError(err),
            err => DecodingError::Decoding(err.to_string())
        })?;

    let img = DynamicImage::from_decoder(decoder)
        .map_err(|err| match err {
            ImageError::IoError(err) => DecodingError::IOError(err),
            err => DecodingError::Decoding(err.to_string())
        })?;

    read_into_buffer(img).map(|buffer| DecodedImage {
        buffer
    })
}