use d10_core::pixelbuffer::PixelBuffer;
use d10_core::color::Color;

pub fn try_compose<E, C, F, const N: usize>(buffers: [&PixelBuffer<C>; N], default: C, mut func: F) -> Result<PixelBuffer<C>, E>
    where
        C: Color,
        F: FnMut(u32, u32, [C; N]) -> Result<C, E>
{
    assert!(N > 0);

    let width = buffers.iter().map(|i| i.width()).max().unwrap();
    let height = buffers.iter().map(|i| i.height()).max().unwrap();

    let mut pixels = [default; N];

    PixelBuffer::try_new_from_func(width, height, |x, y| {
        for (i, v) in pixels.iter_mut().enumerate() {
            *v = *buffers[i].get_pixel_optional(x as i32, y as i32).unwrap_or(&default);
        }
        func(x, y, pixels)
    })
}

pub fn compose<C, F, const N: usize>(buffers: [&PixelBuffer<C>; N], default: C, mut func: F) -> PixelBuffer<C>
    where
        C: Color,
        F: FnMut(u32, u32, [C; N]) -> C
{
    try_compose::<(), C, _, N>(buffers, default, |x, y, colors| Ok(func(x, y, colors))).unwrap()
}

pub fn try_compose_slice<E, C, F>(buffers: &[&PixelBuffer<C>], default: C, mut func: F) -> Result<PixelBuffer<C>, E>
    where
        C: Color,
        F: FnMut(u32, u32, &[C]) -> Result<C, E>
{
    assert!(!buffers.is_empty());

    let width = buffers.iter().map(|i| i.width()).max().unwrap();
    let height = buffers.iter().map(|i| i.height()).max().unwrap();

    let mut pixels = vec![default; buffers.len()];

    PixelBuffer::try_new_from_func(width, height, |x, y| {
        for (i, v) in pixels.iter_mut().enumerate() {
            *v = *buffers[i].get_pixel_optional(x as i32, y as i32).unwrap_or(&default);
        }
        func(x, y, &pixels)
    })
}

pub fn compose_slice<C, F>(buffers: &[&PixelBuffer<C>], default: C, mut func: F) -> PixelBuffer<C>
    where
        C: Color,
        F: FnMut(u32, u32, &[C]) -> C
{
    try_compose_slice::<(), C, _>(buffers, default, |x, y, colors| Ok(func(x, y, colors))).unwrap()
}


#[cfg(test)]
mod tests {
    use d10_core::pixelbuffer::PixelBuffer;
    use d10_core::color::Rgb;
    use super::*;

    #[test]
    fn test_compose() {
        let b1 = PixelBuffer::new_with_color(4, 2, Rgb::GREEN);
        let b2 = PixelBuffer::new_with_color(2, 5, Rgb::BLUE);
        let b3 = PixelBuffer::new_with_color(2, 2, Rgb::RED);

        let result = compose([&b1, &b2, &b3], Rgb::NONE, |_, _, colors| {
            colors.iter().fold(Rgb::NONE, |c1, c2| c1.alpha_blend(*c2))
        });

        assert_eq!(result.width(), 4);
        assert_eq!(result.height(), 5);

        assert_eq!(result.get_pixel(3, 0), &Rgb::GREEN);
        assert_eq!(result.get_pixel(0, 4), &Rgb::BLUE);
        assert_eq!(result.get_pixel(1, 1), &Rgb::RED);
        assert_eq!(result.get_pixel(3, 4), &Rgb::default());
    }

    #[test]
    fn test_compose_slice() {
        let b1 = PixelBuffer::new_with_color(4, 2, Rgb::GREEN);
        let b2 = PixelBuffer::new_with_color(2, 5, Rgb::BLUE);
        let b3 = PixelBuffer::new_with_color(2, 2, Rgb::RED);

        let result = compose_slice(&[&b1, &b2, &b3], Rgb::NONE, |_, _, colors| {
            colors.iter().fold(Rgb::NONE, |c1, c2| c1.alpha_blend(*c2))
        });

        assert_eq!(result.width(), 4);
        assert_eq!(result.height(), 5);

        assert_eq!(result.get_pixel(3, 0), &Rgb::GREEN);
        assert_eq!(result.get_pixel(0, 4), &Rgb::BLUE);
        assert_eq!(result.get_pixel(1, 1), &Rgb::RED);
        assert_eq!(result.get_pixel(3, 4), &Rgb::default());
    }
}