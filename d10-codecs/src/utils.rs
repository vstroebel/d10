use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::{Color, RGB, SRGB};
use d10_core::errors::D10Result;
use image::{DynamicImage, GenericImageView};

/// Convert color channel value between 0.0 and 1.0 into an u8
pub(crate) fn as_u8(value: f32) -> u8 {
    (value * 255.0).clamp(0.0, 255.0) as u8
}

/// Convert color channel value between 0.0 and 1.0 into an u16
pub(crate) fn as_u16(value: f32) -> u16 {
    (value * 65535.0).clamp(0.0, 65535.0) as u16
}

/// Convert color channel value between 0.0 and 1.0 into an big endian tuple of u8
pub(crate) fn as_u16_be(value: f32) -> (u8, u8) {
    let value = as_u16(value);

    let v1 = (value >> 8) as u8;
    let v2 = (value & 0x00ff) as u8;

    (v1, v2)
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

pub(crate) fn to_rgb8_vec(buffer: &PixelBuffer<RGB>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 3);

    for color in buffer.data().iter() {
        let color = color.to_srgb();

        out.push(as_u8(color.red()));
        out.push(as_u8(color.green()));
        out.push(as_u8(color.blue()));
    }

    out
}

pub(crate) fn to_rgba8_vec(buffer: &PixelBuffer<RGB>) -> Vec<u8> {
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

pub(crate) fn to_l16_be_vec(buffer: &PixelBuffer<RGB>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 2);

    for color in buffer.data().iter() {
        let color = color.to_gray().to_srgb();

        let (v1, v2) = as_u16_be(color.red());
        out.push(v1);
        out.push(v2);
    }

    out
}

pub(crate) fn to_la16_be_vec(buffer: &PixelBuffer<RGB>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 4);


    for color in buffer.data().iter() {
        let color = color.to_gray().to_srgb();

        let (v1, v2) = as_u16_be(color.red());
        out.push(v1);
        out.push(v2);

        let (v1, v2) = as_u16_be(color.alpha());
        out.push(v1);
        out.push(v2);
    }

    out
}

pub(crate) fn to_rgb16_be_vec(buffer: &PixelBuffer<RGB>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 6);

    for color in buffer.data().iter() {
        let color = color.to_srgb();

        let (v1, v2) = as_u16_be(color.red());
        out.push(v1);
        out.push(v2);

        let (v1, v2) = as_u16_be(color.green());
        out.push(v1);
        out.push(v2);

        let (v1, v2) = as_u16_be(color.blue());
        out.push(v1);
        out.push(v2);
    }

    out
}

pub(crate) fn to_rgba16_be_vec(buffer: &PixelBuffer<RGB>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 8);

    for color in buffer.data().iter() {
        let color = color.to_srgb();

        let (v1, v2) = as_u16_be(color.red());
        out.push(v1);
        out.push(v2);

        let (v1, v2) = as_u16_be(color.green());
        out.push(v1);
        out.push(v2);

        let (v1, v2) = as_u16_be(color.blue());
        out.push(v1);
        out.push(v2);

        let (v1, v2) = as_u16_be(color.alpha());
        out.push(v1);
        out.push(v2);
    }

    out
}

pub fn read_into_buffer(img: DynamicImage) -> D10Result<PixelBuffer<RGB>> {
    let width = img.width();
    let height = img.height();

    use image::DynamicImage::*;

    let data = match img {
        ImageRgb8(img) => img.pixels().map(|pixel| SRGB {
            data: [f32::from(pixel[0]) / 255.0,
                f32::from(pixel[1]) / 255.0,
                f32::from(pixel[2]) / 255.0,
                1.0]
        }.to_rgb()).collect(),
        ImageRgba8(img) => img.pixels().map(|pixel| SRGB {
            data: [f32::from(pixel[0]) / 255.0,
                f32::from(pixel[1]) / 255.0,
                f32::from(pixel[2]) / 255.0,
                f32::from(pixel[3]) / 255.0]
        }.to_rgb()).collect(),
        ImageBgr8(img) => img.pixels().map(|pixel| SRGB {
            data: [f32::from(pixel[2]) / 255.0,
                f32::from(pixel[1]) / 255.0,
                f32::from(pixel[0]) / 255.0,
                1.0]
        }.to_rgb()).collect(),
        ImageBgra8(img) => img.pixels().map(|pixel| SRGB {
            data: [f32::from(pixel[2]) / 255.0,
                f32::from(pixel[1]) / 255.0,
                f32::from(pixel[0]) / 255.0,
                f32::from(pixel[2]) / 255.0]
        }.to_rgb()).collect(),
        ImageRgb16(img) => img.pixels().map(|pixel| SRGB {
            data: [f32::from(pixel[0]) / 65535.0,
                f32::from(pixel[1]) / 65535.0,
                f32::from(pixel[2]) / 65535.0,
                0.0]
        }.to_rgb()).collect(),
        ImageRgba16(img) => img.pixels().map(|pixel| SRGB {
            data: [f32::from(pixel[0]) / 65535.0,
                f32::from(pixel[1]) / 65535.0,
                f32::from(pixel[2]) / 65535.0,
                f32::from(pixel[3]) / 65535.0]
        }.to_rgb()).collect(),
        ImageLuma8(img) => img.pixels().map(|pixel| SRGB {
            data: [f32::from(pixel[0]) / 255.0,
                f32::from(pixel[0]) / 255.0,
                f32::from(pixel[0]) / 255.0,
                1.0]
        }.to_rgb()).collect(),
        ImageLumaA8(img) => img.pixels().map(|pixel| SRGB {
            data: [f32::from(pixel[0]) / 255.0,
                f32::from(pixel[0]) / 255.0,
                f32::from(pixel[0]) / 255.0,
                f32::from(pixel[1]) / 255.0, ]
        }.to_rgb()).collect(),
        ImageLuma16(img) => img.pixels().map(|pixel| SRGB {
            data: [f32::from(pixel[0]) / 65535.0,
                f32::from(pixel[0]) / 65535.0,
                f32::from(pixel[0]) / 65535.0,
                1.0]
        }.to_rgb()).collect(),
        ImageLumaA16(img) => img.pixels().map(|pixel| SRGB {
            data: [f32::from(pixel[0]) / 65535.0,
                f32::from(pixel[0]) / 65535.0,
                f32::from(pixel[0]) / 65535.0,
                f32::from(pixel[1]) / 65535.0]
        }.to_rgb()).collect(),
    };

    PixelBuffer::new_from_raw(width, height, data)
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

    #[test]
    fn test_as_u16_be() {
        assert_eq!(as_u16_be(0.0), (0, 0));
        assert_eq!(as_u16_be(-0.5), (0, 0));

        assert_eq!(as_u16_be(1.0), (255, 255));
        assert_eq!(as_u16_be(1.5), (255, 255));

        assert_eq!(as_u16_be(0.5), (127, 255));
    }
}