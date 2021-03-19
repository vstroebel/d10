mod utils;
mod jpeg;
mod png;
mod gif;
mod bmp;
mod ico;

use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;
use d10_core::errors::{D10Result, D10Error};

use std::path::Path;
use std::io::{Cursor, Read, Seek, Write, SeekFrom, BufReader, BufRead, BufWriter};
use std::fs::File;

pub use crate::png::{PNGColorType, PNGCompressionType, PNGFilterType};
pub use crate::bmp::BMPColorType;
pub use crate::ico::ICOColorType;

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
    pub fn from_path(path: &Path) -> D10Result<Format> {
        let ext = path
            .extension()
            .ok_or_else(|| D10Error::SaveError(format!("Missing file extension in path: {}", path.to_string_lossy())))?
            .to_string_lossy()
            .to_ascii_lowercase();

        match ext.as_str() {
            "jpg" | "jpeg" => Ok(Self::JPEG),
            "png" => Ok(Self::PNG),
            "gif" => Ok(Self::GIF),
            "bmp" => Ok(Self::BMP),
            "ico" => Ok(Self::ICO),
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
            [0x47, 0x49, 0x46, 0x38, 0x37, 0x61, ..] => Ok(Format::GIF),
            [0x47, 0x49, 0x46, 0x38, 0x39, 0x61, ..] => Ok(Format::GIF),
            [0x42, 0x4D, ..] => Ok(Format::BMP),
            [0x00, 0x00, 0x01, 0x00, ..] => Ok(Format::ICO),
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
            compression: PNGCompressionType::Default,
            filter: PNGFilterType::Sub,
        }
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

    pub fn from_path(path: &Path) -> D10Result<EncodingFormat> {
        match Format::from_path(path)? {
            Format::JPEG => Ok(EncodingFormat::jpeg_default()),
            Format::PNG => Ok(EncodingFormat::png_default()),
            Format::GIF => Ok(EncodingFormat::GIF),
            Format::BMP => Ok(EncodingFormat::bmp_default()),
            Format::ICO => Ok(EncodingFormat::ico_default()),
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
        Format::PNG => decode_png(reader),
        Format::GIF => decode_gif(reader),
        Format::BMP => decode_bmp(reader),
        Format::ICO => decode_ico(reader),
    }
}

pub fn encode_to_file<P>(path: P, buffer: &PixelBuffer<RGB>, format: Option<EncodingFormat>) -> D10Result<()> where P: AsRef<Path> {
    let format = match format {
        Some(format) => format,
        None => EncodingFormat::from_path(path.as_ref())?
    };

    let mut w = BufWriter::new(File::create(path)?);

    encode(&mut w, buffer, format)
}

pub fn encode<W>(w: &mut W, buffer: &PixelBuffer<RGB>, format: EncodingFormat) -> D10Result<()> where W: Write {
    match format {
        EncodingFormat::JPEG { quality } => encode_jpeg(w, buffer, quality),
        EncodingFormat::PNG { color_type, compression, filter } => encode_png(w, buffer, color_type, compression, filter),
        EncodingFormat::GIF => encode_gif(w, buffer),
        EncodingFormat::BMP { color_type } => encode_bmp(w, buffer, color_type),
        EncodingFormat::ICO { color_type } => encode_ico(w, buffer, color_type),
    }
}

