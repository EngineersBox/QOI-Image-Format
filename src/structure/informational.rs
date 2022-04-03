use crate::enum_convertable;

enum_convertable!(u8
    #[derive(Debug,Eq,PartialEq,Clone)]
    pub enum QOIChannels {
        DEFAULT => 0u8,
        RGB => 3u8,
        RGBA => 4u8,
    }
);

enum_convertable!(u8
    #[derive(Debug,Eq,PartialEq,Clone)]
    pub enum QOIColorSpace {
        DEFAULT => 2u8,
        SRGB_ALPHA_LINEAR => 0u8,
        ALL_LINEAR => 1u8,
    }
);