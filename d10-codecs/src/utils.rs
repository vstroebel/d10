/// Convert color channel value between 0.0 and 1.0 into an u8
pub(crate) fn as_u8(value: f32) -> u8 {
    value.clamp(0.0, 1.0) as u8
}
