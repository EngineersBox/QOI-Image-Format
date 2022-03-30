use crate::enum_convertable;

const QOI_MAGIC_STRING: [char; 4] = ['q', 'o', 'i', 'f'];

enum_convertable!(
    u8
    pub enum QOIChannels {
        DEFAULT => 0u8,
        RGB => 3u8,
        RGBA => 4u8,
    }
);

enum_convertable!(
    u8
    pub enum QOIColorSpace {
        DEFAULT => 2u8,
        SRGB_ALPHA_LINEAR => 0u8,
        ALL_LINEAR => 1u8,
    }
);

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