mod utils;

use utils::*;

use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;
use d10_core::errors::{D10Result, D10Error};

use image::{ImageError, ImageBuffer, Rgba, ColorType};
use image::io::Reader;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;

use std::path::Path;
use std::io::{Cursor, Read, Seek, BufRead};
use std::fs::File;

//TODO: Wrap types to not export image crate internals
pub use image::codecs::png::CompressionType as PNGCompressionType;
pub use image::codecs::png::FilterType as PNGFilterType;

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

pub enum Format {
    Auto,
    JPEG {
        quality: u8
    },
    PNG {
        color_type: PNGColorType,
        compression: PNGCompressionType,
        filter: PNGFilterType,
    },
}

pub struct DecodedImage {
    pub buffer: PixelBuffer<RGB>
}

pub fn decode_file<P>(path: P) -> D10Result<DecodedImage> where P: AsRef<Path> {
    decode(Reader::open(path)?)
}

pub fn decode_buffer(buffer: &[u8]) -> D10Result<DecodedImage> {
    decode(Reader::new(Cursor::new(buffer)).with_guessed_format()?)
}

fn decode<T>(reader: Reader<T>) -> D10Result<DecodedImage> where T: Read + Seek + BufRead {
    let img = reader.decode().map_err(|err| match err {
        ImageError::IoError(err) => D10Error::IOError(err),
        ImageError::Limits(l) => D10Error::Limits(format!("{:?}", l)),
        err => D10Error::OpenError(format!("Open error: {:?}", err))
    })?;

    read_into_buffer(img).map(|buffer| DecodedImage {
        buffer
    })
}

pub fn save_to_file<P>(path: P, buffer: &PixelBuffer<RGB>, format: Format) -> D10Result<()> where P: AsRef<Path> {
    match format {
        Format::JPEG { quality } => save_to_file_jpeg(path, buffer, quality),
        Format::PNG { color_type, compression, filter } => save_to_file_png(path, buffer, color_type, compression, filter),
        Format::Auto => save_to_file_auto(path, buffer)
    }
}

fn save_to_file_auto<P>(path: P, buffer: &PixelBuffer<RGB>) -> D10Result<()> where P: AsRef<Path> {
    let out = to_rgba8_vec(buffer);

    let out: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(buffer.width(), buffer.height(), out)
        .ok_or_else(|| D10Error::OpenError("Unable to create buffer".to_owned()))?;

    out.save(path).map_err(|err| match err {
        ImageError::IoError(err) => D10Error::IOError(err),
        ImageError::Limits(l) => D10Error::Limits(format!("{:?}", l)),
        err => D10Error::SaveError(format!("Save error: {:?}", err))
    })?;

    Ok(())
}

fn save_to_file_jpeg<P>(path: P, buffer: &PixelBuffer<RGB>, quality: u8) -> D10Result<()> where P: AsRef<Path> {
    let out = to_rgb8_vec(buffer);

    let mut result = File::create(path)?;

    if let Err(err) = JpegEncoder::new_with_quality(&mut result, quality).encode(&out, buffer.width(), buffer.height(), ColorType::Rgb8) {
        Err(D10Error::SaveError(format!("Save error: {:?}", err)))
    } else {
        Ok(())
    }
}

fn save_to_file_png<P>(path: P,
                       buffer: &PixelBuffer<RGB>,
                       color_type: PNGColorType,
                       compression: PNGCompressionType,
                       filter: PNGFilterType) -> D10Result<()>
    where P: AsRef<Path> {
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

    let mut result = File::create(path)?;

    if let Err(err) = PngEncoder::new_with_quality(&mut result, compression, filter)
        .encode(&out, buffer.width(), buffer.height(), color_type) {
        Err(D10Error::SaveError(format!("Save error: {:?}", err)))
    } else {
        Ok(())
    }
}