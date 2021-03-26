mod utils;
mod jpeg;
mod png;
mod gif;
mod bmp;
mod ico;
mod errors;

use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::Rgb;

use std::path::Path;
use std::io::{Cursor, Read, Seek, Write, SeekFrom, BufReader, BufRead, BufWriter};
use std::fs::File;

pub use crate::png::{PngColorType, PngCompression, PngFilterType};
pub use crate::bmp::BmpColorType;
pub use crate::ico::IcoColorType;
pub use crate::errors::*;

use crate::jpeg::{decode_jpeg, encode_jpeg};
use crate::png::{decode_png, encode_png};
use crate::gif::{decode_gif, encode_gif};
use crate::bmp::{decode_bmp, encode_bmp};
use crate::ico::{decode_ico, encode_ico};

#[derive(Debug)]
pub enum Format {
    Jpeg,
    Png,
    Gif,
    Bmp,
    Ico,
}

impl Format {
    pub fn from_path(path: &Path) -> Option<Format> {
        let ext = path
            .extension()?
            .to_string_lossy()
            .to_ascii_lowercase();

        match ext.as_str() {
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "png" => Some(Self::Png),
            "gif" => Some(Self::Gif),
            "bmp" => Some(Self::Bmp),
            "ico" => Some(Self::Ico),
            _ => None,
        }
    }

    pub fn from_reader<T>(reader: &mut T) -> Result<Format, DecodingError> where T: Read + Seek {
        let mut buf = [0u8; 8];

        let len = reader.read(&mut buf)?;

        reader.seek(SeekFrom::Start(0))?;

        match buf[0..len] {
            [0xFF, 0xD8, 0xFF, ..] => Ok(Format::Jpeg),
            [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, ..] => Ok(Format::Png),
            [0x47, 0x49, 0x46, 0x38, 0x37, 0x61, ..] => Ok(Format::Gif),
            [0x47, 0x49, 0x46, 0x38, 0x39, 0x61, ..] => Ok(Format::Gif),
            [0x42, 0x4D, ..] => Ok(Format::Bmp),
            [0x00, 0x00, 0x01, 0x00, ..] => Ok(Format::Ico),
            _ => Err(DecodingError::UnknownFormat),
        }
    }
}

#[derive(Clone, Debug)]
pub enum EncodingFormat {
    Jpeg {
        quality: u8,
        grayscale: bool,
    },
    Png {
        color_type: PngColorType,
        compression: PngCompression,
        filter: PngFilterType,
    },
    Gif,
    Bmp {
        color_type: BmpColorType
    },
    Ico {
        color_type: IcoColorType
    },
}

impl EncodingFormat {
    pub fn format(&self) -> Format {
        match self {
            EncodingFormat::Jpeg { .. } => Format::Jpeg,
            EncodingFormat::Png { .. } => Format::Png,
            EncodingFormat::Gif => Format::Gif,
            EncodingFormat::Bmp { .. } => Format::Bmp,
            EncodingFormat::Ico { .. } => Format::Ico,
        }
    }

    pub fn jpeg_default() -> Self {
        Self::Jpeg {
            quality: 85,
            grayscale: false,
        }
    }

    pub fn png_default() -> Self {
        Self::Png {
            color_type: PngColorType::Rgba8,
            compression: PngCompression::Default,
            filter: PngFilterType::Sub,
        }
    }

    pub fn gif_default() -> Self {
        Self::Gif
    }

    pub fn bmp_default() -> Self {
        Self::Bmp {
            color_type: BmpColorType::Rgba8
        }
    }

    pub fn ico_default() -> Self {
        Self::Ico {
            color_type: IcoColorType::Rgba8
        }
    }

    pub fn from_path(path: &Path) -> Result<EncodingFormat, EncodingError> {
        match Format::from_path(path) {
            Some(Format::Jpeg) => Ok(EncodingFormat::jpeg_default()),
            Some(Format::Png) => Ok(EncodingFormat::png_default()),
            Some(Format::Gif) => Ok(EncodingFormat::gif_default()),
            Some(Format::Bmp) => Ok(EncodingFormat::bmp_default()),
            Some(Format::Ico) => Ok(EncodingFormat::ico_default()),
            None => Err(EncodingError::BadFileExtension(path.to_string_lossy().to_string()))
        }
    }
}

pub struct DecodedImage {
    pub buffer: PixelBuffer<Rgb>
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
        Format::Jpeg => decode_jpeg(reader),
        Format::Png => decode_png(reader),
        Format::Gif => decode_gif(reader),
        Format::Bmp => decode_bmp(reader),
        Format::Ico => decode_ico(reader),
    }
}

pub fn encode_to_file<P>(path: P, buffer: &PixelBuffer<Rgb>, format: Option<EncodingFormat>) -> Result<(), EncodingError> where P: AsRef<Path> {
    let format = match format {
        Some(format) => format,
        None => EncodingFormat::from_path(path.as_ref())?
    };

    let mut w = BufWriter::new(File::create(path)?);

    encode(&mut w, buffer, format)
}

pub fn encode<W>(w: &mut W, buffer: &PixelBuffer<Rgb>, format: EncodingFormat) -> Result<(), EncodingError> where W: Write {
    match format {
        EncodingFormat::Jpeg { quality, grayscale } => encode_jpeg(w, buffer, quality, grayscale),
        EncodingFormat::Png { color_type, compression, filter } => encode_png(w, buffer, color_type, compression, filter),
        EncodingFormat::Gif => encode_gif(w, buffer),
        EncodingFormat::Bmp { color_type } => encode_bmp(w, buffer, color_type),
        EncodingFormat::Ico { color_type } => encode_ico(w, buffer, color_type),
    }
}

