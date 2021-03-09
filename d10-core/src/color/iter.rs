use crate::color::{Color, RGB, SRGB, HSL, HSV, YUV};

use std::iter::Cloned;
use std::marker::PhantomData;

pub struct ToRGBIter<I, C: Color> {
    iter: I,
    _phantom: PhantomData<C>,
}

impl<I, C: Color> Iterator for ToRGBIter<I, C>
    where I: Iterator<Item=C> {
    type Item = RGB;

    fn next(&mut self) -> Option<RGB> {
        self.iter.next().map(|v| v.to_rgb())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct ToSRGBIter<I, C: Color> {
    iter: I,
    _phantom: PhantomData<C>,
}

impl<I, C: Color> Iterator for ToSRGBIter<I, C>
    where I: Iterator<Item=C> {
    type Item = SRGB;

    fn next(&mut self) -> Option<SRGB> {
        self.iter.next().map(|v| v.to_srgb())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct ToHSLIter<I, C: Color> {
    iter: I,
    _phantom: PhantomData<C>,
}

impl<I, C: Color> Iterator for ToHSLIter<I, C>
    where I: Iterator<Item=C> {
    type Item = HSL;

    fn next(&mut self) -> Option<HSL> {
        self.iter.next().map(|v| v.to_hsl())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct ToHSVIter<I, C: Color> {
    iter: I,
    _phantom: PhantomData<C>,
}

impl<I, C: Color> Iterator for ToHSVIter<I, C>
    where I: Iterator<Item=C> {
    type Item = HSV;

    fn next(&mut self) -> Option<HSV> {
        self.iter.next().map(|v| v.to_hsv())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct ToYUVIter<I, C: Color> {
    iter: I,
    _phantom: PhantomData<C>,
}

impl<I, C: Color> Iterator for ToYUVIter<I, C>
    where I: Iterator<Item=C> {
    type Item = YUV;

    fn next(&mut self) -> Option<YUV> {
        self.iter.next().map(|v| v.to_yuv())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub trait ColorIter<T: Color>: Iterator<Item=T> {
    fn into_rgb(self) -> ToRGBIter<Self, Self::Item>
        where Self: Sized
    {
        ToRGBIter {
            iter: self,
            _phantom: PhantomData::default(),
        }
    }

    fn into_srgb(self) -> ToSRGBIter<Self, Self::Item>
        where Self: Sized
    {
        ToSRGBIter {
            iter: self,
            _phantom: PhantomData::default(),
        }
    }

    fn into_hsl(self) -> ToHSLIter<Self, Self::Item>
        where Self: Sized
    {
        ToHSLIter {
            iter: self,
            _phantom: PhantomData::default(),
        }
    }

    fn into_hsv(self) -> ToHSVIter<Self, Self::Item>
        where Self: Sized
    {
        ToHSVIter {
            iter: self,
            _phantom: PhantomData::default(),
        }
    }

    fn into_yuv(self) -> ToYUVIter<Self, Self::Item>
        where Self: Sized
    {
        ToYUVIter {
            iter: self,
            _phantom: PhantomData::default(),
        }
    }
}

impl<T: ?Sized, C: Color> ColorIter<C> for T where T: Iterator<Item=C> {}

pub trait ColorIterRef<'a, C: Color, T: 'a + Color>: Iterator<Item=&'a T> {
    fn into_rgb(self) -> ToRGBIter<Cloned<Self>, C>
        where Self: Sized
    {
        ToRGBIter {
            iter: self.cloned(),
            _phantom: PhantomData::default(),
        }
    }

    fn into_srgb(self) -> ToSRGBIter<Cloned<Self>, C>
        where Self: Sized
    {
        ToSRGBIter {
            iter: self.cloned(),
            _phantom: PhantomData::default(),
        }
    }

    fn into_hsl(self) -> ToHSLIter<Cloned<Self>, C>
        where Self: Sized
    {
        ToHSLIter {
            iter: self.cloned(),
            _phantom: PhantomData::default(),
        }
    }

    fn into_hsv(self) -> ToHSVIter<Cloned<Self>, C>
        where Self: Sized
    {
        ToHSVIter {
            iter: self.cloned(),
            _phantom: PhantomData::default(),
        }
    }

    fn into_yuv(self) -> ToYUVIter<Cloned<Self>, C>
        where Self: Sized
    {
        ToYUVIter {
            iter: self.cloned(),
            _phantom: PhantomData::default(),
        }
    }
}

impl<'a, T: ?Sized, C: Color, T2: 'a + Color> ColorIterRef<'a, C, T2> for T where T: Iterator<Item=&'a T2> {}

#[cfg(test)]
mod tests {
    use crate::color::{RGB, HSL, HSV, YUV, ColorIter, ColorIterRef};

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
        let to: Vec<_> = RGB_HSL.iter().map(|(v, _)| RGB::new(v.0, v.1, v.2)).collect();
        let from: Vec<_> = RGB_HSL.iter().map(|(_, v)| HSL::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.into_iter().into_rgb().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_hsl_iter() {
        let from: Vec<_> = RGB_HSL.iter().map(|(v, _)| RGB::new(v.0, v.1, v.2)).collect();
        let to: Vec<_> = RGB_HSL.iter().map(|(_, v)| HSL::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.into_iter().into_hsl().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_hsv_iter() {
        let from: Vec<_> = RGB_HSV.iter().map(|(v, _)| RGB::new(v.0, v.1, v.2)).collect();
        let to: Vec<_> = RGB_HSV.iter().map(|(_, v)| HSV::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.into_iter().into_hsv().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_yuv_iter() {
        let from: Vec<_> = RGB_YUV.iter().map(|(v, _)| RGB::new(v.0, v.1, v.2)).collect();
        let to: Vec<_> = RGB_YUV.iter().map(|(_, v)| YUV::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.into_iter().into_yuv().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_rgb_iter_ref() {
        let to: Vec<_> = RGB_HSL.iter().map(|(v, _)| RGB::new(v.0, v.1, v.2)).collect();
        let from: Vec<_> = RGB_HSL.iter().map(|(_, v)| HSL::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.iter().into_rgb().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_hsl_iter_ref() {
        let from: Vec<_> = RGB_HSL.iter().map(|(v, _)| RGB::new(v.0, v.1, v.2)).collect();
        let to: Vec<_> = RGB_HSL.iter().map(|(_, v)| HSL::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.iter().into_hsl().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_hsv_iter_ref() {
        let from: Vec<_> = RGB_HSV.iter().map(|(v, _)| RGB::new(v.0, v.1, v.2)).collect();
        let to: Vec<_> = RGB_HSV.iter().map(|(_, v)| HSV::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.iter().into_hsv().collect();
        assert_eq!(to, result)
    }

    #[test]
    fn test_to_yuv_iter_ref() {
        let from: Vec<_> = RGB_YUV.iter().map(|(v, _)| RGB::new(v.0, v.1, v.2)).collect();
        let to: Vec<_> = RGB_YUV.iter().map(|(_, v)| YUV::new(v.0, v.1, v.2)).collect();
        let result: Vec<_> = from.iter().into_yuv().collect();
        assert_eq!(to, result)
    }
}