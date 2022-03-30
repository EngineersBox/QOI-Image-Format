use std::borrow::Borrow;
use std::io::BufReader;
use std::marker::PhantomData;
use std::io::Error;
use crate::color::color::RGBA;
use crate::encoding::common;
use crate::structure::qoi_header::QOIHeader;

const QIO_SEEN_WINDOW: usize = 64;

pub struct Decoder<T> {
    seen: [Option<Box<RGBA>>; QIO_SEEN_WINDOW],
    header: QOIHeader,
    pixels: Vec<RGBA>,
    phantom: PhantomData<T>,
}

const OPTION_ARRAY_INIT: Option<Box<RGBA>> = None;

impl<T> Decoder<T> {
    pub fn new() -> Decoder<T> {
        return Decoder {
            seen: [OPTION_ARRAY_INIT; QIO_SEEN_WINDOW],
            header: QOIHeader::new(),
            pixels: Vec::new(),
            phantom: PhantomData,
        };
    }
    #[inline]
    fn index_seen(&self, rgba: &RGBA) -> &Option<Box<RGBA>> {
        self.seen[common::hash_rgba(rgba) as usize].borrow()
    }
    fn read_header(&self, reader: &BufReader<T>) -> Result<(), Error> {
        todo!()
    }
    fn process_op_rgb(&self, reader: &BufReader<T>) -> Result<RGBA, Error> {
        todo!()
    }
    fn process_op_rgba(&self, reader: &BufReader<T>) -> Result<RGBA, Error> {
        todo!()
    }
    fn process_op_index(&self, reader: &BufReader<T>) -> Result<RGBA, Error> {
        todo!()
    }
    fn process_op_diff(&self, reader: &BufReader<T>) -> Result<RGBA, Error> {
        // Prev: self.pixels.get(self.pixels.len() - 1)
        todo!()
    }
    fn process_op_luma(&self, reader: &BufReader<T>) -> Result<RGBA, Error> {
        // Prev: self.pixels.get(self.pixels.len() - 1)
        todo!()
    }
    fn process_op_run(&self, reader: &BufReader<T>) -> Result<Vec<RGBA>, Error> {
        todo!()
    }
    pub fn decode_buffer(&self, reader: &BufReader<T>) -> Result<Vec<RGBA>, Error> {
        todo!()
    }
}