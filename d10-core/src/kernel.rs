pub struct Kernel<const N: usize> {
    pub data: [[f32; N]; N],
}

impl<const N: usize> Kernel<N> {
    pub const fn new(data: [[f32; N]; N]) -> Kernel<N> {
        Kernel {
            data,
        }
    }

    // Silence clippy because this doesn't make code more readable in a multidimensional array
    #[allow(clippy::needless_range_loop)]
    pub fn apply_kernel<T, F, const S: usize>(&self, data: &[[T; N]; N], func: F) -> [f32; S]
        where F: Fn(&T, usize) -> f32
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
}
