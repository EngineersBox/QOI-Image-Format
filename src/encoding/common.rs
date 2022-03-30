use crate::color::color::RGBA;

#[inline]
pub fn hash(r: u8, g: u8, b: u8, a: u8) -> u8 {
    (r * 3 + g * 5 + b * 7 + a * 11) % 64
}

pub fn hash_rgba(rgba: RGBA) -> u8 {
    return hash(rgba.r, rgba.g, rgba.b, rgba.a);
}