use crate::color::color::RGBA;
use crate::structure::qoi_header::QOIHeader;

#[derive(Debug,Default,Eq,PartialEq,Clone)]
pub struct QOIImage {
    pub header: QOIHeader,
    pub pixels: Vec<RGBA>,
}

impl QOIImage {
    pub fn new() -> QOIImage {
        return QOIImage {
            header: QOIHeader::new(),
            pixels: Vec::new(),
        };
    }
}