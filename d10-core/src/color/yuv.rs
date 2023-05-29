use std::array::from_fn;
use super::{apply_matrix, format_color, Color, Rgb, Srgb, EPSILON};
use std::fmt::Display;

pub(crate) const RGB_TO_YUV: [[f32; 3]; 3] = [
    [0.299, 0.587, 0.114],
    [-0.147_141_19, -0.288_869_17, 0.436_010_36],
    [0.614_975_4, -0.514_965_1, -0.100_010_26],
];

pub(crate) const YUV_TO_RGB: [[f32; 3]; 3] = [
    [1.0, 0.0, 1.139_883],
    [1.0, -0.394_642_32, -0.580_621_84],
    [1.0, 2.032_061_8, 0.0],
];

#[derive(Debug, Copy, Clone)]
pub struct Yuv {
    pub data: [f32; 4],
}

impl Yuv {
    pub fn new(y: f32, u: f32, v: f32) -> Yuv {
        Yuv {
            data: [y, u, v, 1.0],
        }
    }

    pub fn new_with_alpha(y: f32, u: f32, v: f32, alpha: f32) -> Yuv {
        Yuv {
            data: [y, u, v, alpha],
        }
    }

    pub fn new_from_fn<F: Fn(usize) -> f32>(func: F) -> Yuv {
        Yuv {
            data: from_fn(func)
        }
    }

    pub fn y(&self) -> f32 {
        self.data[0]
    }

    pub fn set_y(&mut self, y: f32) {
        self.data[0] = y;
    }

    pub fn with_y(&self, y: f32) -> Yuv {
        Yuv {
            data: [y, self.data[1], self.data[2], self.data[3]],
        }
    }

    pub fn u(&self) -> f32 {
        self.data[1]
    }

    pub fn set_u(&mut self, u: f32) {
        self.data[1] = u;
    }

    pub fn with_u(&self, u: f32) -> Yuv {
        Yuv {
            data: [self.data[0], u, self.data[2], self.data[3]],
        }
    }

    pub fn with_v(&self, v: f32) -> Yuv {
        Yuv {
            data: [self.data[0], self.data[1], v, self.data[3]],
        }
    }

    pub fn set_v(&mut self, v: f32) {
        self.data[2] = v;
    }

    pub fn v(&self) -> f32 {
        self.data[2]
    }
}

impl Default for Yuv {
    fn default() -> Yuv {
        Yuv {
            data: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

impl Color for Yuv {
    fn to_yuv(&self) -> Yuv {
        *self
    }

    fn to_rgb(&self) -> Rgb {
        Srgb {
            data: apply_matrix(&self.data, &YUV_TO_RGB),
        }
        .to_rgb()
    }

    fn data(&self) -> &[f32] {
        &self.data
    }

    fn data_mut(&mut self) -> &mut [f32] {
        &mut self.data
    }

    fn alpha(&self) -> f32 {
        self.data[3]
    }

    fn set_alpha(&mut self, alpha: f32) {
        self.data[3] = alpha;
    }

    fn with_alpha(&self, alpha: f32) -> Yuv {
        Yuv {
            data: [self.data[0], self.data[1], self.data[2], alpha],
        }
    }

    fn try_map_color_channels<E, F: FnMut(f32) -> Result<f32, E>>(
        &self,
        mut func: F,
    ) -> Result<Self, E> {
        Ok(Self::new_with_alpha(
            func(self.data[0])?,
            func(self.data[1])?,
            func(self.data[2])?,
            self.data[3],
        ))
    }

    fn type_name(&self) -> &'static str {
        "yuv"
    }
}

impl PartialEq for Yuv {
    fn eq(&self, other: &Yuv) -> bool {
        for (v1, v2) in self.data.iter().zip(other.data.iter()) {
            if (v1 - v2).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

impl Display for Yuv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_color(self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::{Color, Rgb, Yuv};

    const RGB_YUV: [((f32, f32, f32), (f32, f32, f32)); 15] = [
        ((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)),
        ((1.0, 1.0, 1.0), (1.0, 0.0, 0.0)),
        ((0.5, 0.5, 0.5), (0.7353569, 0.0, 0.0)),
        ((1.0, 0.0, 0.0), (0.299, -0.14714119, 0.6149754)),
        ((0.0, 1.0, 0.0), (0.587, -0.28886917, -0.5149651)),
        ((0.0, 0.0, 1.0), (0.114, 0.43601036, -0.10001026)),
        ((1.0, 0.5, 0.5), (0.8144852, -0.038939863, 0.16274892)),
        ((0.5, 1.0, 0.5), (0.89070237, -0.07644719, -0.1362819)),
        ((0.5, 0.5, 1.0), (0.7655262, 0.11538708, -0.026467033)),
        ((0.5, 0.0, 0.0), (0.21987171, -0.108201295, 0.4522264)),
        ((0.0, 0.5, 0.0), (0.4316545, -0.21242195, -0.37868318)),
        ((0.0, 0.0, 0.5), (0.08383069, 0.32062325, -0.073543236)),
        ((0.5, 0.0, 0.5), (0.3037024, 0.21242195, 0.37868315)),
        ((0.5, 0.5, 0.0), (0.65152629, -0.32062326, 0.07354324)),
        ((0.0, 0.5, 0.5), (0.51548525, 0.1082013, -0.45222644)),
    ];

    #[test]
    fn test_rgb_to_yuv() {
        for (from, to) in &RGB_YUV {
            assert_eq!(
                Rgb::new(from.0, from.1, from.2).to_yuv(),
                Yuv::new(to.0, to.1, to.2),
                "Error in conversion from {:?} to {:?}",
                from,
                to
            );
        }
    }

    #[test]
    fn test_yuv_to_rgb() {
        for (to, from) in &RGB_YUV {
            assert_eq!(
                Yuv::new(from.0, from.1, from.2).to_rgb(),
                Rgb::new(to.0, to.1, to.2),
                "Error in conversion from {:?} to {:?}",
                from,
                to
            );
        }
    }

    #[test]
    fn type_name() {
        assert_eq!(Yuv::default().type_name(), "yuv");
    }

    #[test]
    fn to_string() {
        assert_eq!(
            Yuv::new_with_alpha(0.0, 0.0, 0.0, 1.0).to_string(),
            "yuv(0.0, 0.0, 0.0)"
        );
        assert_eq!(
            Yuv::new_with_alpha(1.0, 1.0, 1.0, 1.0).to_string(),
            "yuv(1.0, 1.0, 1.0)"
        );
        assert_eq!(
            Yuv::new_with_alpha(0.0, 0.0, 0.0, 0.0).to_string(),
            "yuva(0.0, 0.0, 0.0, 0.0)"
        );
        assert_eq!(
            Yuv::new_with_alpha(0.3, 0.6, 0.9, 0.5).to_string(),
            "yuva(0.3, 0.6, 0.9, 0.5)"
        );
        assert_eq!(
            Yuv::new_with_alpha(0.33, 0.666, 0.999, 0.5555).to_string(),
            "yuva(0.33, 0.666, 0.999, 0.5555)"
        );
    }

    #[test]
    fn test_setters() {
        let mut color = Yuv::new_with_alpha(0.1, 0.3, 0.5, 0.7);
        assert_eq!(color.y(), 0.1);
        assert_eq!(color.with_y(0.2).y(), 0.2);
        color.set_y(0.2);
        assert_eq!(color.y(), 0.2);

        assert_eq!(color.u(), 0.3);
        assert_eq!(color.with_u(0.4).u(), 0.4);
        color.set_u(0.4);
        assert_eq!(color.u(), 0.4);

        assert_eq!(color.v(), 0.5);
        assert_eq!(color.with_v(0.6).v(), 0.6);
        color.set_v(0.6);
        assert_eq!(color.v(), 0.6);

        assert_eq!(color.alpha(), 0.7);
        assert_eq!(color.with_alpha(0.8).alpha(), 0.8);
        color.set_alpha(0.8);
        assert_eq!(color.alpha(), 0.8);
    }
}
