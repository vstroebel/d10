use crate::color::{Color, Rgb, apply_matrix, EPSILON, format_color};
use std::fmt::Display;

pub(crate) const RGB_TO_XYZ: [[f32; 3]; 3] = [
    [0.412_453, 0.357_580, 0.180_423, ],
    [0.212_671, 0.715_160, 0.072_169],
    [0.019_334, 0.119_193, 0.950_227]
];

pub(crate) const XYZ_TO_RGB: [[f32; 3]; 3] = [
    [3.240_479, -1.537_15, -0.498_535],
    [-0.969_256, 1.875_991, 0.041_556],
    [0.055_648, -0.204_043, 1.057_311]
];

/// CIE XYZ.Rec 709 with D65 white point
#[derive(Debug, Copy, Clone)]
pub struct Xyz {
    pub data: [f32; 4]
}

impl Xyz {
    pub fn new(x: f32, y: f32, z: f32) -> Xyz {
        Xyz { data: [x, y, z, 1.0] }
    }

    pub fn new_with_alpha(x: f32, y: f32, z: f32, alpha: f32) -> Xyz {
        Xyz { data: [x, y, z, alpha] }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn set_x(&mut self, x: f32) {
        self.data[0] = x;
    }

    pub fn with_x(&self, x: f32) -> Xyz {
        Xyz { data: [x, self.data[1], self.data[2], self.data[3]] }
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn set_y(&mut self, y: f32) {
        self.data[1] = y;
    }

    pub fn with_y(&self, y: f32) -> Xyz {
        Xyz { data: [self.data[0], y, self.data[2], self.data[3]] }
    }

    pub fn with_z(&self, z: f32) -> Xyz {
        Xyz { data: [self.data[0], self.data[1], z, self.data[3]] }
    }

    pub fn set_z(&mut self, z: f32) {
        self.data[2] = z;
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }
}

impl Default for Xyz {
    fn default() -> Xyz {
        Xyz {
            data: [0.0, 0.0, 0.0, 0.0]
        }
    }
}

impl Color for Xyz {
    fn to_xyz(&self) -> Xyz {
        *self
    }

    fn to_rgb(&self) -> Rgb {
        Rgb {
            data: apply_matrix(&self.data, &XYZ_TO_RGB)
        }.to_rgb()
    }

    fn data(&self) -> &[f32] {
        &self.data
    }

    fn alpha(&self) -> f32 {
        self.data[3]
    }

    fn set_alpha(&mut self, alpha: f32) {
        self.data[3] = alpha;
    }

    fn with_alpha(&self, alpha: f32) -> Xyz {
        Xyz { data: [self.data[0], self.data[1], self.data[2], alpha] }
    }

    fn try_map_color_channels<E, F: FnMut(f32) -> Result<f32, E>>(&self, mut func: F) -> Result<Self, E> {
        Ok(Self::new_with_alpha(
            func(self.data[0])?,
            func(self.data[1])?,
            func(self.data[2])?,
            self.data[3]))
    }

    fn type_name(&self) -> &'static str {
        "xyz"
    }
}

impl PartialEq for Xyz {
    fn eq(&self, other: &Xyz) -> bool {
        for (v1, v2) in self.data.iter().zip(other.data.iter()) {
            if (v1 - v2).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

impl Display for Xyz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_color(self, f)
    }
}


#[cfg(test)]
mod tests {
    use crate::color::{Xyz, Color, Srgb};

    const SRGB_XYZ: [((f32, f32, f32), (f32, f32, f32)); 9] = [
        ((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)),
        ((1.0, 1.0, 1.0), (0.950_456, 1.0, 1.088_754)),
        ((0.5, 0.5, 0.5), (0.203_436_69, 0.214_041_14, 0.233_038_15)),
        ((1.0, 0.0, 0.0), (0.412_453, 0.212_671, 0.019_334)),
        ((0.0, 1.0, 0.0), (0.357_58, 0.715_16, 0.119_193)),
        ((0.0, 0.0, 1.0), (0.180_423, 0.072_169, 0.950_227)),
        ((1.0, 0.5, 0.5), (0.527_607_78, 0.381_191_8, 0.248_233_88)),
        ((0.5, 1.0, 0.5), (0.484_479_86, 0.776_127_48, 0.326_718_94)),
        ((0.5, 0.5, 1.0), (0.345_241_74, 0.270_763_01, 0.979_877_48)),
    ];

    #[test]
    fn test_srgb_to_xyz() {
        for (from, to) in &SRGB_XYZ {
            assert_eq!(Srgb::new(from.0, from.1, from.2).to_xyz(), Xyz::new(to.0, to.1, to.2),
                       "Error in conversion from {:?} to {:?}", from, to);
        }
    }

    #[test]
    fn test_yuv_to_rgb() {
        for (to, from) in &SRGB_XYZ {
            assert_eq!(Xyz::new(from.0, from.1, from.2).to_srgb(), Srgb::new(to.0, to.1, to.2),
                       "Error in conversion from {:?} to {:?}", from, to);
        }
    }

    #[test]
    fn type_name() {
        assert_eq!(Xyz::default().type_name(), "xyz");
    }

    #[test]
    fn to_string() {
        assert_eq!(Xyz::new_with_alpha(0.0, 0.0, 0.0, 1.0).to_string(), "xyz(0.0, 0.0, 0.0)");
        assert_eq!(Xyz::new_with_alpha(1.0, 1.0, 1.0, 1.0).to_string(), "xyz(1.0, 1.0, 1.0)");
        assert_eq!(Xyz::new_with_alpha(0.0, 0.0, 0.0, 0.0).to_string(), "xyza(0.0, 0.0, 0.0, 0.0)");
        assert_eq!(Xyz::new_with_alpha(0.3, 0.6, 0.9, 0.5).to_string(), "xyza(0.3, 0.6, 0.9, 0.5)");
        assert_eq!(Xyz::new_with_alpha(0.33, 0.666, 0.999, 0.5555).to_string(), "xyza(0.33, 0.666, 0.999, 0.5555)");
    }

    #[test]
    fn test_setters() {
        let mut color = Xyz::new_with_alpha(0.1, 0.3, 0.5, 0.7);
        assert_eq!(color.x(), 0.1);
        assert_eq!(color.with_x(0.2).x(), 0.2);
        color.set_x(0.2);
        assert_eq!(color.x(), 0.2);

        assert_eq!(color.y(), 0.3);
        assert_eq!(color.with_y(0.4).y(), 0.4);
        color.set_y(0.4);
        assert_eq!(color.y(), 0.4);

        assert_eq!(color.z(), 0.5);
        assert_eq!(color.with_z(0.6).z(), 0.6);
        color.set_z(0.6);
        assert_eq!(color.z(), 0.6);

        assert_eq!(color.alpha(), 0.7);
        assert_eq!(color.with_alpha(0.8).alpha(), 0.8);
        color.set_alpha(0.8);
        assert_eq!(color.alpha(), 0.8);
    }
}