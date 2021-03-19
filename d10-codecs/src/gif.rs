use d10_core::errors::{D10Result, D10Error};
use d10_core::color::{SRGB, Color, RGB};
use d10_core::pixelbuffer::PixelBuffer;

use std::io::{Read, Seek, BufRead, Write};

use crate::DecodedImage;
use crate::utils::{from_u8, to_rgba8_vec};

use gif::{DecodeOptions, Frame, Encoder};
use std::convert::TryInto;

pub(crate) fn encode_gif<W>(w: &mut W, buffer: &PixelBuffer<RGB>) -> D10Result<()>
    where W: Write {
    let width = buffer.width().try_into().map_err(|_| D10Error::SaveError(format!("Unsupported width for gif: {}", buffer.width())))?;
    let height = buffer.height().try_into().map_err(|_| D10Error::SaveError(format!("Unsupported height for gif: {}", buffer.height())))?;

    let mut raw = to_rgba8_vec(buffer);

    let frame = Frame::from_rgba_speed(width, height, &mut raw, 10);

    let mut encoder = Encoder::new(w, frame.width, frame.height, &[])
        .map_err(|err| D10Error::SaveError(format!("Error writing image: {}", err)))?;

    encoder.write_frame(&frame).map_err(|err| D10Error::SaveError(format!("Error writing frame: {}", err)))?;

    Ok(())
}


pub(crate) fn decode_gif<T>(reader: T) -> D10Result<DecodedImage>
    where T: Read + Seek + BufRead {
    let mut decoder = DecodeOptions::new();

    decoder.set_color_output(gif::ColorOutput::RGBA);

    let mut decoder = decoder.read_info(reader).unwrap();

    if let Some(frame) = decoder.read_next_frame().unwrap() {
        let data = frame.buffer.chunks(4).map(|chunks| {
            SRGB::new_with_alpha(
                from_u8(chunks[0]),
                from_u8(chunks[1]),
                from_u8(chunks[2]),
                from_u8(chunks[3]),
            ).to_rgb()
        }).collect();

        let buffer = PixelBuffer::new_from_raw(frame.width as u32, frame.height as u32, data).unwrap();

        Ok(DecodedImage {
            buffer
        })
    } else {
        Err(D10Error::OpenError("No frame found".to_owned()))
    }
}