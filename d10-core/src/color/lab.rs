use super::{format_color, Color, Rgb, Xyz, EPSILON};

use crate::color::illuminant::D65;
use crate::color::observer::O2;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
pub struct Lab<I: Illuminant = D65, O: Observer = O2> {
    pub data: [f32; 4],
    _phantom: PhantomData<I>,
    _phantom2: PhantomData<O>,
}

pub type DefaultLab = Lab<D65, O2>;

pub trait Illuminant: Debug + Copy + Clone + Send + Sync {
    #[doc(hidden)]
    fn get_refs() -> &'static [[f32; 3]; 2];

    #[doc(hidden)]
    fn type_name_lab() -> &'static [&'static str];

    #[doc(hidden)]
    fn type_name_lch() -> &'static [&'static str];
}

pub mod illuminant {
    use super::Illuminant;

    #[macro_export]
    macro_rules! illuminant {
        ($name:ident, $refs:expr) => {
            #[derive(Debug, Copy, Clone)]
            pub struct $name {}

            impl Illuminant for $name {
                fn get_refs() -> &'static [[f32; 3]; 2] {
                    &$refs
                }

                fn type_name_lab() -> &'static [&'static str] {
                    &[
                        concat!("lab<", stringify!($name), ",O2>"),
                        concat!("lab<", stringify!($name), ",O10>"),
                    ]
                }

                fn type_name_lch() -> &'static [&'static str] {
                    &[
                        concat!("lch<", stringify!($name), ",O2>"),
                        concat!("lch<", stringify!($name), ",O10>"),
                    ]
                }
            }
        };
    }

    illuminant!(
        D50,
        [
            [0.964_212, 1.0, 0.825_188_3],
            [0.967_206_3, 1.0, 0.814_280_15]
        ]
    );
    illuminant!(
        D65,
        [[0.950_47, 1.0, 1.088_83], [0.948_096_7, 1.0, 1.073_051_3]]
    );
    illuminant!(E, [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]);
}

pub trait Observer: Debug + Copy + Clone + Send + Sync {
    #[doc(hidden)]
    fn get_refs(refs: &'static [[f32; 3]; 2]) -> &'static [f32; 3];

    #[doc(hidden)]
    fn type_name_lab(refs: &'static [&'static str]) -> &'static str;

    #[doc(hidden)]
    fn type_name_lch(refs: &'static [&'static str]) -> &'static str;
}

pub mod observer {
    use super::Observer;

    #[derive(Debug, Copy, Clone)]
    pub struct O2 {}

    impl Observer for O2 {
        fn get_refs(refs: &'static [[f32; 3]; 2]) -> &'static [f32; 3] {
            &refs[0]
        }

        fn type_name_lab(refs: &'static [&'static str]) -> &'static str {
            refs[0]
        }

        fn type_name_lch(refs: &'static [&'static str]) -> &'static str {
            refs[0]
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct O10 {}

    impl Observer for O10 {
        fn get_refs(refs: &'static [[f32; 3]; 2]) -> &'static [f32; 3] {
            &refs[1]
        }

        fn type_name_lab(refs: &'static [&'static str]) -> &'static str {
            refs[1]
        }

        fn type_name_lch(refs: &'static [&'static str]) -> &'static str {
            refs[1]
        }
    }
}

impl<I: Illuminant, O: Observer> Lab<I, O> {
    pub fn new(l: f32, a: f32, b: f32) -> Lab<I, O> {
        Self::new_with_alpha(l, a, b, 1.0)
    }

    pub fn new_with_alpha(l: f32, a: f32, b: f32, alpha: f32) -> Lab<I, O> {
        Lab {
            data: [l, a, b, alpha],
            _phantom: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn l(&self) -> f32 {
        self.data[0]
    }

    pub fn set_l(&mut self, l: f32) {
        self.data[0] = l;
    }

    pub fn with_l(&self, l: f32) -> Self {
        Self::new_with_alpha(l, self.data[1], self.data[2], self.data[3])
    }

    pub fn a(&self) -> f32 {
        self.data[1]
    }

    pub fn set_a(&mut self, a: f32) {
        self.data[1] = a;
    }

    pub fn with_a(&self, a: f32) -> Self {
        Self::new_with_alpha(self.data[0], a, self.data[2], self.data[3])
    }

    pub fn b(&self) -> f32 {
        self.data[2]
    }

    pub fn set_b(&mut self, b: f32) {
        self.data[2] = b;
    }

    pub fn with_b(&self, b: f32) -> Self {
        Self::new_with_alpha(self.data[0], self.data[1], b, self.data[3])
    }
}

impl<I: Illuminant, O: Observer> Default for Lab<I, O> {
    fn default() -> Self {
        Self::new_with_alpha(0.0, 0.0, 0.0, 0.0)
    }
}

impl<I: Illuminant, O: Observer> Color for Lab<I, O> {
    fn to_rgb(&self) -> Rgb {
        self.to_xyz().to_rgb()
    }

    fn to_xyz(&self) -> Xyz {
        fn func(v: f32) -> f32 {
            if v > 0.206_893_03 {
                v.powf(3.0)
            } else {
                (v - 16.0 / 116.0) / 7.787
            }
        }

        let l = self.l() * 100.0;
        let a = self.a() * 128.0;
        let b = self.b() * 128.0;

        let ry = (l + 16.0) / 116.0;
        let rx = a / 500.0 + ry;
        let rz = ry - b / 200.0;

        let rx = func(rx);
        let ry = func(ry);
        let rz = func(rz);

        let refs = get_refs::<I, O>();

        Xyz::new_with_alpha(rx * refs[0], ry * refs[1], rz * refs[2], self.alpha())
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

    fn with_alpha(&self, alpha: f32) -> Self {
        Self::new_with_alpha(self.data[0], self.data[1], self.data[2], alpha)
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
        O::type_name_lab(I::type_name_lab())
    }
}

impl<I: Illuminant, O: Observer> PartialEq for Lab<I, O> {
    fn eq(&self, other: &Self) -> bool {
        for (v1, v2) in self.data.iter().zip(other.data.iter()) {
            if (v1 - v2).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

impl<I: Illuminant, O: Observer> Display for Lab<I, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_color(self, f)
    }
}

pub(crate) fn get_refs<I: Illuminant, O: Observer>() -> &'static [f32; 3] {
    O::get_refs(I::get_refs())
}

#[derive(Debug, Copy, Clone)]
pub struct Lch<I: Illuminant = D65, O: Observer = O2> {
    pub data: [f32; 4],
    _phantom: PhantomData<I>,
    _phantom2: PhantomData<O>,
}

impl<I: Illuminant, O: Observer> Lch<I, O> {
    pub fn new(l: f32, c: f32, h: f32) -> Lch<I, O> {
        Self::new_with_alpha(l, c, h, 1.0)
    }

    pub fn new_with_alpha(l: f32, c: f32, h: f32, alpha: f32) -> Lch<I, O> {
        Lch {
            data: [l, c, h, alpha],
            _phantom: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn l(&self) -> f32 {
        self.data[0]
    }

    pub fn set_l(&mut self, l: f32) {
        self.data[0] = l;
    }

    pub fn with_l(&self, l: f32) -> Self {
        Self::new_with_alpha(l, self.data[1], self.data[2], self.data[3])
    }

    pub fn c(&self) -> f32 {
        self.data[1]
    }

    pub fn set_c(&mut self, c: f32) {
        self.data[1] = c;
    }

    pub fn with_c(&self, c: f32) -> Self {
        Self::new_with_alpha(self.data[0], c, self.data[2], self.data[3])
    }

    pub fn h(&self) -> f32 {
        self.data[2]
    }

    pub fn set_h(&mut self, h: f32) {
        self.data[2] = h;
    }

    pub fn with_h(&self, h: f32) -> Self {
        Self::new_with_alpha(self.data[0], self.data[1], h, self.data[3])
    }
}

impl<I: Illuminant, O: Observer> Default for Lch<I, O> {
    fn default() -> Self {
        Self::new_with_alpha(0.0, 0.0, 0.0, 0.0)
    }
}

impl<I: Illuminant, O: Observer> Color for Lch<I, O> {
    fn to_rgb(&self) -> Rgb {
        self.to_lab::<I, O>().to_rgb()
    }

    fn to_xyz(&self) -> Xyz {
        let a = self.c() * self.h().cos();
        let b = self.c() * self.h().sin();
        Lab::<I, O>::new_with_alpha(self.l(), a, b, self.alpha()).to_xyz()
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

    fn with_alpha(&self, alpha: f32) -> Self {
        Self::new_with_alpha(self.data[0], self.data[1], self.data[2], alpha)
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
        O::type_name_lch(I::type_name_lch())
    }
}

impl<I: Illuminant, O: Observer> PartialEq for Lch<I, O> {
    fn eq(&self, other: &Self) -> bool {
        for (v1, v2) in self.data.iter().zip(other.data.iter()) {
            if (v1 - v2).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

impl<I: Illuminant, O: Observer> Display for Lch<I, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_color(self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::illuminant::{D50, D65, E};
    use crate::color::lab::DefaultLab;
    use crate::color::observer::{O10, O2};
    use crate::color::{Color, Lab, Srgb};

    const SRGB_LAB_65_2: [((f32, f32, f32), (f32, f32, f32)); 6] = [
        ((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)),
        ((1.0, 1.0, 1.0), (1.0, -0.000_019, 0.000_036)),
        ((0.5, 0.5, 0.5), (0.533_889_6, 0.0, 0.0)),
        (
            (1.0, 0.0, 0.0),
            (0.532_405_879_4, 0.625_721_16, 0.525_021_49),
        ),
        (
            (0.0, 1.0, 0.0),
            (0.877_350_994_9, -0.673_304_92, 0.649_841_43),
        ),
        (
            (0.0, 0.0, 1.0),
            (0.322_956_725_7, 0.618_637_43, -0.842_635_16),
        ),
    ];

    #[test]
    fn test_srgb_to_lab_65_2() {
        for (from, to) in &SRGB_LAB_65_2 {
            let srgb = Srgb::new(from.0, from.1, from.2);
            assert_eq!(
                srgb.to_lab(),
                Lab::<D65, O2>::new(to.0, to.1, to.2),
                "Error in conversion from {:?} to {:?} with xyz {}",
                from,
                to,
                srgb.to_xyz()
            );
        }
    }

    #[test]
    fn test_lab_65_2_to_rgb() {
        for (to, from) in &SRGB_LAB_65_2 {
            assert_eq!(
                Lab::<D65, O2>::new(from.0, from.1, from.2).to_srgb(),
                Srgb::new(to.0, to.1, to.2),
                "Error in conversion from {:?} to {:?}",
                from,
                to
            );
        }
    }

    const SRGB_LAB_65_10: [((f32, f32, f32), (f32, f32, f32)); 6] = [
        ((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)),
        ((1.0, 1.0, 1.0), (1.0, 0.003_237_53, -0.007_584_81)),
        ((0.5, 0.5, 0.5), (0.533_889_6, 0.001_936_63, -0.004_537_08)),
        ((1.0, 0.0, 0.0), (0.532_405_88, 0.628_186_78, 0.523_033_18)),
        (
            (0.0, 1.0, 0.0),
            (0.877_350_986_0, -0.670_953_88, 0.646_195_61),
        ),
        ((0.0, 0.0, 1.0), (0.322_956_73, 0.620_509_11, -0.849_918_33)),
    ];

    #[test]
    fn test_srgb_to_lab_65_10() {
        for (from, to) in &SRGB_LAB_65_10 {
            let srgb = Srgb::new(from.0, from.1, from.2);
            assert_eq!(
                srgb.to_lab(),
                Lab::<D65, O10>::new(to.0, to.1, to.2),
                "Error in conversion from {:?} to {:?} with xyz {}",
                from,
                to,
                srgb.to_xyz()
            );
        }
    }

    #[test]
    fn test_lab_65_10_to_rgb() {
        for (to, from) in &SRGB_LAB_65_10 {
            assert_eq!(
                Lab::<D65, O10>::new(from.0, from.1, from.2).to_srgb(),
                Srgb::new(to.0, to.1, to.2),
                "Error in conversion from {:?} to {:?}",
                from,
                to
            );
        }
    }

    #[test]
    fn to_string() {
        assert_eq!(
            DefaultLab::new_with_alpha(0.0, 0.0, 0.0, 1.0).to_string(),
            "lab<D65,O2>(0.0, 0.0, 0.0)"
        );
        assert_eq!(
            DefaultLab::new_with_alpha(1.0, 1.0, 1.0, 1.0).to_string(),
            "lab<D65,O2>(1.0, 1.0, 1.0)"
        );
        assert_eq!(
            DefaultLab::new_with_alpha(0.0, 0.0, 0.0, 0.0).to_string(),
            "lab<D65,O2>a(0.0, 0.0, 0.0, 0.0)"
        );
        assert_eq!(
            DefaultLab::new_with_alpha(0.3, 0.6, 0.9, 0.5).to_string(),
            "lab<D65,O2>a(0.3, 0.6, 0.9, 0.5)"
        );
        assert_eq!(
            DefaultLab::new_with_alpha(0.33, 0.666, 0.999, 0.5555).to_string(),
            "lab<D65,O2>a(0.33, 0.666, 0.999, 0.5555)"
        );

        assert_eq!(
            Lab::<D65, O10>::new_with_alpha(0.3, 0.6, 0.9, 0.5).to_string(),
            "lab<D65,O10>a(0.3, 0.6, 0.9, 0.5)"
        );
        assert_eq!(
            Lab::<D50, O2>::new_with_alpha(0.3, 0.6, 0.9, 0.5).to_string(),
            "lab<D50,O2>a(0.3, 0.6, 0.9, 0.5)"
        );
        assert_eq!(
            Lab::<D50, O10>::new_with_alpha(0.3, 0.6, 0.9, 0.5).to_string(),
            "lab<D50,O10>a(0.3, 0.6, 0.9, 0.5)"
        );
        assert_eq!(
            Lab::<E, O2>::new_with_alpha(0.3, 0.6, 0.9, 0.5).to_string(),
            "lab<E,O2>a(0.3, 0.6, 0.9, 0.5)"
        );
        assert_eq!(
            Lab::<E, O10>::new_with_alpha(0.3, 0.6, 0.9, 0.5).to_string(),
            "lab<E,O10>a(0.3, 0.6, 0.9, 0.5)"
        );
    }

    #[test]
    fn test_setters() {
        let mut color = DefaultLab::new_with_alpha(0.1, 0.3, 0.5, 0.7);
        assert_eq!(color.l(), 0.1);
        assert_eq!(color.with_l(0.2).l(), 0.2);
        color.set_l(0.2);
        assert_eq!(color.l(), 0.2);

        assert_eq!(color.a(), 0.3);
        assert_eq!(color.with_a(0.4).a(), 0.4);
        color.set_a(0.4);
        assert_eq!(color.a(), 0.4);

        assert_eq!(color.b(), 0.5);
        assert_eq!(color.with_b(0.6).b(), 0.6);
        color.set_b(0.6);
        assert_eq!(color.b(), 0.6);

        assert_eq!(color.alpha(), 0.7);
        assert_eq!(color.with_alpha(0.8).alpha(), 0.8);
        color.set_alpha(0.8);
        assert_eq!(color.alpha(), 0.8);
    }
}
