use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;
use d10_core::errors::{D10Result, D10Error};
use image::{ColorType, ImageError, DynamicImage};

use std::io::{Write, Seek, BufRead, Read};

use crate::utils::*;
use crate::DecodedImage;

//TODO: Wrap types to not export image crate internals
pub use image::codecs::png::CompressionType as PNGCompressionType;
pub use image::codecs::png::FilterType as PNGFilterType;
use image::codecs::png::{PngEncoder, PngDecoder};


pub enum PNGColorType {
    L8,
    LA8,
    L16,
    LA16,
    RGB8,
    RGBA8,
    RGB16,
    RGBA16,
}


pub(crate) fn save_png<W>(w: &mut W,
                          buffer: &PixelBuffer<RGB>,
                          color_type: PNGColorType,
                          compression: PNGCompressionType,
                          filter: PNGFilterType) -> D10Result<()>
    where W: Write {
    let (out, color_type) = match color_type {
        PNGColorType::L8 => (to_l8_vec(buffer), ColorType::L8),
        PNGColorType::LA8 => (to_la8_vec(buffer), ColorType::La8),
        PNGColorType::L16 => (to_l16_be_vec(buffer), ColorType::L16),
        PNGColorType::LA16 => (to_la16_be_vec(buffer), ColorType::La16),
        PNGColorType::RGB8 => (to_rgb8_vec(buffer), ColorType::Rgb8),
        PNGColorType::RGBA8 => (to_rgba8_vec(buffer), ColorType::Rgba8),
        PNGColorType::RGB16 => (to_rgb16_be_vec(buffer), ColorType::Rgba16),
        PNGColorType::RGBA16 => (to_rgba16_be_vec(buffer), ColorType::Rgba16)
    };

    if let Err(err) = PngEncoder::new_with_quality(w, compression, filter)
        .encode(&out, buffer.width(), buffer.height(), color_type) {
        Err(D10Error::SaveError(format!("Save error: {:?}", err)))
    } else {
        Ok(())
    }
}

pub(crate) fn decode_png<T>(reader: T) -> D10Result<DecodedImage> where T: Read + Seek + BufRead {
    let decoder = PngDecoder::new(reader)
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