use crate::color::{Color, Rgb, Srgb, Hsl, Hsv, Yuv};

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
}

impl<'a, T: ?Sized, C: Color, T2: 'a + Color> ColorIterRef<'a, C, T2> for T where T: Iterator<Item=&'a T2> {}

#[cfg(test)]
mod tests {
    use crate::color::{Rgb, Hsl, Hsv, Yuv, ColorIter, ColorIterRef};

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
}