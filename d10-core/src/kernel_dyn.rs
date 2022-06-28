use std::f32::consts::PI;

pub struct KernelDyn {
    data: Vec<f32>,
    pub width: u32,
    pub height: u32,
}

impl KernelDyn {
    pub fn new(data: Vec<f32>, width: u32, height: u32) -> KernelDyn {
        assert_eq!(data.len(), (width * height) as usize);

        KernelDyn {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, x: u32, y: u32) -> f32 {
        self.data[(x + y * self.width) as usize]
    }

    pub fn get_offset_x(&self) -> i32 {
        self.width as i32 / 2
    }

    pub fn get_offset_y(&self) -> i32 {
        self.height as i32 / 2
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (u32, u32, f32)> + '_ {
        let width = self.width;

        self.data
            .iter()
            .enumerate()
            .map(move |(i, v)| (i as u32 % width, i as u32 / width, *v))
    }

    pub fn new_gaussian(size: u32, sigma: f32) -> KernelDyn {
        let mut data = vec![0.0; (size * size) as usize];

        {
            let size = size as isize;
            let offset = size / 2;

            let s = 2.0 * sigma * sigma;

            let mut sum = 0.0;

            for x in -offset..size - offset {
                for y in -offset..size - offset {
                    let r = (x as f32 * x as f32 + y as f32 * y as f32).sqrt();

                    let v = ((-(r * r) / s).exp()) / (PI * s);

                    let index = ((x + offset) + ((y + offset) * size)) as usize;

                    data[index] = v;
                    sum += v;
                }
            }

            for v in &mut data {
                *v /= sum;
            }
        }

        Self::new(data, size, size)
    }

    pub fn new_sobel_x() -> KernelDyn {
        KernelDyn {
            data: vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0],
            width: 3,
            height: 3,
        }
    }

    pub fn new_sobel_y() -> KernelDyn {
        KernelDyn {
            data: vec![1.0, 2.0, 1.0, 0.0, 0.0, 0.0, -1.0, -2.0, -1.0],
            width: 3,
            height: 3,
        }
    }
}
