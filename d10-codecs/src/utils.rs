use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::{Color, RGB};

/// Convert color channel value between 0.0 and 1.0 into an u8
pub(crate) fn as_u8(value: f32) -> u8 {
    (value * 255.0).clamp(0.0, 255.0) as u8
}

pub(crate) fn to_l8_vec(buffer: &PixelBuffer<RGB>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize);

    for color in buffer.data().iter() {
        let color = color.to_gray().to_srgb();
        out.push(as_u8(color.red()));
    }

    out
}

pub(crate) fn to_la8_vec(buffer: &PixelBuffer<RGB>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 2);

    for color in buffer.data().iter() {
        let color = color.to_gray().to_srgb();
        out.push(as_u8(color.red()));
        out.push(as_u8(color.alpha()));
    }

    out
}

pub(crate) fn to_rgb24_vec(buffer: &PixelBuffer<RGB>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 3);

    for color in buffer.data().iter() {
        let color = color.to_srgb();

        out.push(as_u8(color.red()));
        out.push(as_u8(color.green()));
        out.push(as_u8(color.blue()));
    }

    out
}

pub(crate) fn to_rgba32_vec(buffer: &PixelBuffer<RGB>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 4);

    for color in buffer.data().iter() {
        let color = color.to_srgb();

        out.push(as_u8(color.red()));
        out.push(as_u8(color.green()));
        out.push(as_u8(color.blue()));
        out.push(as_u8(color.alpha()));
    }

    out
}

