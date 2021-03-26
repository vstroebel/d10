use d10_core::pixelbuffer::{PixelBuffer, is_valid_buffer_size};
use d10_core::color::{RGB, SRGB, Color};
use d10_core::errors::ParseEnumError;

use std::io::{Write, Seek, BufRead, Read};
use std::str::FromStr;

use crate::utils::*;
use crate::{DecodedImage, EncodingError, DecodingError};

use png::{Compression, FilterType};
use png::{ColorType, BitDepth, DecodingError as PNGDecodingError, Encoder, EncodingError  as PNGEncodingError, Decoder};

#[derive(Copy, Clone, Debug)]
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

impl FromStr for PNGColorType {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use PNGColorType::*;
        match value {
            "l8" => Ok(L8),
            "la8" => Ok(LA8),
            "l16" => Ok(L16),
            "la16" => Ok(LA16),
            "rgb8" => Ok(RGB8),
            "rgba8" => Ok(RGBA8),
            "rgb16" => Ok(RGB16),
            "rgba16" => Ok(RGBA16),
            _ => Err(ParseEnumError::new(value, "PNGColorType"))
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PNGFilterType {
    NoFilter,
    Sub,
    Up,
    Avg,
    Paeth,
}

impl Into<FilterType> for PNGFilterType {
    fn into(self) -> FilterType {
        match self {
            PNGFilterType::NoFilter => FilterType::NoFilter,
            PNGFilterType::Sub => FilterType::Sub,
            PNGFilterType::Up => FilterType::Up,
            PNGFilterType::Avg => FilterType::Avg,
            PNGFilterType::Paeth => FilterType::Paeth,
        }
    }
}

impl FromStr for PNGFilterType {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use PNGFilterType::*;
        match value {
            "no_filter" => Ok(NoFilter),
            "sub" => Ok(Sub),
            "up" => Ok(Up),
            "avg" => Ok(Avg),
            "paeth" => Ok(Paeth),
            _ => Err(ParseEnumError::new(value, "PNGFilterType"))
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PNGCompression {
    Default,
    Fast,
    Best,
    Huffman,
    Rle,
}


impl Into<Compression> for PNGCompression {
    fn into(self) -> Compression {
        match self {
            PNGCompression::Default => Compression::Default,
            PNGCompression::Fast => Compression::Fast,
            PNGCompression::Best => Compression::Best,
            PNGCompression::Huffman => Compression::Huffman,
            PNGCompression::Rle => Compression::Rle,
        }
    }
}


impl FromStr for PNGCompression {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use PNGCompression::*;
        match value {
            "default" => Ok(Default),
            "fast" => Ok(Fast),
            "best" => Ok(Best),
            "huffman" => Ok(Huffman),
            "rle" => Ok(Rle),
            _ => Err(ParseEnumError::new(value, "PNGCompression"))
        }
    }
}

fn encode_error(err: PNGEncodingError) -> EncodingError {
    match err {
        PNGEncodingError::IoError(err) => EncodingError::IOError(err),
        err => EncodingError::Encoding(err.to_string()),
    }
}

pub(crate) fn encode_png<W>(w: &mut W,
                            buffer: &PixelBuffer<RGB>,
                            color_type: PNGColorType,
                            compression: PNGCompression,
                            filter: PNGFilterType) -> Result<(), EncodingError>
    where W: Write {
    let (out, color_type, bit_depth) = match color_type {
        PNGColorType::L8 => (to_l8_vec(buffer), ColorType::Grayscale, BitDepth::Eight),
        PNGColorType::LA8 => (to_la8_vec(buffer), ColorType::GrayscaleAlpha, BitDepth::Eight),
        PNGColorType::L16 => (to_l16_be_vec(buffer), ColorType::Grayscale, BitDepth::Sixteen),
        PNGColorType::LA16 => (to_la16_be_vec(buffer), ColorType::GrayscaleAlpha, BitDepth::Sixteen),
        PNGColorType::RGB8 => (to_rgb8_vec(buffer), ColorType::RGB, BitDepth::Eight),
        PNGColorType::RGBA8 => (to_rgba8_vec(buffer), ColorType::RGBA, BitDepth::Eight),
        PNGColorType::RGB16 => (to_rgb16_be_vec(buffer), ColorType::RGB, BitDepth::Sixteen),
        PNGColorType::RGBA16 => (to_rgba16_be_vec(buffer), ColorType::RGBA, BitDepth::Sixteen),
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

fn decode_error(err: PNGDecodingError) -> DecodingError {
    match err {
        PNGDecodingError::IoError(err) => DecodingError::IOError(err),
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


    let raw: Vec<RGB> = match (color_type, bits) {
        (ColorType::RGBA, BitDepth::Eight) => {
            let mut buffer = vec![0u8; num_pixel * 4];
            reader.next_frame(&mut buffer).map_err(decode_error)?;

            buffer.chunks(4).map(|chunks| {
                SRGB::new_with_alpha(from_u8(chunks[0]),
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
                SRGB::new(from_u8(chunks[0]),
                          from_u8(chunks[1]),
                          from_u8(chunks[2]))
                    .to_rgb()
            }).collect()
        }
        (ColorType::Grayscale, BitDepth::Eight) => {
            let mut buffer = vec![0u8; num_pixel];
            reader.next_frame(&mut buffer).map_err(decode_error)?;

            buffer.iter().map(|v| {
                SRGB::new(from_u8(*v),
                          from_u8(*v),
                          from_u8(*v))
                    .to_rgb()
            }).collect()
        }
        (ColorType::GrayscaleAlpha, BitDepth::Eight) => {
            let mut buffer = vec![0u8; num_pixel * 2];
            reader.next_frame(&mut buffer).map_err(decode_error)?;

            buffer.chunks(2).map(|chunks| {
                SRGB::new_with_alpha(from_u8(chunks[0]),
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
                SRGB::new_with_alpha(from_u16_be([chunks[0], chunks[1]]),
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
                SRGB::new(from_u16_be([chunks[0], chunks[1]]),
                          from_u16_be([chunks[2], chunks[3]]),
                          from_u16_be([chunks[4], chunks[5]]))
                    .to_rgb()
            }).collect()
        }
        (ColorType::Grayscale, BitDepth::Sixteen) => {
            let mut buffer = vec![0u8; num_pixel * 2];
            reader.next_frame(&mut buffer).map_err(decode_error)?;

            buffer.chunks(2).map(|chunks| {
                SRGB::new(from_u16_be([chunks[0], chunks[1]]),
                          from_u16_be([chunks[0], chunks[1]]),
                          from_u16_be([chunks[0], chunks[1]]))
                    .to_rgb()
            }).collect()
        }
        (ColorType::GrayscaleAlpha, BitDepth::Sixteen) => {
            let mut buffer = vec![0u8; num_pixel * 4];
            reader.next_frame(&mut buffer).map_err(decode_error)?;

            buffer.chunks(4).map(|chunks| {
                SRGB::new_with_alpha(from_u16_be([chunks[0], chunks[1]]),
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