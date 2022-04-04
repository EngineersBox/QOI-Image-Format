use std::borrow::Borrow;
use std::io::{BufReader, ErrorKind, Read};
use std::marker::PhantomData;
use std::io::Error;
use arraydeque::{ArrayDeque, Wrapping};
use crate::color::color::RGBA;
use crate::encoding::common;
use crate::structure::qoi_image::QOIImage;

const QIO_SEEN_WINDOW: usize = 64;
const OPTION_ARRAY_INIT: Option<Box<RGBA>> = None;
const EOF_MARKER: [u8; 8] = [
    0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x01u8
];
const TAG_2BIT_MASK: u8 = 0b11000000u8;
const DIFF_RED_MASK: u8 = 0b00110000u8;
const DIFF_GREEN_MASK: u8 = 0b00001100u8;
const DIFF_BLUE_MASK: u8 = 0b00000011u8;
const RUN_LENGTH_MASK: u8 = 0b00111111u8;
const LUMA_GREEN_DIFF_MASK: u8 = 0b00111111u8;
const LUMA_DR_DG_MASK: u8 = 0b11110000u8;
const LUMA_DB_DG_MASK: u8 = 0b00001111u8;

pub struct Decoder<T: Read> {
    seen: [Option<Box<RGBA>>; QIO_SEEN_WINDOW],
    image: QOIImage,
    dequeue: ArrayDeque<[u8; 8], Wrapping>,
    phantom: PhantomData<T>,
}

impl<T: Read> Decoder<T> {
    pub fn new() -> Decoder<T> {
        return Decoder {
            seen: [OPTION_ARRAY_INIT; QIO_SEEN_WINDOW],
            image: QOIImage::new(),
            dequeue: ArrayDeque::new(),
            phantom: PhantomData,
        };
    }
    #[inline]
    fn index_seen(&self, rgba: &RGBA) -> &Option<Box<RGBA>> {
        self.seen[common::hash_rgba(rgba) as usize].borrow()
    }
    #[inline]
    fn insert_seen(&mut self, rgba: &RGBA) {
        self.seen[common::hash_rgba(rgba) as usize] = Some(Box::new(*rgba))
    }
    #[inline]
    fn last_pixel(&self) -> RGBA {
        match self.image.pixels.get(self.image.pixels.len() - 1) {
            Some(v) => *v,
            None => RGBA::new(),
        }
    }
    fn read_header(&mut self, reader: &mut BufReader<T>) -> Result<(), Error> {
        return self.image.header.read_buffer::<T>(reader);
    }
    fn process_op_rgb(&mut self, reader: &mut BufReader<T>) -> Result<(), Error> {
        let mut color_value: [u8; 3] = [0; 3];
        for i in 0..=2 {
            match self.dequeue.get(i) {
                Some(v) => color_value[i] = *v,
                None => return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "Expected to read RGBA byte, but attempted to access out of window range at index: {}",
                        i
                    ),
                )),
            };
        }
        let mut pixel: RGBA = RGBA::from_rgb_bytes(&color_value);
        // Back reference previous pixel alpha value
        pixel.a = self.last_pixel().a;
        self.insert_seen(&pixel);
        self.image.pixels.push(pixel);

        // Move past color bytes
        reader.read(&mut color_value)?;
        color_value.iter()
            .for_each(|byte: &u8| { self.dequeue.push_back(*byte); });
        return Ok(());
    }
    fn process_op_rgba(&mut self, reader: &mut BufReader<T>) -> Result<(), Error> {
        let mut color_value: [u8; 4] = [0; 4];
        for i in 0..=3 {
            match self.dequeue.get(i) {
                Some(v) => color_value[i] = *v,
                None => return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "Expected to read RGBA byte, but attempted to access out of window range at index: {}",
                        i
                    ),
                )),
            };
        }
        let pixel: RGBA = RGBA::from_rgba_bytes(&color_value);
        self.insert_seen(&pixel);
        self.image.pixels.push(pixel);

        // Move past color bytes
        reader.read(&mut color_value)?;
        color_value.iter()
            .for_each(|byte: &u8| { self.dequeue.push_back(*byte); });
        return Ok(());
    }
    fn process_op_index(&mut self, tag: u8) -> Result<(), Error> {
        let index: usize = (tag & 0b00111111u8) as usize;
        let indexed_pixel = match &self.seen[index] {
            Some(v) => (*v).as_ref().clone(),
            None => return Err(Error::new(
                ErrorKind::InvalidData,
                format!("No pixel value found at seen index: {}", index),
            )),
        };
        self.image.pixels.push(indexed_pixel);
        return Ok(());
    }
    fn process_op_diff(&mut self, tag: u8) -> Result<(), Error> {
        let last_pixel: RGBA = self.last_pixel();
        self.image.pixels.push(RGBA{
            r: last_pixel.r.wrapping_add((tag & DIFF_RED_MASK).wrapping_sub(2)),
            g: last_pixel.g.wrapping_add((tag & DIFF_GREEN_MASK).wrapping_sub(2)),
            b: last_pixel.b.wrapping_add((tag & DIFF_BLUE_MASK).wrapping_sub(2)),
            a: last_pixel.a,
        });
        return Ok(());
    }
    fn process_op_luma(&mut self, reader: &mut BufReader<T>, tag: u8) -> Result<(), Error> {
        let last_pixel: RGBA = self.last_pixel();
        let unbiased_diff_green: u8 = (tag & LUMA_GREEN_DIFF_MASK).wrapping_sub(32);
        let difference_byte: u8 = match self.dequeue.get(0) {
            Some(v) => *v,
            None => return Err(Error::new(
                ErrorKind::InvalidData,
                "Expected to read RGBA byte, but attempted to access out of window range at index: 0"
            )),
        };
        self.image.pixels.push(RGBA {
            r: last_pixel.r.wrapping_add(((difference_byte & LUMA_DR_DG_MASK) >> 4).wrapping_sub(8).wrapping_add(unbiased_diff_green)),
            g: last_pixel.g.wrapping_add((tag & LUMA_GREEN_DIFF_MASK).wrapping_sub(32)),
            b: last_pixel.b.wrapping_add((difference_byte & LUMA_DB_DG_MASK).wrapping_sub(8).wrapping_add(unbiased_diff_green)),
            a: last_pixel.a,
        });

        // Move past difference byte
        let mut next_byte: [u8; 1] = [0; 1];
        reader.read(&mut next_byte)?;
        self.dequeue.push_back(next_byte[0]);
        return Ok(());
    }
    fn process_op_run(&mut self, tag: u8) -> Result<(), Error> {
        let last_pixel: RGBA = self.last_pixel();
        for _ in 0..((tag & RUN_LENGTH_MASK) + 1) {
            self.image.pixels.push(last_pixel.clone());
        }
        return Ok(());
    }
    fn init_dequeue(&mut self, reader: &mut BufReader<T>) -> Result<(), Error> {
        self.dequeue.clear();
        let mut first_bytes: [u8; 8] = [0; 8];
        reader.read(&mut first_bytes)?;
        for byte in first_bytes {
            self.dequeue.push_back(byte);
        }
        return Ok(());
    }
    pub fn decode_buffer(&mut self, reader: &mut BufReader<T>) -> Result<Vec<RGBA>, Error> {
        self.read_header(reader)?;
        self.init_dequeue(reader)?;
        let mut next: [u8; 1] = [0; 1];
        let mut front: u8 = 0;
        loop {
            if self.dequeue.iter().eq(EOF_MARKER.iter()) {
                break;
            }
            front = match self.dequeue.front() {
                Some(v) => *v,
                None => return Err(Error::new(
                    ErrorKind::UnexpectedEof,
                    "Unexpected EOF",
                )),
            };
            reader.read(&mut next)?;
            self.dequeue.push_back(next[0]);
            match front {
                0b11111110u8 => self.process_op_rgb(reader)?,
                0b11111111u8 => self.process_op_rgba(reader)?,
                _ => match front & TAG_2BIT_MASK {
                    0b00000000u8 => self.process_op_index(front)?,
                    0b01000000u8 => self.process_op_diff(front)?,
                    0b10000000u8 => self.process_op_luma(reader, front)?,
                    0b11000000u8 => self.process_op_run(front)?,
                    _ => return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Unknown tag type: {}", front)
                    )),
                },
            }
        }
        todo!()
    }
}