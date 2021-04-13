use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::{Color, Rgb, Intensity};

pub fn despeckle(img: &PixelBuffer<Rgb>, threshold: f32, amount: u8) -> PixelBuffer<Rgb> {
    img.map_colors_enumerated(|x, y, c| {
        if c.to_gray_with_intensity(Intensity::Average).red() < threshold {
            let k = img.get_kernel::<3>(x as i32, y as i32);

            let mut count = 0;

            for y in 0..3 {
                for x in 0..3 {
                    if k[y][x].to_gray_with_intensity(Intensity::Average).red() < threshold {
                        count += 1;
                    }
                }
            }

            if count <= amount {
                let mut data = [0f32; 3];

                for y in 0..3 {
                    for x in 0..3 {
                        let kc = k[y][x];
                        if kc.to_gray_with_intensity(Intensity::Average).red() >= threshold {
                            data[0] += kc.data[0];
                            data[1] += kc.data[1];
                            data[2] += kc.data[2];
                        }
                    }
                }

                let sum = (9 - count) as f32;

                return Rgb::new_with_alpha(data[0] / sum, data[1] / sum, data[2] / sum, c.alpha());
            }
        }

        *c
    })
}