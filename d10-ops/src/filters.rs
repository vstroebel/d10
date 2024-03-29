use d10_core::color::Rgb;
use d10_core::errors::ParseEnumError;
use d10_core::pixelbuffer::PixelBuffer;

use std::f32::consts::PI;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum FilterMode {
    Nearest,
    Bilinear,
    Bicubic,
    Lanczos3,
    Auto,
}

impl FromStr for FilterMode {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use FilterMode::*;
        match value {
            "nearest" => Ok(Nearest),
            "bilinear" => Ok(Bilinear),
            "bicubic" => Ok(Bicubic),
            "lanczos3" | "Lanczos" => Ok(Lanczos3),
            "default" | "auto" => Ok(Auto),
            _ => Err(ParseEnumError::new(value, "FilterMode")),
        }
    }
}

fn linear_interpolate(v1: f32, v2: f32, t: f32) -> f32 {
    v1 + (v2 - v1) * t
}

/// Calculate the base pixel position and relative offset used as a factor in calculating interpolated values
fn get_base_and_offset(pos: f32) -> (i32, f32) {
    let pos_b = pos.floor();
    let offset = pos - pos_b;
    (pos_b as i32, offset)
}

/// Get the pixel at the given position applying a bilinear filter
pub(crate) fn get_pixel_bilinear(buffer: &PixelBuffer<Rgb>, x: f32, y: f32) -> Rgb {
    let (x, tx) = get_base_and_offset(x);
    let (y, ty) = get_base_and_offset(y);

    let c1 = buffer.get_pixel_clamped(x, y);
    let c2 = buffer.get_pixel_clamped(x + 1, y);
    let c3 = buffer.get_pixel_clamped(x, y + 1);
    let c4 = buffer.get_pixel_clamped(x + 1, y + 1);

    let calc = |i| {
        linear_interpolate(
            linear_interpolate(c1.data[i], c2.data[i], tx),
            linear_interpolate(c3.data[i], c4.data[i], tx),
            ty,
        )
    };

    Rgb::new_with_alpha(calc(0), calc(1), calc(2), calc(3))
}

fn cubic_hermite_interpolate(v1: f32, v2: f32, v3: f32, v4: f32, t: f32) -> f32 {
    let o1 = -v1 / 2.0 + (3.0 * v2) / 2.0 - (3.0 * v3) / 2.0 + v4 / 2.0;
    let o2 = v1 - (5.0 * v2) / 2.0 + 2.0 * v3 - v4 / 2.0;
    let o3 = -v1 / 2.0 + v3 / 2.0;
    let o4 = v2;

    o1 * t * t * t + o2 * t * t + o3 * t + o4
}

/// Get the pixel at the given position applying a bicubic filter
pub fn get_pixel_bicubic(buffer: &PixelBuffer<Rgb>, x: f32, y: f32) -> Rgb {
    let (x, tx) = get_base_and_offset(x);
    let (y, ty) = get_base_and_offset(y);

    let c00 = buffer.get_pixel_clamped(x - 1, y - 1);
    let c10 = buffer.get_pixel_clamped(x, y - 1);
    let c20 = buffer.get_pixel_clamped(x + 1, y - 1);
    let c30 = buffer.get_pixel_clamped(x + 2, y - 1);

    let c01 = buffer.get_pixel_clamped(x - 1, y);
    let c11 = buffer.get_pixel_clamped(x, y);
    let c21 = buffer.get_pixel_clamped(x + 1, y);
    let c31 = buffer.get_pixel_clamped(x + 2, y);

    let c02 = buffer.get_pixel_clamped(x - 1, y + 1);
    let c12 = buffer.get_pixel_clamped(x, y + 1);
    let c22 = buffer.get_pixel_clamped(x + 1, y + 1);
    let c32 = buffer.get_pixel_clamped(x + 2, y + 1);

    let c03 = buffer.get_pixel_clamped(x - 1, y + 2);
    let c13 = buffer.get_pixel_clamped(x, y + 2);
    let c23 = buffer.get_pixel_clamped(x + 1, y + 2);
    let c33 = buffer.get_pixel_clamped(x + 2, y + 2);

    let calc = |i| {
        cubic_hermite_interpolate(
            cubic_hermite_interpolate(c00.data[i], c10.data[i], c20.data[i], c30.data[i], tx),
            cubic_hermite_interpolate(c01.data[i], c11.data[i], c21.data[i], c31.data[i], tx),
            cubic_hermite_interpolate(c02.data[i], c12.data[i], c22.data[i], c32.data[i], tx),
            cubic_hermite_interpolate(c03.data[i], c13.data[i], c23.data[i], c33.data[i], tx),
            ty,
        )
    };

    Rgb::new_with_alpha(calc(0), calc(1), calc(2), calc(3))
}

/// sinc used for lanczos
fn sinc(v: f32) -> f32 {
    if v == 0.0 {
        1.0
    } else {
        let v = v * PI;
        v.sin() / v
    }
}

#[allow(clippy::collapsible_else_if)]
fn lanczos<const N: usize>(v: f32) -> f32 {
    let v = v.abs();

    if N == 7 {
        if v < 3.0 {
            sinc(v) * sinc(v / 3.0)
        } else {
            0.0
        }
    } else {
        let size = N as f32;
        let v = v / size * 3.0;
        if v < 3.0 {
            (sinc(v) * sinc(v / 3.0)) * (3.0 / size)
        } else {
            0.0
        }
    }
}

/// Get the pixel at the given position applying a lanczos filter with a window of 7
// Silence clippy because this would result in a mixture of range and non range loops...
#[allow(clippy::needless_range_loop)]
pub fn get_pixel_lanczos3(buffer: &PixelBuffer<Rgb>, x: f32, y: f32) -> Rgb {
    get_pixel_lanczos::<7>(buffer, x, y)
}

/// Get the pixel at the given position applying a lanczos filter with a window of N
// Silence clippy because this would result in a mixture of range and non range loops...
#[allow(clippy::needless_range_loop)]
pub fn get_pixel_lanczos<const N: usize>(buffer: &PixelBuffer<Rgb>, x: f32, y: f32) -> Rgb {
    let (x, tx) = get_base_and_offset(x);
    let (y, ty) = get_base_and_offset(y);

    let kernel = buffer.get_kernel::<N>(x, y);

    let size = ((N as f32) - 1.0) / 2.0;

    let row_scale: [f32; N] = std::array::from_fn(|i| {
        let pos = i as f32 - size;
        lanczos::<N>(pos - tx)
    });

    let mut rows = [[0.0; 4]; N];

    for y in 0..N {
        for x in 0..N {
            let scale = row_scale[x];
            for i in 0..=3 {
                let v = kernel[y][x].data[i];
                rows[y][i] += v * scale
            }
        }
    }

    let mut data = [0.0; 4];

    for y in 0..N {
        let scale = lanczos::<N>(y as f32 - size - ty);
        for i in 0..=3 {
            let v = rows[y][i];
            data[i] += v * scale;
        }
    }

    Rgb { data }
}

fn lanczos_dyn(v: f32, size: usize) -> f32 {
    let size = size as f32;

    let v = v.abs();

    let v = v / size * 3.0;

    if v < 3.0 {
        (sinc(v) * sinc(v / 3.0)) * (3.0 / size)
    } else {
        0.0
    }
}

/// Get the pixel at the given position applying a lanczos filter with a window of size
// Silence clippy because this would result in a mixture of range and non range loops...
#[allow(clippy::needless_range_loop)]
pub fn get_pixel_lanczos_dyn(buffer: &PixelBuffer<Rgb>, x: f32, y: f32, size: usize) -> Rgb {
    let (x, tx) = get_base_and_offset(x);
    let (y, ty) = get_base_and_offset(y);

    let rowsize = size * 2 + 1;

    let kernel = buffer.get_kernel_dyn(x, y, rowsize);

    let mut row_scale = Vec::with_capacity(rowsize);

    let size = size as isize;

    for i in 0..rowsize {
        row_scale.push(lanczos_dyn((i as isize - size) as f32 - tx, size as usize))
    }

    let mut rows = vec![[0.0; 4]; rowsize];

    for y in 0..rowsize {
        for x in 0..rowsize {
            let scale = row_scale[x];
            for i in 0..=3 {
                let v = kernel[y][x].data[i];
                rows[y][i] += v * scale
            }
        }
    }

    let mut data = [0.0; 4];

    for y in 0..rowsize {
        let scale = lanczos_dyn((y as isize - size) as f32 - ty, size as usize);
        for i in 0..=3 {
            let v = rows[y][i];
            data[i] += v * scale;
        }
    }

    Rgb { data }
}
