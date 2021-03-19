use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;
use d10_core::errors::D10Result;
use d10_codecs::{encode, EncodingFormat, decode_buffer};

/// Returns a new buffer with a simulated jpeg quality
///
/// If `preserve_alpha` is not set all alpha values will be set to 1.0
pub fn jpeg_quality(buffer: &PixelBuffer<RGB>, quality: u8, preserve_alpha: bool) -> D10Result<PixelBuffer<RGB>> {
    let mut temp = vec![];

    encode(&mut temp, buffer, EncodingFormat::JPEG { quality })?;

    let mut out = decode_buffer(&temp)?.buffer;

    if preserve_alpha {
        for (c_in, c_out) in buffer.data().iter().zip(out.data_mut().iter_mut()) {
            c_out.data[3] = c_in.data[3]
        }
    }

    Ok(out)
}