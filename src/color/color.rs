#[derive(Debug,Clone)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    #[inline]
    pub fn to_rgb_bytes(&self) -> [u8; 3] {
        return [self.r, self.g, self.b];
    }
    #[inline]
    pub fn from_rgb_bytes(bytes: &[u8; 3]) -> Self {
        return RGBA {
            r: bytes[0],
            g: bytes[1],
            b: bytes[2],
            a: 255u8,
        };
    }
    #[inline]
    pub fn to_rgba_bytes(&self) -> [u8; 4] {
        return [self.r, self.g, self.b, self.a];
    }
    #[inline]
    pub fn from_rgba_bytes(bytes: &[u8; 4]) -> Self {
        return RGBA {
            r: bytes[0],
            g: bytes[1],
            b: bytes[2],
            a: bytes[3],
        };
    }
}