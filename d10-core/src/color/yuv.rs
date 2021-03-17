use super::{Color, RGB, SRGB, EPSILON};
use std::fmt::Display;
use crate::color::format_color;

#[derive(Debug, Copy, Clone)]
pub struct YUV {
    pub data: [f32; 4]
}

impl YUV {
    pub fn new(y: f32, u: f32, v: f32) -> YUV {
        YUV { data: [y, u, v, 1.0] }
    }

    pub fn new_with_alpha(y: f32, u: f32, v: f32, alpha: f32) -> YUV {
        YUV { data: [y, u, v, alpha] }
    }

    pub fn y(&self) -> f32 {
        self.data[0]
    }

    pub fn u(&self) -> f32 {
        self.data[1]
    }

    pub fn v(&self) -> f32 {
        self.data[2]
    }

    pub fn with_y(&self, y: f32) -> YUV {
        YUV { data: [y, self.data[1], self.data[2], self.data[3]] }
    }

    pub fn with_u(&self, u: f32) -> YUV {
        YUV { data: [self.data[0], u, self.data[2], self.data[3]] }
    }

    pub fn with_v(&self, v: f32) -> YUV {
        YUV { data: [self.data[0], self.data[1], v, self.data[3]] }
    }
}

impl Default for YUV {
    fn default() -> YUV {
        YUV {
            data: [0.0, 0.0, 0.0, 0.0]
        }
    }
}

impl Color for YUV {
    fn to_yuv(&self) -> YUV {
        *self
    }

    fn to_rgb(&self) -> RGB {
        let red = self.y() + 1.139_883 * self.v();
        let green = self.y() + -0.394_642_32 * self.u() + -0.580_621_84 * self.v();
        let blue = self.y() + 2.032_061_8 * self.u();

        SRGB {
            data: [red, green, blue, self.alpha()]
        }.to_rgb()
    }

    fn data(&self) -> &[f32] {
        &self.data
    }

    fn alpha(&self) -> f32 {
        self.data[3]
    }

    fn map_color_channels<F: FnMut(f32) -> f32>(&self, mut func: F) -> Self {
        Self::new_with_alpha(
            func(self.data[0]),
            func(self.data[1]),
            func(self.data[2]),
            self.data[3])
    }

    fn type_name(&self) -> &'static str {
        "yuv"
    }
}

impl PartialEq for YUV {
    fn eq(&self, other: &YUV) -> bool {
        for (v1, v2) in self.data.iter().zip(other.data.iter()) {
            if (v1 - v2).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

impl Display for YUV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_color(self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::{RGB, YUV, Color};


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
        ((0.0, 0.5, 0.5), (0.51548525, 0.1082013, -0.45222644))
    ];

    #[test]
    fn test_rgb_to_yuv() {
        for (from, to) in &RGB_YUV {
            assert_eq!(RGB::new(from.0, from.1, from.2).to_yuv(), YUV::new(to.0, to.1, to.2),
                       "Error in conversion from {:?} to {:?}", from, to);
        }
    }

    #[test]
    fn test_yuv_to_rgb() {
        for (to, from) in &RGB_YUV {
            assert_eq!(YUV::new(from.0, from.1, from.2).to_rgb(), RGB::new(to.0, to.1, to.2),
                       "Error in conversion from {:?} to {:?}", from, to);
        }
    }

    #[test]
    fn type_name() {
        assert_eq!(YUV::default().type_name(), "yuv");
    }

    #[test]
    fn to_string() {
        assert_eq!(YUV::new_with_alpha(0.0, 0.0, 0.0, 1.0).to_string(), "yuv(0.0, 0.0, 0.0)");
        assert_eq!(YUV::new_with_alpha(1.0, 1.0, 1.0, 1.0).to_string(), "yuv(1.0, 1.0, 1.0)");
        assert_eq!(YUV::new_with_alpha(0.0, 0.0, 0.0, 0.0).to_string(), "yuva(0.0, 0.0, 0.0, 0.0)");
        assert_eq!(YUV::new_with_alpha(0.3, 0.6, 0.9, 0.5).to_string(), "yuva(0.3, 0.6, 0.9, 0.5)");
        assert_eq!(YUV::new_with_alpha(0.33, 0.666, 0.999, 0.5555).to_string(), "yuva(0.33, 0.666, 0.999, 0.5555)");
    }
}