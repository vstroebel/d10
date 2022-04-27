use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::{Color, Rgb, Srgb};
use d10_core::errors::ParseEnumError;

use std::io::{Write, Read, Seek, BufRead};
use std::str::FromStr;

use jpeg_encoder::{Encoder, SamplingFactor, ColorType, EncodingError as JpegEncodingError};
use jpeg_decoder::{Decoder, PixelFormat, Error as DecoderError};

use crate::utils::{to_rgb8_vec, to_l8_vec, from_u8, cmyk_to_rgb, from_u16_ne};
use crate::{DecodedImage, EncodingError, DecodingError};


#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum JpegSamplingFactor {
    /// 1x1
    F_1_1,

    /// 2x1
    F_2_1,

    /// 1x2
    F_1_2,

    /// 2x2
    F_2_2,

    /// 4x1 (not supported by all decoders)
    F_4_1,

    /// 4x2 (not supported by all decoders)
    F_4_2,

    /// 1x4 (not supported by all decoders)
    F_1_4,

    /// 2x4 (not supported by all decoders)
    F_2_4,

    /// Alias for F_1_1
    R_4_4_4,

    /// Alias for F_1_2
    R_4_4_0,

    /// Alias for F_1_4
    R_4_4_1,

    /// Alias for F_2_1
    R_4_2_2,

    /// Alias for F_2_2
    R_4_2_0,

    /// Alias for F_2_4
    R_4_2_1,

    /// Alias for F_4_1
    R_4_1_1,

    /// Alias for F_4_2
    R_4_1_0,
}

impl FromStr for JpegSamplingFactor {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use JpegSamplingFactor::*;
        match value {
            "1x1" => Ok(F_1_1),
            "1x2" => Ok(F_1_2),
            "2x1" => Ok(F_2_1),
            "2x2" => Ok(F_2_2),
            "4x1" => Ok(F_4_1),
            "4x2" => Ok(F_4_2),
            "1x4" => Ok(F_1_4),
            "2x4" => Ok(F_2_4),
            "4:4:4" => Ok(R_4_4_4),
            "4:4:0" => Ok(R_4_4_0),
            "4:4:1" => Ok(R_4_4_1),
            "4:2:2" => Ok(R_4_2_2),
            "4:2:0" => Ok(R_4_2_0),
            "4:2:1" => Ok(R_4_2_1),
            "4:1:1" => Ok(R_4_1_1),
            "4:1:0" => Ok(R_4_1_0),
            _ => Err(ParseEnumError::new(value, "JpegSamplingFactor"))
        }
    }
}

impl From<JpegSamplingFactor> for SamplingFactor {
    fn from(f: JpegSamplingFactor) -> SamplingFactor {
        match f {
            JpegSamplingFactor::F_1_1 => SamplingFactor::F_1_1,
            JpegSamplingFactor::F_2_1 => SamplingFactor::F_2_1,
            JpegSamplingFactor::F_1_2 => SamplingFactor::F_1_2,
            JpegSamplingFactor::F_2_2 => SamplingFactor::F_2_2,
            JpegSamplingFactor::F_4_1 => SamplingFactor::F_4_1,
            JpegSamplingFactor::F_4_2 => SamplingFactor::F_4_2,
            JpegSamplingFactor::F_1_4 => SamplingFactor::F_1_4,
            JpegSamplingFactor::F_2_4 => SamplingFactor::F_2_4,
            JpegSamplingFactor::R_4_4_4 => SamplingFactor::R_4_4_4,
            JpegSamplingFactor::R_4_4_0 => SamplingFactor::R_4_4_0,
            JpegSamplingFactor::R_4_4_1 => SamplingFactor::R_4_4_1,
            JpegSamplingFactor::R_4_2_2 => SamplingFactor::R_4_2_2,
            JpegSamplingFactor::R_4_2_0 => SamplingFactor::R_4_2_0,
            JpegSamplingFactor::R_4_2_1 => SamplingFactor::R_4_2_1,
            JpegSamplingFactor::R_4_1_1 => SamplingFactor::R_4_1_1,
            JpegSamplingFactor::R_4_1_0 => SamplingFactor::R_4_1_0,
        }
    }
}

pub(crate) fn encode_jpeg<W>(w: W,
                             buffer: &PixelBuffer<Rgb>,
                             quality: u8,
                             progressive: bool,
                             sampling_factor: Option<JpegSamplingFactor>,
                             grayscale: bool,
                             optimize_huffman_tables: bool) -> Result<(), EncodingError> where W: Write {
    let width = buffer.width();
    let height = buffer.height();

    if width > u16::MAX as u32 || height > u16::MAX as u32 {
        return Err(EncodingError::BadDimensions {
            format: "jpeg",
            width,
            height,
        });
    }

    let (out, color_type) = if grayscale {
        (to_l8_vec(buffer), ColorType::Luma)
    } else {
        (to_rgb8_vec(buffer), ColorType::Rgb)
    };

    // Ensure quality is always in the valid range.
    let quality = quality.clamp(1, 100);

    let mut encoder = Encoder::new(w, quality);

    if let Some(sampling_factor) = sampling_factor {
        encoder.set_sampling_factor(sampling_factor.into());
    }

    if progressive {
        encoder.set_progressive(true);
    }

    if optimize_huffman_tables {
        encoder.set_optimized_huffman_tables(true);
    }

    if let Err(err) = encoder.encode(
        &out,
        width as u16,
        height as u16,
        color_type) {
        Err(match err {
            JpegEncodingError::IoError(err) => EncodingError::IoError(err),
            err => EncodingError::Encoding(err.to_string())
        })
    } else {
        Ok(())
    }
}

pub(crate) fn decode_jpeg<T>(reader: T) -> Result<DecodedImage, DecodingError> where T: Read + Seek + BufRead {
    let mut decoder = Decoder::new(reader);

    let data = decoder.decode().map_err(|err| match err {
        DecoderError::Io(err) => DecodingError::IoError(err),
        err => DecodingError::Decoding(err.to_string()),
    })?;

    let info = decoder.info().ok_or_else(|| DecodingError::Decoding("Missing jpeg info".to_owned()))?;

    let width = info.width as u32;
    let height = info.height as u32;

    let data = match info.pixel_format {
        PixelFormat::L8 => {
            data.iter().map(|v| {
                Srgb::new(from_u8(*v),
                          from_u8(*v),
                          from_u8(*v))
                    .to_rgb()
            }).collect()
        }
        PixelFormat::L16 => {
            data.chunks(2).map(|chunks| {
                let v = from_u16_ne([chunks[0], chunks[1]]);
                Srgb::new(v, v, v).to_rgb()
            }).collect()
        }
        PixelFormat::RGB24 => {
            data.chunks(3).map(|chunks| {
                Srgb::new(from_u8(chunks[0]),
                          from_u8(chunks[1]),
                          from_u8(chunks[2]))
                    .to_rgb()
            }).collect()
        }
        PixelFormat::CMYK32 => {
            data.chunks(4).map(|chunks| {
                cmyk_to_rgb(chunks[0], chunks[1], chunks[2], chunks[3])
            }).collect()
        }
    };

    Ok(DecodedImage {
        buffer: PixelBuffer::new_from_raw(width, height, data)
    })
}