use crate::FilterMode;
use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::Rgb;
use std::f32::consts::PI;

use crate::filters::{get_pixel_bicubic, get_pixel_bilinear};

fn rotate_nearest(buffer: &PixelBuffer<Rgb>, radians: f32, bg_color: Rgb) -> PixelBuffer<Rgb> {
    let radians = radians / -180.0 * PI;

    let sinf = radians.sin();
    let cosf = radians.cos();

    let center_x = (buffer.width() + 1) as f32 / 2.0;
    let center_y = (buffer.height() + 1) as f32 / 2.0;

    let new_width = buffer.width();
    let new_height = buffer.height();

    let result = (0..new_width * new_height)
        .map(|i| (i % new_width, i / new_width))
        .map(|(x, y)| {
            let x = x as f32 + 1.0;
            let y = y as f32 + 1.0;

            let a = x - center_x;
            let b = y - center_y;
            let xx = (a * cosf - b * sinf + center_x) - 1.0;
            let yy = (a * sinf + b * cosf + center_y) - 1.0;

            let xx = xx.round() as i32;
            let yy = yy.round() as i32;

            *buffer.get_pixel_optional(xx, yy).unwrap_or(&bg_color)
        }).collect();

    PixelBuffer::new_from_raw(new_width, new_height, result)
}

fn rotate_bilinear(buffer: &PixelBuffer<Rgb>, radians: f32, bg_color: Rgb) -> PixelBuffer<Rgb> {
    let radians = radians / -180.0 * PI;

    let sinf = radians.sin();
    let cosf = radians.cos();

    let center_x = (buffer.width() + 1) as f32 / 2.0;
    let center_y = (buffer.height() + 1) as f32 / 2.0;

    let new_width = buffer.width();
    let new_height = buffer.height();

    let result = (0..new_width * new_height)
        .map(|i| (i % new_width, i / new_width))
        .map(|(x, y)| {
            let x = x as f32 + 1.0;
            let y = y as f32 + 1.0;

            let a = x - center_x;
            let b = y - center_y;
            let xx = a * cosf - b * sinf + center_x - 1.0;
            let yy = a * sinf + b * cosf + center_y - 1.0;

            if buffer.is_in_image(xx.round() as i32, yy.round() as i32) {
                get_pixel_bilinear(&buffer, xx, yy)
            } else {
                bg_color
            }
        }).collect();

    PixelBuffer::new_from_raw(new_width, new_height, result)
}

fn rotate_bicubic(buffer: &PixelBuffer<Rgb>, radians: f32, bg_color: Rgb) -> PixelBuffer<Rgb> {
    let radians = radians / -180.0 * PI;

    let sinf = radians.sin();
    let cosf = radians.cos();

    let center_x = (buffer.width() + 1) as f32 / 2.0;
    let center_y = (buffer.height() + 1) as f32 / 2.0;

    let new_width = buffer.width();
    let new_height = buffer.height();

    let result = (0..new_width * new_height)
        .map(|i| (i % new_width, i / new_width))
        .map(|(x, y)| {
            let x = x as f32 + 1.0;
            let y = y as f32 + 1.0;

            let a = x - center_x;
            let b = y - center_y;
            let xx = a * cosf - b * sinf + center_x - 1.0;
            let yy = a * sinf + b * cosf + center_y - 1.0;

            if buffer.is_in_image(xx.round() as i32, yy.round() as i32) {
                get_pixel_bicubic(&buffer, xx, yy)
            } else {
                bg_color
            }
        }).collect();

    PixelBuffer::new_from_raw(new_width, new_height, result)
}

pub fn rotate(buffer: &PixelBuffer<Rgb>, radians: f32, bg_color: Rgb, filter: FilterMode) -> PixelBuffer<Rgb> {
    match filter {
        FilterMode::Nearest => rotate_nearest(buffer, radians, bg_color),
        FilterMode::Bilinear => rotate_bilinear(buffer, radians, bg_color),
        FilterMode::Bicubic => rotate_bicubic(buffer, radians, bg_color),
    }
}
