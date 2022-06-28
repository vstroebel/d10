use d10_core::color::{Color, Rgb, Srgb};
use d10_core::pixelbuffer::{is_valid_buffer_size, PixelBuffer};

use std::io::{BufRead, Read, Seek, Write};

use crate::utils::{from_u8, to_rgba8_vec};
use crate::{DecodedImage, DecodingError, EncodingError};

use gif::{
    DecodeOptions, DecodingError as GIFDecodingError, Encoder, EncodingError as GIFEncodingError,
    Frame,
};

fn encode_error(err: GIFEncodingError) -> EncodingError {
    match err {
        GIFEncodingError::Io(err) => EncodingError::IoError(err),
        err => EncodingError::Encoding(err.to_string()),
    }
}

pub(crate) fn encode_gif<W>(w: W, buffer: &PixelBuffer<Rgb>) -> Result<(), EncodingError>
where
    W: Write,
{
    let width = buffer.width();
    let height = buffer.height();

    if width > u16::MAX as u32 || height > u16::MAX as u32 {
        return Err(EncodingError::BadDimensions {
            format: "gif",
            width,
            height,
        });
    }

    let width = width as u16;
    let height = height as u16;

    let mut raw = to_rgba8_vec(buffer);

    let frame = Frame::from_rgba_speed(width, height, &mut raw, 10);

    let mut encoder = Encoder::new(w, frame.width, frame.height, &[]).map_err(encode_error)?;

    encoder.write_frame(&frame).map_err(encode_error)?;

    Ok(())
}

fn decode_error(err: GIFDecodingError) -> DecodingError {
    match err {
        GIFDecodingError::Io(err) => DecodingError::IoError(err),
        err => DecodingError::Decoding(err.to_string()),
    }
}

pub(crate) fn decode_gif<T>(reader: T) -> Result<DecodedImage, DecodingError>
where
    T: Read + Seek + BufRead,
{
    let mut decoder = DecodeOptions::new();

    decoder.set_color_output(gif::ColorOutput::RGBA);

    let mut decoder = decoder.read_info(reader).map_err(decode_error)?;

    if let Some(frame) = decoder.read_next_frame().map_err(decode_error)? {
        let data = frame
            .buffer
            .chunks(4)
            .map(|chunks| {
                Srgb::new_with_alpha(
                    from_u8(chunks[0]),
                    from_u8(chunks[1]),
                    from_u8(chunks[2]),
                    from_u8(chunks[3]),
                )
                .to_rgb()
            })
            .collect();

        let width = frame.width as u32;
        let height = frame.height as u32;

        if !is_valid_buffer_size(width, height) {
            return Err(DecodingError::InvalidBufferSize { width, height });
        }

        let buffer = PixelBuffer::new_from_raw(width, height, data);

        Ok(DecodedImage { buffer })
    } else {
        Err(DecodingError::Decoding("No frame found".to_owned()))
    }
}
