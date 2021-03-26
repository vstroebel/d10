mod utils;
mod jpeg;
mod png;
mod gif;
mod bmp;
mod ico;
mod errors;

use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;

use std::path::Path;
use std::io::{Cursor, Read, Seek, Write, SeekFrom, BufReader, BufRead, BufWriter};
use std::fs::File;

pub use crate::png::{PNGColorType, PNGCompression, PNGFilterType};
pub use crate::bmp::BMPColorType;
pub use crate::ico::ICOColorType;
pub use crate::errors::*;

use crate::jpeg::{decode_jpeg, encode_jpeg};
use crate::png::{decode_png, encode_png};
use crate::gif::{decode_gif, encode_gif};
use crate::bmp::{decode_bmp, encode_bmp};
use crate::ico::{decode_ico, encode_ico};

#[derive(Debug)]
pub enum Format {
    JPEG,
    PNG,
    GIF,
    BMP,
    ICO,
}

impl Format {
    pub fn from_path(path: &Path) -> Option<Format> {
        let ext = path
            .extension()?
            .to_string_lossy()
            .to_ascii_lowercase();

        match ext.as_str() {
            "jpg" | "jpeg" => Some(Self::JPEG),
            "png" => Some(Self::PNG),
            "gif" => Some(Self::GIF),
            "bmp" => Some(Self::BMP),
            "ico" => Some(Self::ICO),
            _ => None,
        }
    }

    pub fn from_reader<T>(reader: &mut T) -> Result<Format, DecodingError> where T: Read + Seek {
        let mut buf = [0u8; 8];

        let len = reader.read(&mut buf)?;

        reader.seek(SeekFrom::Start(0))?;

        match buf[0..len] {
            [0xFF, 0xD8, 0xFF, ..] => Ok(Format::JPEG),
            [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, ..] => Ok(Format::PNG),
            [0x47, 0x49, 0x46, 0x38, 0x37, 0x61, ..] => Ok(Format::GIF),
            [0x47, 0x49, 0x46, 0x38, 0x39, 0x61, ..] => Ok(Format::GIF),
            [0x42, 0x4D, ..] => Ok(Format::BMP),
            [0x00, 0x00, 0x01, 0x00, ..] => Ok(Format::ICO),
            _ => Err(DecodingError::UnknownFormat),
        }
    }
}

#[derive(Clone, Debug)]
pub enum EncodingFormat {
    JPEG {
        quality: u8
    },
    PNG {
        color_type: PNGColorType,
        compression: PNGCompression,
        filter: PNGFilterType,
    },
    GIF,
    BMP {
        color_type: BMPColorType
    },
    ICO {
        color_type: ICOColorType
    },
}

impl EncodingFormat {
    pub fn format(&self) -> Format {
        match self {
            EncodingFormat::JPEG { .. } => Format::JPEG,
            EncodingFormat::PNG { .. } => Format::PNG,
            EncodingFormat::GIF => Format::GIF,
            EncodingFormat::BMP { .. } => Format::BMP,
            EncodingFormat::ICO { .. } => Format::ICO,
        }
    }

    pub fn jpeg_default() -> Self {
        Self::JPEG {
            quality: 85
        }
    }

    pub fn png_default() -> Self {
        Self::PNG {
            color_type: PNGColorType::RGBA8,
            compression: PNGCompression::Default,
            filter: PNGFilterType::Sub,
        }
    }

    pub fn gif_default() -> Self {
        Self::GIF
    }

    pub fn bmp_default() -> Self {
        Self::BMP {
            color_type: BMPColorType::RGBA8
        }
    }

    pub fn ico_default() -> Self {
        Self::ICO {
            color_type: ICOColorType::RGBA8
        }
    }

    pub fn from_path(path: &Path) -> Result<EncodingFormat, EncodingError> {
        match Format::from_path(path) {
            Some(Format::JPEG) => Ok(EncodingFormat::jpeg_default()),
            Some(Format::PNG) => Ok(EncodingFormat::png_default()),
            Some(Format::GIF) => Ok(EncodingFormat::gif_default()),
            Some(Format::BMP) => Ok(EncodingFormat::bmp_default()),
            Some(Format::ICO) => Ok(EncodingFormat::ico_default()),
            None => Err(EncodingError::BadFileExtension(path.to_string_lossy().to_string()))
        }
    }
}

pub struct DecodedImage {
    pub buffer: PixelBuffer<RGB>
}

pub fn decode_file<P>(path: P) -> Result<DecodedImage, DecodingError> where P: AsRef<Path> {
    let format = match Format::from_path(path.as_ref()) {
        Some(format) => format,
        None => return Err(DecodingError::BadFileExtension(path.as_ref().to_string_lossy().to_string()))
    };

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    decode(reader, format)
}

pub fn decode_buffer(buffer: &[u8]) -> Result<DecodedImage, DecodingError> {
    let mut reader = Cursor::new(buffer);
    let format = Format::from_reader(&mut reader)?;

    decode(reader, format)
}

fn decode<T>(reader: T, format: Format) -> Result<DecodedImage, DecodingError> where T: Read + Seek + BufRead {
    match format {
        Format::JPEG => decode_jpeg(reader),
        Format::PNG => decode_png(reader),
        Format::GIF => decode_gif(reader),
        Format::BMP => decode_bmp(reader),
        Format::ICO => decode_ico(reader),
    }
}

pub fn encode_to_file<P>(path: P, buffer: &PixelBuffer<RGB>, format: Option<EncodingFormat>) -> Result<(), EncodingError> where P: AsRef<Path> {
    let format = match format {
        Some(format) => format,
        None => EncodingFormat::from_path(path.as_ref())?
    };

    let mut w = BufWriter::new(File::create(path)?);

    encode(&mut w, buffer, format)
}

pub fn encode<W>(w: &mut W, buffer: &PixelBuffer<RGB>, format: EncodingFormat) -> Result<(), EncodingError> where W: Write {
    match format {
        EncodingFormat::JPEG { quality } => encode_jpeg(w, buffer, quality),
        EncodingFormat::PNG { color_type, compression, filter } => encode_png(w, buffer, color_type, compression, filter),
        EncodingFormat::GIF => encode_gif(w, buffer),
        EncodingFormat::BMP { color_type } => encode_bmp(w, buffer, color_type),
        EncodingFormat::ICO { color_type } => encode_ico(w, buffer, color_type),
    }
}

