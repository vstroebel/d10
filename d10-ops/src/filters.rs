use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;

pub enum FilterMode {
    Nearest,
    Bilinear,
    Bicubic,
}

fn linear_interpolate(v1: f32, v2: f32, t: f32) -> f32 {
    v1 + (v2 - v1) * t
}

/// Calculate the base pixel position and the relative offset used as a factor in calculating the linear or bicubic interpolated value
fn get_base_and_offset(pos: f32) -> (i32, f32) {
    let pos_b = pos.floor();
    let offset = pos - pos_b;
    (pos_b as i32, offset)
}

/// Get the pixel at the given position applying a bilinear filter
pub(crate) fn get_pixel_bilinear(buffer: &PixelBuffer<RGB>, x: f32, y: f32) -> RGB {
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

    RGB::new_with_alpha(calc(0), calc(1), calc(2), calc(3))
}

fn cubic_hermite_interpolate(v1: f32, v2: f32, v3: f32, v4: f32, t: f32) -> f32 {
    let o1 = -v1 / 2.0 + (3.0 * v2) / 2.0 - (3.0 * v3) / 2.0 + v4 / 2.0;
    let o2 = v1 - (5.0 * v2) / 2.0 + 2.0 * v3 - v4 / 2.0;
    let o3 = -v1 / 2.0 + v3 / 2.0;
    let o4 = v2;

    o1 * t * t * t
        + o2 * t * t
        + o3 * t
        + o4
}

/// Get the pixel at the given position applying a bicubic filter
pub fn get_pixel_bicubic(buffer: &PixelBuffer<RGB>, x: f32, y: f32) -> RGB {
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

    RGB::new_with_alpha(calc(0), calc(1), calc(2), calc(3))
}