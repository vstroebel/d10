use crate::color::{Color, Rgb, Srgb, Hsl, Hsv, Yuv, Xyz, Lab, Lch, lab::{Illuminant, Observer}};

use std::iter::Cloned;
use std::marker::PhantomData;

pub struct ToRgbIter<I, C: Color> {
    iter: I,
    _phantom: PhantomData<C>,
}

impl<I, C: Color> Iterator for ToRgbIter<I, C>
    where I: Iterator<Item=C> {
    type Item = Rgb;

    fn next(&mut self) -> Option<Rgb> {
        self.iter.next().map(|v| v.to_rgb())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct ToSrgbIter<I, C: Color> {
    iter: I,
    _phantom: PhantomData<C>,
}

impl<I, C: Color> Iterator for ToSrgbIter<I, C>
    where I: Iterator<Item=C> {
    type Item = Srgb;

    fn next(&mut self) -> Option<Srgb> {
        self.iter.next().map(|v| v.to_srgb())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct ToHslIter<I, C: Color> {
    iter: I,
    _phantom: PhantomData<C>,
}

impl<I, C: Color> Iterator for ToHslIter<I, C>
    where I: Iterator<Item=C> {
    type Item = Hsl;

    fn next(&mut self) -> Option<Hsl> {
        self.iter.next().map(|v| v.to_hsl())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct ToHsvIter<I, C: Color> {
    iter: I,
    _phantom: PhantomData<C>,
}

impl<I, C: Color> Iterator for ToHsvIter<I, C>
    where I: Iterator<Item=C> {
    type Item = Hsv;

    fn next(&mut self) -> Option<Hsv> {
        self.iter.next().map(|v| v.to_hsv())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct ToYuvIter<I, C: Color> {
    iter: I,
    _phantom: PhantomData<C>,
}

impl<I, C: Color> Iterator for ToYuvIter<I, C>
    where I: Iterator<Item=C> {
    type Item = Yuv;

    fn next(&mut self) -> Option<Yuv> {
        self.iter.next().map(|v| v.to_yuv())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct ToXyzIter<I, C: Color> {
    iter: I,
    _phantom: PhantomData<C>,
}

impl<I, C: Color> Iterator for ToXyzIter<I, C>
    where I: Iterator<Item=C> {
    type Item = Xyz;

    fn next(&mut self) -> Option<Xyz> {
        self.iter.next().map(|v| v.to_xyz())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct ToLabIter<I, C: Color, IL: Illuminant, O: Observer> {
    iter: I,
    _phantom: PhantomData<C>,
    _phantom2: PhantomData<IL>,
    _phantom3: PhantomData<O>,
}

impl<I, C: Color, IL: Illuminant, O: Observer> Iterator for ToLabIter<I, C, IL, O>
    where I: Iterator<Item=C> {
    type Item = Lab<IL, O>;

    fn next(&mut self) -> Option<Lab<IL, O>> {
        self.iter.next().map(|v| v.to_lab())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct ToLchIter<I, C: Color, IL: Illuminant, O: Observer> {
    iter: I,
    _phantom: PhantomData<C>,
    _phantom2: PhantomData<IL>,
    _phantom3: PhantomData<O>,
}

impl<I, C: Color, IL: Illuminant, O: Observer> Iterator for ToLchIter<I, C, IL, O>
    where I: Iterator<Item=C> {
    type Item = Lch<IL, O>;

    fn next(&mut self) -> Option<Lch<IL, O>> {
        self.iter.next().map(|v| v.to_lch())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub trait ColorIter<T: Color>: Iterator<Item=T> {
    fn into_rgb(self) -> ToRgbIter<Self, Self::Item>
        where Self: Sized
    {
        ToRgbIter {
            iter: self,
            _phantom: PhantomData::default(),
        }
    }

    fn into_srgb(self) -> ToSrgbIter<Self, Self::Item>
        where Self: Sized
    {
        ToSrgbIter {
            iter: self,
            _phantom: PhantomData::default(),
        }
    }

    fn into_hsl(self) -> ToHslIter<Self, Self::Item>
        where Self: Sized
    {
        ToHslIter {
            iter: self,
            _phantom: PhantomData::default(),
        }
    }

    fn into_hsv(self) -> ToHsvIter<Self, Self::Item>
        where Self: Sized
    {
        ToHsvIter {
            iter: self,
            _phantom: PhantomData::default(),
        }
    }

    fn into_yuv(self) -> ToYuvIter<Self, Self::Item>
        where Self: Sized
    {
        ToYuvIter {
            iter: self,
            _phantom: PhantomData::default(),
        }
    }

    fn into_xyz(self) -> ToXyzIter<Self, Self::Item>
        where Self: Sized
    {
        ToXyzIter {
            iter: self,
            _phantom: PhantomData::default(),
        }
    }

    fn into_lab<IL: Illuminant, O: Observer>(self) -> ToLabIter<Self, Self::Item, IL, O>
        where Self: Sized
    {
        ToLabIter {
            iter: self,
            _phantom: PhantomData::default(),
            _phantom2: PhantomData::default(),
            _phantom3: PhantomData::default(),
        }
    }

    fn into_lch<IL: Illuminant, O: Observer>(self) -> ToLchIter<Self, Self::Item, IL, O>
        where Self: Sized
    {
        ToLchIter {
            iter: self,
            _phantom: PhantomData::default(),
            _phantom2: PhantomData::default(),
            _phantom3: PhantomData::default(),
        }
    }
}

impl<T: ?Sized, C: Color> ColorIter<C> for T where T: Iterator<Item=C> {}

pub trait ColorIterRef<'a, C: Color, T: 'a + Color>: Iterator<Item=&'a T> {
    fn into_rgb(self) -> ToRgbIter<Cloned<Self>, C>
        where Self: Sized
    {
        ToRgbIter {
            iter: self.cloned(),
            _phantom: PhantomData::default(),
        }
    }

    fn into_srgb(self) -> ToSrgbIter<Cloned<Self>, C>
        where Self: Sized
    {
        ToSrgbIter {
            iter: self.cloned(),
            _phantom: PhantomData::default(),
        }
    }

    fn into_hsl(self) -> ToHslIter<Cloned<Self>, C>
        where Self: Sized
    {
        ToHslIter {
            iter: self.cloned(),
            _phantom: PhantomData::default(),
        }
    }

    fn into_hsv(self) -> ToHsvIter<Cloned<Self>, C>
        where Self: Sized
    {
        ToHsvIter {
            iter: self.cloned(),
            _phantom: PhantomData::default(),
        }
    }

    fn into_yuv(self) -> ToYuvIter<Cloned<Self>, C>
        where Self: Sized
    {
        ToYuvIter {
            iter: self.cloned(),
            _phantom: PhantomData::default(),
        }
    }

    fn into_xyz(self) -> ToXyzIter<Cloned<Self>, C>
        where Self: Sized
    {
        ToXyzIter {
            iter: self.cloned(),
            _phantom: PhantomData::default(),
        }
    }

    fn into_lab<IL: Illuminant, O: Observer>(self) -> ToLabIter<Cloned<Self>, C, IL, O>
        where Self: Sized
    {
        ToLabIter {
            iter: self.cloned(),
            _phantom: PhantomData::default(),
            _phantom2: PhantomData::default(),
            _phantom3: PhantomData::default(),
        }
    }

    fn into_lch<IL: Illuminant, O: Observer>(self) -> ToLchIter<Cloned<Self>, C, IL, O>
        where Self: Sized
    {
        ToLchIter {
            iter: self.cloned(),
            _phantom: PhantomData::default(),
            _phantom2: PhantomData::default(),
            _phantom3: PhantomData::default(),
        }
    }
}

impl<'a, T: ?Sized, C: Color, T2: 'a + Color> ColorIterRef<'a, C, T2> for T where T: Iterator<Item=&'a T2> {}

#[cfg(test)]
mod tests {
    use crate::color::{Rgb, Hsl, Hsv, Yuv, ColorIter, ColorIterRef, Srgb, Xyz, Lab};
    use crate::color::illuminant::D65;
    use crate::color::observer::O2;

    const RGB_HSL: [((f32, f32, f32), (f32, f32, f32)); 15] = [
        ((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)),
        ((1.0, 1.0, 1.0), (0.0, 0.0, 1.0)),
        ((0.5, 0.5, 0.5), (0.0, 0.0, 0.5)),
        ((1.0, 0.0, 0.0), (0.0, 1.0, 0.5)),
        ((0.0, 1.0, 0.0), (0.33333334, 1.0, 0.5)),
        ((0.0, 0.0, 1.0), (0.6666667, 1.0, 0.5)),
        ((1.0, 0.5, 0.5), (0.0, 1.0, 0.75)),
        ((0.5, 1.0, 0.5), (0.33333334, 1.0, 0.75)),
        ((0.5, 0.5, 1.0), (0.6666667, 1.0, 0.75)),
        ((0.5, 0.0, 0.0), (0.0, 1.0, 0.25)),
        ((0.0, 0.5, 0.0), (0.33333334, 1.0, 0.25)),
        ((0.0, 0.0, 0.5), (0.6666667, 1.0, 0.25)),
        ((0.5, 0.0, 0.5), (0.8333333, 1.0, 0.25)),
        ((0.5, 0.5, 0.0), (0.16666667, 1.0, 0.25)),
        ((0.0, 0.5, 0.5), (0.5, 1.0, 0.25))];


    const RGB_HSV: [((f32, f32, f32), (f32, f32, f32)); 15] = [
        ((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)),
        ((1.0, 1.0, 1.0), (0.0, 0.0, 1.0)),
        ((0.5, 0.5, 0.5), (0.0, 0.0, 0.5)),
        ((1.0, 0.0, 0.0), (0.0, 1.0, 1.0)),
        ((0.0, 1.0, 0.0), (0.33333334, 1.0, 1.0)),
        ((0.0, 0.0, 1.0), (0.6666667, 1.0, 1.0)),
        ((1.0, 0.5, 0.5), (0.0, 0.5, 1.0)),
        ((0.5, 1.0, 0.5), (0.33333334, 0.5, 1.0)),
        ((0.5, 0.5, 1.0), (0.6666667, 0.5, 1.0)),
        ((0.5, 0.0, 0.0), (0.0, 1.0, 0.5)),
        ((0.0, 0.5, 0.0), (0.33333334, 1.0, 0.5)),
        ((0.0, 0.0, 0.5), (0.6666667, 1.0, 0.5)),
        ((0.5, 0.0, 0.5), (0.8333333, 1.0, 0.5)),
        ((0.5, 0.5, 0.0), (0.16666667, 1.0, 0.5)),
        ((0.0, 0.5, 0.5), (0.5, 1.0, 0.5))];

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

    const SRGB_XYZ: [((f32, f32, f32), (f32, f32, f32)); 9] = [
        ((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)),
        ((1.0, 1.0, 1.0), (0.950_456, 1.0, 1.088_754)),
        ((0.5, 0.5, 0.5), (0.203_436_69, 0.214_041_14, 0.233_038_15)),
        ((1.0, 0.0, 0.0), (0.412453, 0.212671, 0.019334)),
        ((0.0, 1.0, 0.0), (0.35758, 0.71516, 0.119193)),
        ((0.0, 0.0, 1.0), (0.180423, 0.072169, 0.950227)),
        ((1.0, 0.5, 0.5), (0.52760778, 0.3811918, 0.24823388)),
        ((0.5, 1.0, 0.5), (0.48447986, 0.77612748, 0.32671894)),
        ((0.5, 0.5, 1.0), (0.34524174, 0.27076301, 0.97987748)),
    ];

    const SRGB_LAB_65_2: [((f32, f32, f32), (f32, f32, f32)); 6] = [
        ((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)),
        ((1.0, 1.0, 1.0), (1.0, -0.000_019, 0.000_036)),
        ((0.5, 0.5, 0.5), (0.533_889_6, 0.0, 0.0)),
        ((1.0, 0.0, 0.0), (0.532_405_879_4, 0.625_721_16, 0.525_021_49)),
        ((0.0, 1.0, 0.0), (0.877_350_994_9, -0.673_304_92, 0.649_841_43)),
        ((0.0, 0.0, 1.0), (0.322_956_725_7, 0.618_637_43, -0.842_635_16)),
    ];

    #[test]
    fn test_to_rgb_iter() {
        let to: Vec<_> = RGB_HSL.iter().map(|(v, _)| Rgb::new(v.0, v.1, v.2)).collect();
        let from: Vec<_> = RGB_HSL.iter().map(|(_, v)| Hsl::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.into_iter().into_rgb().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_hsl_iter() {
        let from: Vec<_> = RGB_HSL.iter().map(|(v, _)| Rgb::new(v.0, v.1, v.2)).collect();
        let to: Vec<_> = RGB_HSL.iter().map(|(_, v)| Hsl::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.into_iter().into_hsl().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_hsv_iter() {
        let from: Vec<_> = RGB_HSV.iter().map(|(v, _)| Rgb::new(v.0, v.1, v.2)).collect();
        let to: Vec<_> = RGB_HSV.iter().map(|(_, v)| Hsv::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.into_iter().into_hsv().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_yuv_iter() {
        let from: Vec<_> = RGB_YUV.iter().map(|(v, _)| Rgb::new(v.0, v.1, v.2)).collect();
        let to: Vec<_> = RGB_YUV.iter().map(|(_, v)| Yuv::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.into_iter().into_yuv().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_xyz_iter() {
        let from: Vec<_> = SRGB_XYZ.iter().map(|(v, _)| Srgb::new(v.0, v.1, v.2)).collect();
        let to: Vec<_> = SRGB_XYZ.iter().map(|(_, v)| Xyz::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.into_iter().into_xyz().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_rgb_iter_ref() {
        let to: Vec<_> = RGB_HSL.iter().map(|(v, _)| Rgb::new(v.0, v.1, v.2)).collect();
        let from: Vec<_> = RGB_HSL.iter().map(|(_, v)| Hsl::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.iter().into_rgb().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_hsl_iter_ref() {
        let from: Vec<_> = RGB_HSL.iter().map(|(v, _)| Rgb::new(v.0, v.1, v.2)).collect();
        let to: Vec<_> = RGB_HSL.iter().map(|(_, v)| Hsl::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.iter().into_hsl().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_hsv_iter_ref() {
        let from: Vec<_> = RGB_HSV.iter().map(|(v, _)| Rgb::new(v.0, v.1, v.2)).collect();
        let to: Vec<_> = RGB_HSV.iter().map(|(_, v)| Hsv::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.iter().into_hsv().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_yuv_iter_ref() {
        let from: Vec<_> = RGB_YUV.iter().map(|(v, _)| Rgb::new(v.0, v.1, v.2)).collect();
        let to: Vec<_> = RGB_YUV.iter().map(|(_, v)| Yuv::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.iter().into_yuv().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_xyz_iter_ref() {
        let from: Vec<_> = SRGB_XYZ.iter().map(|(v, _)| Srgb::new(v.0, v.1, v.2)).collect();
        let to: Vec<_> = SRGB_XYZ.iter().map(|(_, v)| Xyz::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.iter().into_xyz().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_lab_iter_ref() {
        let from: Vec<_> = SRGB_LAB_65_2.iter().map(|(v, _)| Srgb::new(v.0, v.1, v.2)).collect();
        let to: Vec<_> = SRGB_LAB_65_2.iter().map(|(_, v)| Lab::<D65, O2>::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.iter().into_lab().collect();
        assert_eq!(to, result)
    }
}