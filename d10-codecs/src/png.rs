use d10_core::pixelbuffer::{PixelBuffer, is_valid_buffer_size};
use d10_core::color::{Rgb, Srgb, Color};
use d10_core::errors::ParseEnumError;

use std::io::{Write, Seek, BufRead, Read};
use std::str::FromStr;

use crate::utils::*;
use crate::{DecodedImage, EncodingError, DecodingError};

use png::{Compression, FilterType};
use png::{ColorType, BitDepth, DecodingError as PngDecodingError, Encoder, EncodingError  as PngEncodingError, Decoder};

#[derive(Copy, Clone, Debug)]
pub enum PngColorType {
    L8,
    La8,
    L16,
    La16,
    Rgb8,
    Rgba8,
    Rgb16,
    Rgba16,
}

impl FromStr for PngColorType {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use PngColorType::*;
        match value {
            "l8" => Ok(L8),
            "la8" => Ok(La8),
            "l16" => Ok(L16),
            "la16" => Ok(La16),
            "rgb8" => Ok(Rgb8),
            "rgba8" => Ok(Rgba8),
            "rgb16" => Ok(Rgb16),
            "rgba16" => Ok(Rgba16),
            _ => Err(ParseEnumError::new(value, "PngColorType"))
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PngFilterType {
    NoFilter,
    Sub,
    Up,
    Avg,
    Paeth,
}

impl Into<FilterType> for PngFilterType {
    fn into(self) -> FilterType {
        match self {
            PngFilterType::NoFilter => FilterType::NoFilter,
            PngFilterType::Sub => FilterType::Sub,
            PngFilterType::Up => FilterType::Up,
            PngFilterType::Avg => FilterType::Avg,
            PngFilterType::Paeth => FilterType::Paeth,
        }
    }
}

impl FromStr for PngFilterType {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use PngFilterType::*;
        match value {
            "no_filter" => Ok(NoFilter),
            "sub" => Ok(Sub),
            "up" => Ok(Up),
            "avg" => Ok(Avg),
            "paeth" => Ok(Paeth),
            _ => Err(ParseEnumError::new(value, "PngFilterType"))
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PngCompression {
    Default,
    Fast,
    Best,
    Huffman,
    Rle,
}


impl Into<Compression> for PngCompression {
    fn into(self) -> Compression {
        match self {
            PngCompression::Default => Compression::Default,
            PngCompression::Fast => Compression::Fast,
            PngCompression::Best => Compression::Best,
            PngCompression::Huffman => Compression::Huffman,
            PngCompression::Rle => Compression::Rle,
        }
    }
}


impl FromStr for PngCompression {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use PngCompression::*;
        match value {
            "default" => Ok(Default),
            "fast" => Ok(Fast),
            "best" => Ok(Best),
            "huffman" => Ok(Huffman),
            "rle" => Ok(Rle),
            _ => Err(ParseEnumError::new(value, "PngCompression"))
        }
    }
}

fn encode_error(err: PngEncodingError) -> EncodingError {
    match err {
        PngEncodingError::IoError(err) => EncodingError::IoError(err),
        err => EncodingError::Encoding(err.to_string()),
    }
}

pub(crate) fn encode_png<W>(w: &mut W,
                            buffer: &PixelBuffer<Rgb>,
                            color_type: PngColorType,
                            compression: PngCompression,
                            filter: PngFilterType) -> Result<(), EncodingError>
    where W: Write {
    let (out, color_type, bit_depth) = match color_type {
        PngColorType::L8 => (to_l8_vec(buffer), ColorType::Grayscale, BitDepth::Eight),
        PngColorType::La8 => (to_la8_vec(buffer), ColorType::GrayscaleAlpha, BitDepth::Eight),
        PngColorType::L16 => (to_l16_be_vec(buffer), ColorType::Grayscale, BitDepth::Sixteen),
        PngColorType::La16 => (to_la16_be_vec(buffer), ColorType::GrayscaleAlpha, BitDepth::Sixteen),
        PngColorType::Rgb8 => (to_rgb8_vec(buffer), ColorType::RGB, BitDepth::Eight),
        PngColorType::Rgba8 => (to_rgba8_vec(buffer), ColorType::RGBA, BitDepth::Eight),
        PngColorType::Rgb16 => (to_rgb16_be_vec(buffer), ColorType::RGB, BitDepth::Sixteen),
        PngColorType::Rgba16 => (to_rgba16_be_vec(buffer), ColorType::RGBA, BitDepth::Sixteen),
    };

    let mut encoder = Encoder::new(w, buffer.width(), buffer.height());

    encoder.set_color(color_type);
    encoder.set_depth(bit_depth);
    encoder.set_compression(compression);
    encoder.set_filter(filter.into());

    let mut writer = encoder.write_header().map_err(encode_error)?;
    writer.write_image_data(&out).map_err(encode_error)?;

    Ok(())
}

fn decode_error(err: PngDecodingError) -> DecodingError {
    match err {
        PngDecodingError::IoError(err) => DecodingError::IoError(err),
        err => DecodingError::Decoding(err.to_string()),
    }
}

pub(crate) fn decode_png<T>(reader: T) -> Result<DecodedImage, DecodingError> where T: Read + Seek + BufRead {
    let mut decoder = Decoder::new(reader);
    decoder.set_transformations(png::Transformations::EXPAND);

    let (_, mut reader) = decoder
        .read_info()
        .map_err(decode_error)?;

    let (color_type, bits) = reader.output_color_type();
    let info = reader.info();

    let width = info.width;
    let height = info.height;

    if !is_valid_buffer_size(width, height) {
        return Err(DecodingError::InvalidBufferSize { width, height });
    }

    let num_pixel = (width * height) as usize;


    let raw: Vec<Rgb> = match (color_type, bits) {
        (ColorType::RGBA, BitDepth::Eight) => {
            let mut buffer = vec![0u8; num_pixel * 4];
            reader.next_frame(&mut buffer).map_err(decode_error)?;

            buffer.chunks(4).map(|chunks| {
                Srgb::new_with_alpha(from_u8(chunks[0]),
                                     from_u8(chunks[1]),
                                     from_u8(chunks[2]),
                                     from_u8(chunks[3]))
                    .to_rgb()
            }).collect()
        }
        (ColorType::RGB, BitDepth::Eight) => {
            let mut buffer = vec![0u8; num_pixel * 3];
            reader.next_frame(&mut buffer).map_err(decode_error)?;

            buffer.chunks(3).map(|chunks| {
                Srgb::new(from_u8(chunks[0]),
                          from_u8(chunks[1]),
                          from_u8(chunks[2]))
                    .to_rgb()
            }).collect()
        }
        (ColorType::Grayscale, BitDepth::Eight) => {
            let mut buffer = vec![0u8; num_pixel];
            reader.next_frame(&mut buffer).map_err(decode_error)?;

            buffer.iter().map(|v| {
                Srgb::new(from_u8(*v),
                          from_u8(*v),
                          from_u8(*v))
                    .to_rgb()
            }).collect()
        }
        (ColorType::GrayscaleAlpha, BitDepth::Eight) => {
            let mut buffer = vec![0u8; num_pixel * 2];
            reader.next_frame(&mut buffer).map_err(decode_error)?;

            buffer.chunks(2).map(|chunks| {
                Srgb::new_with_alpha(from_u8(chunks[0]),
                                     from_u8(chunks[0]),
                                     from_u8(chunks[0]),
                                     from_u8(chunks[1]))
                    .to_rgb()
            }).collect()
        }
        (ColorType::RGBA, BitDepth::Sixteen) => {
            let mut buffer = vec![0u8; num_pixel * 8];
            reader.next_frame(&mut buffer).map_err(decode_error)?;

            buffer.chunks(8).map(|chunks| {
                Srgb::new_with_alpha(from_u16_be([chunks[0], chunks[1]]),
                                     from_u16_be([chunks[2], chunks[3]]),
                                     from_u16_be([chunks[4], chunks[5]]),
                                     from_u16_be([chunks[6], chunks[7]]))
                    .to_rgb()
            }).collect()
        }
        (ColorType::RGB, BitDepth::Sixteen) => {
            let mut buffer = vec![0u8; num_pixel * 6];
            reader.next_frame(&mut buffer).map_err(decode_error)?;

            buffer.chunks(6).map(|chunks| {
                Srgb::new(from_u16_be([chunks[0], chunks[1]]),
                          from_u16_be([chunks[2], chunks[3]]),
                          from_u16_be([chunks[4], chunks[5]]))
                    .to_rgb()
            }).collect()
        }
        (ColorType::Grayscale, BitDepth::Sixteen) => {
            let mut buffer = vec![0u8; num_pixel * 2];
            reader.next_frame(&mut buffer).map_err(decode_error)?;

            buffer.chunks(2).map(|chunks| {
                Srgb::new(from_u16_be([chunks[0], chunks[1]]),
                          from_u16_be([chunks[0], chunks[1]]),
                          from_u16_be([chunks[0], chunks[1]]))
                    .to_rgb()
            }).collect()
        }
        (ColorType::GrayscaleAlpha, BitDepth::Sixteen) => {
            let mut buffer = vec![0u8; num_pixel * 4];
            reader.next_frame(&mut buffer).map_err(decode_error)?;

            buffer.chunks(4).map(|chunks| {
                Srgb::new_with_alpha(from_u16_be([chunks[0], chunks[1]]),
                                     from_u16_be([chunks[0], chunks[1]]),
                                     from_u16_be([chunks[0], chunks[1]]),
                                     from_u16_be([chunks[2], chunks[3]]))
                    .to_rgb()
            }).collect()
        }
        _ => return Err(DecodingError::Decoding(format!("Unsupported png: {:?}:{:?}", color_type, bits)))
    };

    Ok(DecodedImage {
        buffer: PixelBuffer::new_from_raw(width, height, raw)
    })
}