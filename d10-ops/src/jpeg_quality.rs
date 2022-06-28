use d10_codecs::{decode_buffer, encode, EncodingFormat};
use d10_core::color::Rgb;
use d10_core::pixelbuffer::PixelBuffer;

/// Returns a new buffer with a simulated jpeg quality
///
/// If `preserve_alpha` is not set, all alpha values will be set to 1.0
pub fn jpeg_quality(
    buffer: &PixelBuffer<Rgb>,
    quality: u8,
    preserve_alpha: bool,
) -> PixelBuffer<Rgb> {
    let mut temp = vec![];

    encode(
        &mut temp,
        buffer,
        EncodingFormat::jpeg_with_quality(quality),
    )
    .expect("Encoded image");

    let mut out = decode_buffer(&temp).expect("Decoded image").buffer;

    if preserve_alpha {
        for (c_in, c_out) in buffer.data().iter().zip(out.data_mut().iter_mut()) {
            c_out.data[3] = c_in.data[3]
        }
    }

    out
}
