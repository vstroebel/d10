use crate::FilterMode;
use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::Rgb;
use std::f32::consts::PI;

use crate::filters::{get_pixel_bicubic, get_pixel_bilinear, get_pixel_lanczos3};

fn rotate_with_fn<F>(buffer: &PixelBuffer<Rgb>, radians: f32, bg_color: Rgb, func: F) -> PixelBuffer<Rgb>
    where
        F: Fn(&PixelBuffer<Rgb>, f32, f32) -> Option<Rgb>
{
    let radians = radians / -180.0 * PI;

    let sinf = radians.sin();
    let cosf = radians.cos();

    let center_x = (buffer.width() + 1) as f32 / 2.0;
    let center_y = (buffer.height() + 1) as f32 / 2.0;

    let new_width = buffer.width();
    let new_height = buffer.height();

    PixelBuffer::new_from_func(new_width, new_height, |x, y| {
        let x = x as f32 + 1.0;
        let y = y as f32 + 1.0;

        let a = x - center_x;
        let b = y - center_y;
        let xx = a * cosf - b * sinf + center_x - 1.0;
        let yy = a * sinf + b * cosf + center_y - 1.0;

        func(buffer, xx, yy).unwrap_or(bg_color)
    })
}

fn rotate_pixel_nearest(buffer: &PixelBuffer<Rgb>, x: f32, y: f32) -> Option<Rgb> {
    let x = x.round() as i32;
    let y = y.round() as i32;

    buffer.get_pixel_optional(x, y).cloned()
}

fn rotate_pixel_bilinear(buffer: &PixelBuffer<Rgb>, x: f32, y: f32) -> Option<Rgb> {
    if buffer.is_in_image(x.round() as i32, y.round() as i32) {
        Some(get_pixel_bilinear(buffer, x, y))
    } else {
        None
    }
}

fn rotate_pixel_bicubic(buffer: &PixelBuffer<Rgb>, x: f32, y: f32) -> Option<Rgb> {
    if buffer.is_in_image(x.round() as i32, y.round() as i32) {
        Some(get_pixel_bicubic(buffer, x, y))
    } else {
        None
    }
}

fn rotate_pixel_lanczos3(buffer: &PixelBuffer<Rgb>, x: f32, y: f32) -> Option<Rgb> {
    if buffer.is_in_image(x.round() as i32, y.round() as i32) {
        Some(get_pixel_lanczos3(buffer, x, y))
    } else {
        None
    }
}

pub fn rotate(buffer: &PixelBuffer<Rgb>, radians: f32, bg_color: Rgb, filter: FilterMode) -> PixelBuffer<Rgb> {
    if (radians - 360.0).abs() < f32::EPSILON {
        return buffer.clone();
    }

    match filter {
        FilterMode::Nearest => rotate_with_fn(buffer, radians, bg_color, rotate_pixel_nearest),
        FilterMode::Bilinear => rotate_with_fn(buffer, radians, bg_color, rotate_pixel_bilinear),
        FilterMode::Bicubic => rotate_with_fn(buffer, radians, bg_color, rotate_pixel_bicubic),
        FilterMode::Lanczos3 => rotate_with_fn(buffer, radians, bg_color, rotate_pixel_lanczos3),
    }
}
