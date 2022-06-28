use std::ffi::c_void;
use std::io::{BufRead, Read, Seek, Write};
use std::mem;
use std::str::FromStr;

use libwebp_sys::WebPPreset::{
    WEBP_PRESET_DEFAULT, WEBP_PRESET_DRAWING, WEBP_PRESET_ICON, WEBP_PRESET_PHOTO,
    WEBP_PRESET_PICTURE, WEBP_PRESET_TEXT,
};
use libwebp_sys::{
    WebPConfig, WebPConfigLosslessPreset, WebPDecodeRGBA, WebPEncode, WebPFree, WebPGetInfo,
    WebPPicture, WebPPictureFree,
};

use d10_core::color::{Color, Rgb, Srgb};
use d10_core::errors::ParseEnumError;
use d10_core::pixelbuffer::PixelBuffer;

use crate::utils::{from_u8, to_argb8_vec32};
use crate::{DecodedImage, DecodingError, EncodingError};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WebPPreset {
    Default,
    Picture,
    Photo,
    Drawing,
    Icon,
    Text,
    Lossless,
}

impl FromStr for WebPPreset {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use WebPPreset::*;
        match value {
            "default" => Ok(Default),
            "picture" => Ok(Picture),
            "photo" => Ok(Photo),
            "drawing" => Ok(Drawing),
            "icon" => Ok(Icon),
            "text" => Ok(Text),
            "lossless" => Ok(Lossless),
            _ => Err(ParseEnumError::new(value, "WebPPreset")),
        }
    }
}

pub(crate) fn decode_webp<T>(mut reader: T) -> Result<DecodedImage, DecodingError>
where
    T: Read + Seek + BufRead,
{
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
            return Err(DecodingError::Decoding(
                "Error decoding webp file".to_string(),
            ));
        }

        let image_data = std::slice::from_raw_parts(out_buf, width as usize * height as usize * 4);

        let buffer = PixelBuffer::new_from_func(width as u32, height as u32, |x, y| {
            let offset = (x as usize + y as usize * width as usize) * 4;
            Srgb::new_with_alpha(
                from_u8(image_data[offset]),
                from_u8(image_data[offset + 1]),
                from_u8(image_data[offset + 2]),
                from_u8(image_data[offset + 3]),
            )
            .to_rgb()
        });

        WebPFree(out_buf as *mut c_void);

        Ok(DecodedImage { buffer })
    }
}

#[repr(C)]
struct Writer<'a> {
    w: &'a mut dyn Write,
    err: Option<std::io::Error>,
}

unsafe extern "C" fn writer_function(
    data: *const u8,
    data_size: usize,
    picture: *const WebPPicture,
) -> ::std::os::raw::c_int {
    let write: *mut Writer = mem::transmute((*picture).custom_ptr);

    match (*write)
        .w
        .write_all(std::slice::from_raw_parts(data, data_size))
    {
        Ok(_) => 1,
        Err(err) => {
            (*write).err = Some(err);
            0
        }
    }
}

pub(crate) fn encode_webp<W>(
    mut w: W,
    buffer: &PixelBuffer<Rgb>,
    quality: u8,
    preset: WebPPreset,
) -> Result<(), EncodingError>
where
    W: Write,
{
    unsafe {
        let quality = quality.clamp(0, 100) as f32;
        let width = buffer.width() as i32;
        let height = buffer.height() as i32;

        let config = match preset {
            WebPPreset::Default => WebPConfig::new_with_preset(WEBP_PRESET_DEFAULT, quality),
            WebPPreset::Picture => WebPConfig::new_with_preset(WEBP_PRESET_PICTURE, quality),
            WebPPreset::Photo => WebPConfig::new_with_preset(WEBP_PRESET_PHOTO, quality),
            WebPPreset::Drawing => WebPConfig::new_with_preset(WEBP_PRESET_DRAWING, quality),
            WebPPreset::Icon => WebPConfig::new_with_preset(WEBP_PRESET_ICON, quality),
            WebPPreset::Text => WebPConfig::new_with_preset(WEBP_PRESET_TEXT, quality),
            WebPPreset::Lossless => {
                let mut config = WebPConfig::new();
                if let Ok(config) = &mut config {
                    WebPConfigLosslessPreset(config, 100);
                }
                config
            }
        }
        .map_err(|_| EncodingError::Encoding("Unable to init webp encoder config".to_owned()))?;

        let mut picture = WebPPicture::new().map_err(|_| {
            EncodingError::Encoding("Unable to init webp picture config".to_owned())
        })?;

        let mut write = Writer {
            w: &mut w,
            err: None,
        };

        let raw_data = to_argb8_vec32(buffer);

        picture.use_argb = 1;
        picture.width = width;
        picture.height = height;
        picture.argb = raw_data.as_ptr() as *mut u32;
        picture.argb_stride = width;
        picture.writer = Some(writer_function);
        picture.custom_ptr = &mut write as *mut _ as *mut c_void;

        let res = WebPEncode(&config, &mut picture);
        WebPPictureFree(&mut picture);

        match write.err {
            None => {
                if res == 0 {
                    Err(EncodingError::Encoding(
                        "Error encoding webp file".to_owned(),
                    ))
                } else {
                    Ok(())
                }
            }
            Some(err) => Err(err.into()),
        }
    }
}
