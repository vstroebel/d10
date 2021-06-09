use d10_core::pixelbuffer::{PixelBuffer, is_valid_buffer_size};
use d10_core::color::{Color, Rgb, Srgb};
use image::{DynamicImage, GenericImageView};

use crate::DecodingError;

/// Convert color channel value between 0.0 and 1.0 into an u8
pub(crate) fn as_u8(value: f32) -> u8 {
    (value * 255.0).clamp(0.0, 255.0) as u8
}

/// Convert color channel value between 0.0 and 1.0 into an u16
pub(crate) fn as_u16(value: f32) -> u16 {
    (value * 65535.0).clamp(0.0, 65535.0) as u16
}

pub(crate) fn to_l8_vec(buffer: &PixelBuffer<Rgb>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize);

    for color in buffer.data().iter() {
        let color = color.to_gray().to_srgb();
        out.push(as_u8(color.red()));
    }

    out
}

pub(crate) fn to_la8_vec(buffer: &PixelBuffer<Rgb>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 2);

    for color in buffer.data().iter() {
        let color = color.to_gray().to_srgb();
        out.push(as_u8(color.red()));
        out.push(as_u8(color.alpha()));
    }

    out
}

pub(crate) fn to_rgb8_vec(buffer: &PixelBuffer<Rgb>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 3);

    for color in buffer.data().iter() {
        let color = color.to_srgb();

        out.push(as_u8(color.red()));
        out.push(as_u8(color.green()));
        out.push(as_u8(color.blue()));
    }

    out
}

pub(crate) fn to_rgba8_vec(buffer: &PixelBuffer<Rgb>) -> Vec<u8> {
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

pub(crate) fn to_l16_be_vec(buffer: &PixelBuffer<Rgb>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 2);

    for color in buffer.data().iter() {
        let color = color.to_gray().to_srgb();
        out.extend_from_slice(&color.red().to_be_bytes());
    }

    out
}

pub(crate) fn to_la16_be_vec(buffer: &PixelBuffer<Rgb>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 4);


    for color in buffer.data().iter() {
        let color = color.to_gray().to_srgb();

        out.extend_from_slice(&as_u16(color.red()).to_be_bytes());
        out.extend_from_slice(&as_u16(color.alpha()).to_be_bytes());
    }

    out
}

pub(crate) fn to_rgb16_be_vec(buffer: &PixelBuffer<Rgb>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 6);

    for color in buffer.data().iter() {
        let color = color.to_srgb();

        out.extend_from_slice(&as_u16(color.red()).to_be_bytes());
        out.extend_from_slice(&as_u16(color.green()).to_be_bytes());
        out.extend_from_slice(&as_u16(color.blue()).to_be_bytes());
    }

    out
}

pub(crate) fn to_rgba16_be_vec(buffer: &PixelBuffer<Rgb>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 8);

    for color in buffer.data().iter() {
        let color = color.to_srgb();

        out.extend_from_slice(&as_u16(color.red()).to_be_bytes());
        out.extend_from_slice(&as_u16(color.green()).to_be_bytes());
        out.extend_from_slice(&as_u16(color.blue()).to_be_bytes());
        out.extend_from_slice(&as_u16(color.alpha()).to_be_bytes());
    }

    out
}

pub fn from_u8(v: u8) -> f32 {
    f32::from(v) / 255.0
}

pub fn from_u16_be(v: [u8; 2]) -> f32 {
    f32::from(u16::from_be_bytes(v)) / 65535.0
}

pub fn read_into_buffer(img: DynamicImage) -> Result<PixelBuffer<Rgb>, DecodingError> {
    let width = img.width();
    let height = img.height();

    if !is_valid_buffer_size(width, height) {
        return Err(DecodingError::InvalidBufferSize { width, height });
    }

    use image::DynamicImage::*;

    let data = match img {
        ImageRgb8(img) => img.pixels().map(|pixel| Srgb {
            data: [f32::from(pixel[0]) / 255.0,
                f32::from(pixel[1]) / 255.0,
                f32::from(pixel[2]) / 255.0,
                1.0]
        }.to_rgb()).collect(),
        ImageRgba8(img) => img.pixels().map(|pixel| Srgb {
            data: [f32::from(pixel[0]) / 255.0,
                f32::from(pixel[1]) / 255.0,
                f32::from(pixel[2]) / 255.0,
                f32::from(pixel[3]) / 255.0]
        }.to_rgb()).collect(),
        ImageBgr8(img) => img.pixels().map(|pixel| Srgb {
            data: [f32::from(pixel[2]) / 255.0,
                f32::from(pixel[1]) / 255.0,
                f32::from(pixel[0]) / 255.0,
                1.0]
        }.to_rgb()).collect(),
        ImageBgra8(img) => img.pixels().map(|pixel| Srgb {
            data: [f32::from(pixel[2]) / 255.0,
                f32::from(pixel[1]) / 255.0,
                f32::from(pixel[0]) / 255.0,
                f32::from(pixel[2]) / 255.0]
        }.to_rgb()).collect(),
        ImageRgb16(img) => img.pixels().map(|pixel| Srgb {
            data: [f32::from(pixel[0]) / 65535.0,
                f32::from(pixel[1]) / 65535.0,
                f32::from(pixel[2]) / 65535.0,
                0.0]
        }.to_rgb()).collect(),
        ImageRgba16(img) => img.pixels().map(|pixel| Srgb {
            data: [f32::from(pixel[0]) / 65535.0,
                f32::from(pixel[1]) / 65535.0,
                f32::from(pixel[2]) / 65535.0,
                f32::from(pixel[3]) / 65535.0]
        }.to_rgb()).collect(),
        ImageLuma8(img) => img.pixels().map(|pixel| Srgb {
            data: [f32::from(pixel[0]) / 255.0,
                f32::from(pixel[0]) / 255.0,
                f32::from(pixel[0]) / 255.0,
                1.0]
        }.to_rgb()).collect(),
        ImageLumaA8(img) => img.pixels().map(|pixel| Srgb {
            data: [f32::from(pixel[0]) / 255.0,
                f32::from(pixel[0]) / 255.0,
                f32::from(pixel[0]) / 255.0,
                f32::from(pixel[1]) / 255.0, ]
        }.to_rgb()).collect(),
        ImageLuma16(img) => img.pixels().map(|pixel| Srgb {
            data: [f32::from(pixel[0]) / 65535.0,
                f32::from(pixel[0]) / 65535.0,
                f32::from(pixel[0]) / 65535.0,
                1.0]
        }.to_rgb()).collect(),
        ImageLumaA16(img) => img.pixels().map(|pixel| Srgb {
            data: [f32::from(pixel[0]) / 65535.0,
                f32::from(pixel[0]) / 65535.0,
                f32::from(pixel[0]) / 65535.0,
                f32::from(pixel[1]) / 65535.0]
        }.to_rgb()).collect(),
    };

    Ok(PixelBuffer::new_from_raw(width, height, data))
}

/// Convert CMYK to RGB without color profile
#[allow(clippy::many_single_char_names)]
pub fn cmyk_to_rgb(c: u8, m: u8, y: u8, k: u8) -> Rgb {
    let c = 255.0 - c as f32;
    let m = 255.0 - m as f32;
    let y = 255.0 - y as f32;
    let k = 255.0 - k as f32;

    let r = c * k / 65536.0;
    let g = m * k / 65536.0;
    let b = y * k / 65536.0;

    Srgb::new(r, g, b).to_rgb()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_u8() {
        assert_eq!(as_u8(0.0), 0);
        assert_eq!(as_u8(-0.5), 0);

        assert_eq!(as_u8(1.0), 255);
        assert_eq!(as_u8(1.5), 255);

        assert_eq!(as_u8(0.5), 127);
    }

    #[test]
    fn test_as_u16() {
        assert_eq!(as_u16(0.0), 0);
        assert_eq!(as_u16(-0.5), 0);

        assert_eq!(as_u16(1.0), 65535);
        assert_eq!(as_u16(1.5), 65535);

        assert_eq!(as_u16(0.5), 32767);
    }
}