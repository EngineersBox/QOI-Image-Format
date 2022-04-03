use std::io::{BufReader, Error, ErrorKind, Read};
use crate::structure::informational::{QOIChannels, QOIColorSpace};

const QOI_MAGIC_STRING: [u8; 4] = ['q' as u8, 'o' as u8, 'i' as u8, 'f' as u8];

#[derive(Debug,Default,Eq,PartialEq,Clone)]
pub struct QOIHeader {
    pub magic: [u8; 4],
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
    pub fn read_buffer<T: std::io::Read>(&mut self, buf: &mut BufReader<T>) -> Result<(), Error> {
        buf.read(&mut self.magic)?;
        let mut be_int_32: [u8; 4] = [0; 4];
        buf.read(&mut be_int_32)?;
        self.width = u32::from_be_bytes(be_int_32);
        buf.read(&mut be_int_32)?;
        self.height = u32::from_be_bytes(be_int_32);
        let mut int_u8: [u8; 1] = [0; 1];
        buf.read(&mut int_u8)?;
        if QOIChannels::from(int_u8[0]) == QOIChannels::DEFAULT {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Unknown channel format: {}", int_u8[0])
            ));
        }
        self.channels = int_u8[0];
        buf.read(&mut int_u8)?;
        if QOIColorSpace::from(int_u8[0]) == QOIColorSpace::DEFAULT {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Unknown color space format: {}", int_u8[0])
            ));
        }
        self.color_space = int_u8[0];
        return Ok(());
    }
}