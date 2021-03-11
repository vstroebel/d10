use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::{Color, RGB, SRGB};
use d10_core::errors::{D10Result, D10Error};

use image::{DynamicImage, GenericImageView, ImageError, ImageBuffer, Rgba};
use image::io::Reader;

use std::path::Path;
use std::io::{Cursor, Read, Seek, BufRead};

fn read_into_buffer(img: DynamicImage) -> D10Result<PixelBuffer<RGB>> {
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

pub struct DecodedImage {
    pub buffer: PixelBuffer<RGB>
}

pub fn decode_file<P>(path: P) -> D10Result<DecodedImage> where P: AsRef<Path> {
    decode(Reader::open(path)?)
}

pub fn decode_buffer(buffer: &[u8]) -> D10Result<DecodedImage> {
    decode(Reader::new(Cursor::new(buffer)).with_guessed_format()?)
}

fn decode<T>(reader: Reader<T>) -> D10Result<DecodedImage> where T: Read + Seek + BufRead {
    let img = reader.decode().map_err(|err| match err {
        ImageError::IoError(err) => D10Error::IOError(err),
        ImageError::Limits(l) => D10Error::Limits(format!("{:?}", l)),
        err => D10Error::OpenError(format!("Open error: {:?}", err))
    })?;

    read_into_buffer(img).map(|buffer| DecodedImage {
        buffer
    })
}

pub fn save_to_file<P>(path: P, buffer: &PixelBuffer<RGB>) -> D10Result<()> where P: AsRef<Path> {
    let mut out: Vec<u8> = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 4);

    for color in buffer.data().iter() {
        let color = color.to_srgb();

        out.push((color.red().min(1.0).max(0.0) * 255.0) as u8);
        out.push((color.green().min(1.0).max(0.0) * 255.0) as u8);
        out.push((color.blue().min(1.0).max(0.0) * 255.0) as u8);
        out.push((color.alpha().min(1.0).max(0.0) * 255.0) as u8);
    }

    let out: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(buffer.width(), buffer.height(), out)
        .ok_or_else(|| D10Error::OpenError("Unable to create buffer".to_owned()))?;

    out.save(path).map_err(|err| match err {
        ImageError::IoError(err) => D10Error::IOError(err),
        ImageError::Limits(l) => D10Error::Limits(format!("{:?}", l)),
        err => D10Error::SaveError(format!("Save error: {:?}", err))
    })?;

    Ok(())
}