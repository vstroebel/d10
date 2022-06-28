use std::f32::consts::PI;

pub struct Kernel<const N: usize> {
    pub data: [[f32; N]; N],
}

impl<const N: usize> Kernel<N> {
    pub const fn new(data: [[f32; N]; N]) -> Kernel<N> {
        Kernel { data }
    }

    // Silence clippy because this doesn't make code more readable in a multidimensional array
    #[allow(clippy::needless_range_loop)]
    pub fn apply_kernel<T, F, const S: usize>(&self, data: &[[T; N]; N], func: F) -> [f32; S]
    where
        F: Fn(&T, usize) -> f32,
    {
        let mut result = [0f32; S];

        for y in 0..N {
            for x in 0..N {
                for (i, v) in result.iter_mut().enumerate() {
                    *v += self.data[y][x] * func(&data[y][x], i);
                }
            }
        }

        result
    }

    pub fn new_gaussian(sigma: f32) -> Kernel<N> {
        let mut data = [[0.0; N]; N];

        {
            let size = N as isize;
            let offset = size / 2;

            let s = 2.0 * sigma * sigma;

            let mut sum = 0.0;

            for x in -offset..size - offset {
                for y in -offset..size - offset {
                    let r = (x as f32 * x as f32 + y as f32 * y as f32).sqrt();

                    let v = ((-(r * r) / s).exp()) / (PI * s);

                    data[(y + offset) as usize][(x + offset) as usize] = v;
                    sum += v;
                }
            }

            for row in &mut data {
                for v in row {
                    *v /= sum;
                }
            }
        }

        Self::new(data)
    }
}
