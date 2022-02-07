use d10_codecs::{decode_buffer, decode_file, encode, EncodingFormat};

// Because reference images are u8 based and there might be rounding
// errors in all images tested, a delta of 2 should be save to not have
// false positives while still keeping false negative rate at a minimum
const ALLOWED_DELTA: f32 = 2.0 / 256.0;

fn test_decode(path: &str) {
    let result = decode_file(path).unwrap().buffer;
    let expected = decode_file(&format!("{}.png", path)).unwrap().buffer;

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

fn test_encode(path: &str) {
    let orig = decode_file("tests/images/test.png").unwrap().buffer;

    let mut buffer = vec![];
    encode(&mut buffer, &orig, EncodingFormat::webp_with_quality(95)).unwrap();

    let result = decode_buffer(&buffer).unwrap().buffer;
    let expected = decode_file(&format!("{}.png", path)).unwrap().buffer;

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
pub fn test_webp() {
    test_decode("tests/images/test.webp");
    test_encode("tests/images/test.webp");
}
