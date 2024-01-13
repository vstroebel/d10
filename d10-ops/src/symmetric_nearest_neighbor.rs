use d10_core::color::Rgb;
use d10_core::pixelbuffer::PixelBuffer;

const PAIRS: [(i32, i32, i32, i32); 4] =
    [(-1, -1, 1, 1), (0, -1, 0, 1), (1, -1, -1, 1), (-1, 0, 1, 0)];

fn get_delta_e(c1: &Rgb, c2: &Rgb) -> f32 {
    let dr = c1.red() - c2.red();
    let dg = c1.green() - c2.green();
    let db = c1.blue() - c2.blue();

    (dr * dr + dg * dg + db * db).sqrt()
}

pub fn symmetric_nearest_neighbor(
    buffer: &PixelBuffer<Rgb>,
    radius: usize,
    with_center: bool,
) -> PixelBuffer<Rgb> {
    PixelBuffer::new_from_func(buffer.width(), buffer.height(), |x, y| {
        let center = buffer.get_pixel(x, y);

        let x = x as i32;
        let y = y as i32;

        let colors: Vec<Rgb> = (1..=radius as i32)
            .flat_map(|current_radius| {
                PAIRS.map(|(x1, y1, x2, y2)| {
                    let c1 = buffer
                        .get_pixel_clamped(x + (x1 * current_radius), y + (y1 * current_radius));
                    let c2 = buffer
                        .get_pixel_clamped(x + (x2 * current_radius), y + (y2 * current_radius));

                    *if get_delta_e(c1, center) < get_delta_e(c2, center) {
                        c1
                    } else {
                        c2
                    }
                })
            })
            .collect();

        let mut accumulate = [0.0; 4];

        let mut len = colors.len() as f32;

        if with_center {
            len += 1.0;
            for (i, value) in accumulate.iter_mut().enumerate() {
                *value += center.data[i];
            }
        }

        for c in &colors {
            for (i, value) in accumulate.iter_mut().enumerate() {
                *value += c.data[i];
            }
        }

        Rgb::new_with_alpha(
            accumulate[0] / len,
            accumulate[1] / len,
            accumulate[2] / len,
            accumulate[3] / len,
        )
    })
}
