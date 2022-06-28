use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Cursor, Read, Seek, SeekFrom, Write};
use std::path::Path;

use d10_core::color::Rgb;
use d10_core::pixelbuffer::PixelBuffer;

pub use crate::bmp::BmpColorType;
use crate::bmp::{decode_bmp, encode_bmp};
pub use crate::errors::*;
use crate::gif::{decode_gif, encode_gif};
pub use crate::ico::IcoColorType;
use crate::ico::{decode_ico, encode_ico};
pub use crate::jpeg::JpegSamplingFactor;
use crate::jpeg::{decode_jpeg, encode_jpeg};
use crate::png::{decode_png, encode_png};
pub use crate::png::{PngColorType, PngCompression, PngFilterType};
pub use crate::webp::WebPPreset;
use crate::webp::{decode_webp, encode_webp};

mod bmp;
mod errors;
mod gif;
mod ico;
mod jpeg;
mod png;
mod utils;
mod webp;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Format {
    Jpeg,
    Png,
    Gif,
    Bmp,
    Ico,
    WebP,
}

impl Format {
    pub fn from_path(path: &Path) -> Option<Format> {
        let ext = path.extension()?.to_string_lossy().to_ascii_lowercase();

        match ext.as_str() {
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "png" => Some(Self::Png),
            "gif" => Some(Self::Gif),
            "bmp" => Some(Self::Bmp),
            "ico" => Some(Self::Ico),
            "webp" => Some(Self::WebP),
            _ => None,
        }
    }

    pub fn from_reader<T>(reader: &mut T) -> Result<Format, DecodingError>
    where
        T: Read + Seek,
    {
        let mut buf = [0u8; 12];

        let len = reader.read(&mut buf)?;

        reader.seek(SeekFrom::Start(0))?;

        match buf[0..len] {
            [0xFF, 0xD8, 0xFF, ..] => Ok(Format::Jpeg),
            [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, ..] => Ok(Format::Png),
            [0x47, 0x49, 0x46, 0x38, 0x37, 0x61, ..] => Ok(Format::Gif),
            [0x47, 0x49, 0x46, 0x38, 0x39, 0x61, ..] => Ok(Format::Gif),
            [0x42, 0x4D, ..] => Ok(Format::Bmp),
            [0x00, 0x00, 0x01, 0x00, ..] => Ok(Format::Ico),
            [b'R', b'I', b'F', b'F', _, _, _, _, b'W', b'E', b'B', b'P'] => Ok(Format::WebP),

            _ => Err(DecodingError::UnknownFormat),
        }
    }
}

#[derive(Clone, Debug)]
pub enum EncodingFormat {
    Jpeg {
        quality: u8,
        progressive: bool,
        sampling_factor: Option<JpegSamplingFactor>,
        grayscale: bool,
        optimize_huffman_tables: bool,
    },
    Png {
        color_type: PngColorType,
        compression: PngCompression,
        filter: PngFilterType,
    },
    Gif,
    Bmp {
        color_type: BmpColorType,
    },
    Ico {
        color_type: IcoColorType,
    },
    WebP {
        quality: u8,
        preset: WebPPreset,
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
            EncodingFormat::WebP { .. } => Format::WebP,
        }
    }

    pub fn jpeg_default() -> Self {
        Self::Jpeg {
            quality: 85,
            progressive: false,
            sampling_factor: None,
            grayscale: false,
            optimize_huffman_tables: true,
        }
    }

    pub fn jpeg_with_quality(quality: u8) -> Self {
        Self::Jpeg {
            quality,
            progressive: false,
            sampling_factor: None,
            grayscale: false,
            optimize_huffman_tables: true,
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
            color_type: BmpColorType::Rgba8,
        }
    }

    pub fn ico_default() -> Self {
        Self::Ico {
            color_type: IcoColorType::Rgba8,
        }
    }

    pub fn webp_default() -> Self {
        Self::WebP {
            quality: 90,
            preset: WebPPreset::Default,
        }
    }

    pub fn webp_with_quality(quality: u8) -> Self {
        Self::WebP {
            quality,
            preset: WebPPreset::Default,
        }
    }

    pub fn webp_with_preset(quality: u8, preset: WebPPreset) -> Self {
        Self::WebP { quality, preset }
    }

    pub fn from_path(path: &Path) -> Result<EncodingFormat, EncodingError> {
        match Format::from_path(path) {
            Some(Format::Jpeg) => Ok(EncodingFormat::jpeg_default()),
            Some(Format::Png) => Ok(EncodingFormat::png_default()),
            Some(Format::Gif) => Ok(EncodingFormat::gif_default()),
            Some(Format::Bmp) => Ok(EncodingFormat::bmp_default()),
            Some(Format::Ico) => Ok(EncodingFormat::ico_default()),
            Some(Format::WebP) => Ok(EncodingFormat::webp_default()),
            None => Err(EncodingError::BadFileExtension(
                path.to_string_lossy().to_string(),
            )),
        }
    }
}

pub struct DecodedImage {
    pub buffer: PixelBuffer<Rgb>,
}

pub fn decode_file<P>(path: P) -> Result<DecodedImage, DecodingError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    if let Ok(format) = Format::from_reader(&mut reader) {
        decode(reader, format)
    } else if let Some(format) = Format::from_path(path) {
        decode(reader, format)
    } else {
        Err(DecodingError::UnknownFormat)
    }
}

pub fn decode_buffer(buffer: &[u8]) -> Result<DecodedImage, DecodingError> {
    let mut reader = Cursor::new(buffer);
    let format = Format::from_reader(&mut reader)?;

    decode(reader, format)
}

fn decode<T>(reader: T, format: Format) -> Result<DecodedImage, DecodingError>
where
    T: Read + Seek + BufRead,
{
    match format {
        Format::Jpeg => decode_jpeg(reader),
        Format::Png => decode_png(reader),
        Format::Gif => decode_gif(reader),
        Format::Bmp => decode_bmp(reader),
        Format::Ico => decode_ico(reader),
        Format::WebP => decode_webp(reader),
    }
}

pub fn encode_to_file<P>(
    path: P,
    buffer: &PixelBuffer<Rgb>,
    format: Option<EncodingFormat>,
) -> Result<(), EncodingError>
where
    P: AsRef<Path>,
{
    let format = match format {
        Some(format) => format,
        None => EncodingFormat::from_path(path.as_ref())?,
    };

    let mut w = BufWriter::new(File::create(path)?);

    encode(&mut w, buffer, format)
}

pub fn encode<W>(
    w: W,
    buffer: &PixelBuffer<Rgb>,
    format: EncodingFormat,
) -> Result<(), EncodingError>
where
    W: Write,
{
    match format {
        EncodingFormat::Jpeg {
            quality,
            progressive,
            sampling_factor,
            grayscale,
            optimize_huffman_tables,
        } => encode_jpeg(
            w,
            buffer,
            quality,
            progressive,
            sampling_factor,
            grayscale,
            optimize_huffman_tables,
        ),
        EncodingFormat::Png {
            color_type,
            compression,
            filter,
        } => encode_png(w, buffer, color_type, compression, filter),
        EncodingFormat::Gif => encode_gif(w, buffer),
        EncodingFormat::Bmp { color_type } => encode_bmp(w, buffer, color_type),
        EncodingFormat::Ico { color_type } => encode_ico(w, buffer, color_type),
        EncodingFormat::WebP { quality, preset } => encode_webp(w, buffer, quality, preset),
    }
}
