use crate::compose;
use d10_core::color::{Color, Rgb};
use d10_core::errors::ParseEnumError;
use d10_core::kernel::Kernel;
use d10_core::pixelbuffer::PixelBuffer;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EdgeDetection {
    Sobel,
    Laplace,
}

impl FromStr for EdgeDetection {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<EdgeDetection, Self::Err> {
        match value {
            "sobel" | "default" => Ok(EdgeDetection::Sobel),
            "laplace" => Ok(EdgeDetection::Laplace),
            _ => Err(ParseEnumError::new(value, "EdgeDetection")),
        }
    }
}

fn edge_detection_sobel(buffer: &PixelBuffer<Rgb>) -> PixelBuffer<Rgb> {
    let buffer_x = buffer.apply_kernel(&Kernel::new([
        [1.0, 0.0, -1.0],
        [2.0, 0.0, -2.0],
        [1.0, 0.0, -1.0],
    ]));

    let buffer_y = buffer.apply_kernel(&Kernel::new([
        [1.0, 2.0, 1.0],
        [0.0, 0.0, 0.0],
        [-1.0, -2.0, -1.0],
    ]));

    compose([&buffer_x, &buffer_y], Rgb::BLACK, |_, _, [img1, img2]| {
        let r = (img1.red() * img1.red() + img2.red() * img2.red()).sqrt();
        let g = (img1.green() * img1.green() + img2.green() * img2.green()).sqrt();
        let b = (img1.blue() * img1.blue() + img2.blue() * img2.blue()).sqrt();

        Rgb::new(r, g, b)
    })
}

fn edge_detection_laplace(buffer: &PixelBuffer<Rgb>) -> PixelBuffer<Rgb> {
    buffer
        .apply_kernel(&Kernel::new([
            [1.0, 1.0, 1.0],
            [1.0, -8.0, 1.0],
            [1.0, 1.0, 1.0],
        ]))
        .map_colors(|c| c.with_alpha(1.0))
}

pub fn edge_detection(buffer: &PixelBuffer<Rgb>, mode: EdgeDetection) -> PixelBuffer<Rgb> {
    match mode {
        EdgeDetection::Sobel => edge_detection_sobel(buffer),
        EdgeDetection::Laplace => edge_detection_laplace(buffer),
    }
}
