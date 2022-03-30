use crate::color::color::RGBA;

const QIO_SEEN_WINDOW: usize = 64;

pub struct Decoder {
    pub seen: Vec<RGBA>,
}

impl Decoder {
    fn new() -> Decoder {
        return Decoder {
            seen: Vec::with_capacity(QIO_SEEN_WINDOW),
        };
    }
    fn process_
}