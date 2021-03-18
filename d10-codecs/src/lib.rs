mod utils;
mod jpeg;
mod png;

use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;
use d10_core::errors::{D10Result, D10Error};

use std::path::Path;
use std::io::{Cursor, Read, Seek, Write, SeekFrom, BufReader, BufRead};
use std::fs::File;

pub use crate::png::{PNGColorType, PNGCompressionType, PNGFilterType};
use crate::jpeg::decode_jpeg;
use crate::png::decode_png;

pub enum Format {
    JPEG,
    PNG,
}

impl Format {
    pub fn from_path(path: &Path) -> D10Result<Format> {
        let ext = path
            .extension()
            .ok_or_else(|| D10Error::SaveError(format!("Missing file extension in path: {}", path.to_string_lossy())))?
            .to_string_lossy()
            .to_ascii_lowercase();

        match ext.as_str() {
            "jpg" | "jpeg" => Ok(Self::JPEG),
            "png" => Ok(Self::PNG),
            _ => Err(D10Error::SaveError(format!("Unknown file extension in path: {}", path.to_string_lossy())))
        }
    }

    pub fn from_reader<T>(reader: &mut T) -> D10Result<Format> where T: Read + Seek {
        let mut buf = [0u8; 8];

        let len = reader.read(&mut buf)?;

        reader.seek(SeekFrom::Start(0))?;

        match buf[0..len] {
            [0xFF, 0xD8, 0xFF, ..] => Ok(Format::JPEG),
            [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, ..] => Ok(Format::PNG),
            _ => Err(D10Error::OpenError("Unable to detect format".to_owned())),
        }
    }
}

pub enum EncodingFormat {
    JPEG {
        quality: u8
    },
    PNG {
        color_type: PNGColorType,
        compression: PNGCompressionType,
        filter: PNGFilterType,
    },
}

impl EncodingFormat {
    pub fn format(&self) -> Format {
        match self {
            EncodingFormat::JPEG { .. } => Format::JPEG,
            EncodingFormat::PNG { .. } => Format::PNG,
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
            compression: PNGCompressionType::Default,
            filter: PNGFilterType::Sub,
        }
    }

    pub fn from_path(path: &Path) -> D10Result<EncodingFormat> {
        match Format::from_path(path)? {
            Format::JPEG => Ok(EncodingFormat::jpeg_default()),
            Format::PNG => Ok(EncodingFormat::png_default())
        }
    }
}

pub struct DecodedImage {
    pub buffer: PixelBuffer<RGB>
}

pub fn decode_file<P>(path: P) -> D10Result<DecodedImage> where P: AsRef<Path> {
    let format = Format::from_path(path.as_ref())?;

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    decode(reader, format)
}

pub fn decode_buffer(buffer: &[u8]) -> D10Result<DecodedImage> {
    let mut reader = Cursor::new(buffer);
    let format = Format::from_reader(&mut reader)?;

    decode(reader, format)
}

fn decode<T>(reader: T, format: Format) -> D10Result<DecodedImage> where T: Read + Seek + BufRead {
    match format {
        Format::JPEG => decode_jpeg(reader),
        Format::PNG => decode_png(reader)
    }
}

pub fn save_to_file<P>(path: P, buffer: &PixelBuffer<RGB>, format: Option<EncodingFormat>) -> D10Result<()> where P: AsRef<Path> {
    let format = match format {
        Some(format) => format,
        None => EncodingFormat::from_path(path.as_ref())?
    };

    match format {
        EncodingFormat::JPEG { quality } => jpeg::save_jpeg(&mut File::create(path)?, buffer, quality),
        EncodingFormat::PNG { color_type, compression, filter } => png::save_png(&mut File::create(path)?, buffer, color_type, compression, filter),
    }
}

pub fn save<W>(w: &mut W, buffer: &PixelBuffer<RGB>, format: EncodingFormat) -> D10Result<()> where W: Write {
    match format {
        EncodingFormat::JPEG { quality } => jpeg::save_jpeg(w, buffer, quality),
        EncodingFormat::PNG { color_type, compression, filter } => png::save_png(w, buffer, color_type, compression, filter),
    }
}

