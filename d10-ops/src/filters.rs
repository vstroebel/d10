use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::RGB;

pub enum FilterMode {
    Nearest,
    Bilinear,
}

fn linear_interpolate(v1: f32, v2: f32, t: f32) -> f32 {
    v1 + (v2 - v1) * t
}

/// Calculate the base pixel position and the relative offset used as a factor in calculating the linear interpolated value
fn get_linear_base_and_offset(pos: f32) -> (i32, f32) {
    let pos_b = pos.floor();
    let offset = pos - pos_b;
    (pos_b as i32, offset)
}

/// Get the pixel at the given position applying a bilinear filter
pub(crate) fn get_pixel_bilinear(buffer: &PixelBuffer<RGB>, x: f32, y: f32) -> RGB {
    let (x, tx) = get_linear_base_and_offset(x);
    let (y, ty) = get_linear_base_and_offset(y);

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