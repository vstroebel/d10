use d10_core::color::{Color, Lab, Rgb};
use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::illuminant::D65;
use d10_core::color::observer::O2;

pub fn apply_palette<C: Color>(buffer: &PixelBuffer<Rgb>, palette: &PixelBuffer<C>) -> PixelBuffer<Rgb> {
    let palette = palette.to_lab();
    buffer.map_colors(|c| get_color_from_palette(&palette, c))
}

pub fn apply_palette_in_place<C: Color>(buffer: &mut PixelBuffer<Rgb>, palette: &PixelBuffer<C>) {
    let palette = palette.to_lab();
    buffer.mod_colors(|c| get_color_from_palette(&palette, c));
}

fn get_color_from_palette(palette: &PixelBuffer<Lab<D65, O2>>, c: &Rgb) -> Rgb {
    let mut result = None;
    let mut min_diff = f32::MAX;

    let test_c = c.to_lab();

    for pal_c in palette.data() {
        let diff = get_delta_e(&test_c, &pal_c);

        if diff < min_diff {
            result = Some(pal_c);
            min_diff = diff;
        }
    }

    result.cloned().unwrap_or_default().to_rgb()
}

fn get_delta_e(c1: &Lab<D65, O2>, c2: &Lab<D65, O2>) -> f32 {
    let dl = c1.l() - c2.l();
    let da = c1.a() - c2.a();
    let db = c1.b() - c2.b();

    (dl * dl + da * da + db * db).sqrt()
}