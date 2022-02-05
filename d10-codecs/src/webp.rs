use std::ffi::c_void;
use std::io::{BufRead, Read, Seek, Write};

use libwebp_sys::{WebPDecodeRGBA, WebPEncodeRGBA, WebPFree, WebPGetInfo};

use d10_core::color::{Color, Rgb, Srgb};
use d10_core::pixelbuffer::PixelBuffer;

use crate::{DecodedImage, DecodingError, EncodingError};
use crate::utils::{from_u8, to_rgba8_vec};

pub(crate) fn decode_webp<T>(mut reader: T) -> Result<DecodedImage, DecodingError> where T: Read + Seek + BufRead {
    let mut width = 0;
    let mut height = 0;

    let mut data = vec![];

    let mut buf = [0u8; 4096];

    loop {
        let res = reader.read(&mut buf)?;
        if res > 0 {
            data.extend_from_slice(&buf[0..res]);
        } else {
            break;
        }
    }

    unsafe {
        let len = data.len();

        if WebPGetInfo(data.as_ptr(), len, &mut width, &mut height) == 0 {
            return Err(DecodingError::Decoding("Bad webp file".to_string()));
        }
        let out_buf = WebPDecodeRGBA(data.as_ptr(), len, &mut width, &mut height);
        if out_buf.is_null() {
            return Err(DecodingError::Decoding("Error decoding webp file".to_string()));
        }

        let image_data = std::slice::from_raw_parts(out_buf, width as usize * height as usize * 4);

        let buffer = PixelBuffer::new_from_func(width as u32, height as u32, |x, y| {
            let offset = (x as usize + y as usize * width as usize) * 4;
            Srgb::new_with_alpha(
                from_u8(image_data[offset]),
                from_u8(image_data[offset + 1]),
                from_u8(image_data[offset + 2]),
                from_u8(image_data[offset + 3]),
            ).to_rgb()
        });

        WebPFree(out_buf as *mut c_void);

        Ok(DecodedImage {
            buffer
        })
    }
}

pub(crate) fn encode_webp<W>(mut w: W,
                             buffer: &PixelBuffer<Rgb>,
                             quality: u8) -> Result<(), EncodingError>
    where W: Write {
    unsafe {
        let quality = quality.clamp(0, 100) as f32;
        let width = buffer.width() as i32;
        let height = buffer.height() as i32;

        let mut out_buf = std::ptr::null_mut();
        let stride = width * 4;

        let raw_data = to_rgba8_vec(buffer);

        let len = WebPEncodeRGBA(raw_data.as_ptr(), width, height, stride, quality, &mut out_buf);

        if out_buf.is_null() {
            return Err(EncodingError::Encoding("Error encoding data".to_string()));
        }

        if len == 0 {
            WebPFree(out_buf as *mut c_void);
            return Err(EncodingError::Encoding("Zero length encoding data".to_string()));
        }

        let encoded = std::slice::from_raw_parts(out_buf, len as usize);

        let res = w.write_all(encoded);
        WebPFree(out_buf as *mut c_void);

        res.map_err(|err| err.into())
    }
}

