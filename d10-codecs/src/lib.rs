mod utils;
mod jpeg;
mod png;

use utils::*;

use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;
use d10_core::errors::{D10Result, D10Error};

use image::ImageError;
use image::io::Reader;

use std::path::Path;
use std::io::{Cursor, Read, Seek, BufRead, Write};
use std::fs::File;

pub use crate::png::{PNGColorType, PNGCompressionType, PNGFilterType};

pub enum Format {
    JPEG {
        quality: u8
    },
    PNG {
        color_type: PNGColorType,
        compression: PNGCompressionType,
        filter: PNGFilterType,
    },
}

impl Format {
    pub fn jpeg_default() -> Self {
        Self::JPEG {
            quality: 85
        }
    }

    pub fn png_default() -> Self {
        Self::PNG {
            color_type: PNGColorType::RGBA8,
            compression: PNGCompressionType::Default,
            filter: PNGFilterType::Sub,
        }
    }

    pub fn from_path(path: &Path) -> D10Result<Format> {
        let ext = path
            .extension()
            .ok_or_else(|| D10Error::SaveError(format!("Unknown file extension in path: {}", path.to_string_lossy())))?
            .to_string_lossy()
            .to_ascii_lowercase();

        match ext.as_str() {
            "png" => Ok(Format::png_default()),
            "jpg" | "jpeg" => Ok(Format::jpeg_default()),
            _ => Err(D10Error::SaveError(format!("Unknown file extension in path: {}", path.to_string_lossy())))
        }
    }
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

pub fn save_to_file<P>(path: P, buffer: &PixelBuffer<RGB>, format: Option<Format>) -> D10Result<()> where P: AsRef<Path> {
    let format = match format {
        Some(format) => format,
        None => Format::from_path(path.as_ref())?
    };

    match format {
        Format::JPEG { quality } => jpeg::save_jpeg(&mut File::create(path)?, buffer, quality),
        Format::PNG { color_type, compression, filter } => png::save_png(&mut File::create(path)?, buffer, color_type, compression, filter),
    }
}

pub fn save<W>(w: &mut W, buffer: &PixelBuffer<RGB>, format: Format) -> D10Result<()> where W: Write {
    match format {
        Format::JPEG { quality } => jpeg::save_jpeg(w, buffer, quality),
        Format::PNG { color_type, compression, filter } => png::save_png(w, buffer, color_type, compression, filter),
    }
}

