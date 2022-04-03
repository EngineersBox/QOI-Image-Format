use crate::enum_convertable;

enum_convertable!(u8
    #[derive(Debug)]
    pub enum QIOTagSignatureMask {
        DEFAULT => 0b11111110u8,
        RGB => 0b11111110u8,
        RGBA => 0b11111111u8,
        INDEX => 0b00000000u8,
        DIFF => 0b01000000u8,
        LUMA => 0b10000000u8,
        RUN => 0b11000000u8,
    }
);