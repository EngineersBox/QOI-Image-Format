use crate::structure::informational::{QOIChannels, QOIColorSpace};

const QOI_MAGIC_STRING: [char; 4] = ['q', 'o', 'i', 'f'];

#[derive(Debug)]
pub struct QOIHeader {
    pub magic: [char; 4],
    pub width: u32, // Big endian
    pub height: u32, // Big endian
    pub channels: u8, // 3 = RGB, 4 = RGBA
    pub color_space: u8, // 0 = sRGB with linear alpha, 1 = all channels linear
}

impl QOIHeader {
    pub fn new() -> QOIHeader {
        return QOIHeader {
            magic: QOI_MAGIC_STRING,
            width: 0,
            height: 0,
            channels: QOIChannels::RGB.into(),
            color_space: QOIColorSpace::SRGB_ALPHA_LINEAR.into(),
        };
    }
}