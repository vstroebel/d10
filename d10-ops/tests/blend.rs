use d10_codecs::decode_file;
use d10_core::color::Rgb;
use d10_core::pixelbuffer::PixelBuffer;
use d10_ops::BlendOp;

// Because reference images are u8 based and there might be rounding
// errors in all images tested, a delta of 2 should be save to not have
// false positives while still keeping false negative rate at a minimum
const ALLOWED_DELTA: f32 = 2.0 / 256.0;

fn load(path: &str) -> PixelBuffer<Rgb> {
    decode_file(path).unwrap().buffer
}

fn test_image(blend_op: BlendOp, expected: &str) {
    let img1 = load("tests/images/small_1.png");
    let img2 = load("tests/images/small_2.png");
    let expected = load(&format!("tests/images/blend/{}", expected));
    let result = d10_ops::blend_image(&img1, &img2, blend_op, 0.3);

    assert_eq!(expected.width(), result.width());
    assert_eq!(expected.height(), result.height());

    for ((x, y, c1), &c2) in expected.enumerate().zip(result.data().iter()) {
        for i in 0..4 {
            if (c1.data[i] - c2.data[i]).abs() > ALLOWED_DELTA {
                panic!("Expected {} got {} at position {}x{}", c1, c2, x, y);
            }
        }
    }
}

#[test]
pub fn test_normal() {
    test_image(BlendOp::Normal, "normal.png");
}

#[test]
pub fn test_addition() {
    test_image(BlendOp::Addition, "addition.png");
}

#[test]
pub fn test_subtract() {
    test_image(BlendOp::Subtract, "subtract.png");
}

#[test]
pub fn test_darken() {
    test_image(BlendOp::Darken, "darken.png");
}

#[test]
pub fn test_lighten() {
    test_image(BlendOp::Lighten, "lighten.png");
}


