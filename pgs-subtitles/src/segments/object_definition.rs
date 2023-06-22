use deku::ctx::Endian;
use deku::prelude::*;
use std::collections::VecDeque;
use std::slice::Iter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::shared::{ColorId, ObjectId};

#[deku_derive(DekuRead)]
#[deku(endian = "endian", ctx = "endian: Endian")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ObjectDefinition {
    pub id: ObjectId,
    pub version: u8,
    #[deku(bits = "1")]
    pub is_first_in_sequence: bool,
    #[deku(bits = "1", pad_bits_after = "6")]
    pub is_last_in_sequence: bool,
    #[deku(bytes = "3", temp)]
    pub data_size: u32,
    pub width: u16,
    pub height: u16,
    #[deku(bytes_read = "(data_size - 4)")]
    pub rle_data: Vec<u8>,
}

impl ObjectDefinition {
    pub fn decoded_iter(&self) -> RLEIterator {
        RLEIterator::from_rle_data(&self.rle_data)
    }
}

//

pub struct RLEIterator<'a> {
    internal_iterator: Iter<'a, u8>,
    buffer: VecDeque<ColorId>,
}

impl<'a> RLEIterator<'a> {
    const COLOR_ZERO: ColorId = ColorId { raw_id: 0x00 };

    fn from_rle_data(rle_data: &'a [u8]) -> Self {
        Self {
            internal_iterator: rle_data.iter(),
            buffer: VecDeque::new(),
        }
    }

    fn insert_many(&mut self, num: usize, color: ColorId) {
        for _ in 0..num {
            self.buffer.push_back(color);
        }
    }
}

impl<'a> Iterator for RLEIterator<'a> {
    type Item = ColorId;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.pop_front().or_else(|| {
            let byte1 = self.internal_iterator.next()?;

            if *byte1 != 0x00 {
                return Some(ColorId { raw_id: *byte1 });
            } else {
                let byte2 = *self.internal_iterator.next()?;

                if byte2 == 0x00 {
                    None
                } else if byte2 >> 6 == 0b00 {
                    let byte2 = byte2 & 0x3F;
                    self.insert_many(usize::from(byte2) - 1, Self::COLOR_ZERO);

                    Some(Self::COLOR_ZERO)
                } else if byte2 >> 6 == 0b01 {
                    let byte2 = byte2 & 0x3F;
                    let byte3: u8 = *self.internal_iterator.next()?;
                    let num_entries: u16 = u16::from(byte2) << 8 | u16::from(byte3);
                    self.insert_many(usize::from(num_entries) - 1, Self::COLOR_ZERO);

                    Some(Self::COLOR_ZERO)
                } else if byte2 >> 6 == 0b10 {
                    let byte2 = byte2 & 0x3F;
                    let byte3 = *self.internal_iterator.next()?;
                    let color = ColorId { raw_id: byte3 };
                    self.insert_many(usize::from(byte2) - 1, color);

                    Some(color)
                } else if byte2 >> 6 == 0b11 {
                    let byte2 = byte2 & 0x3F;
                    let byte3: u8 = *self.internal_iterator.next()?;
                    let byte4: u8 = *self.internal_iterator.next()?;
                    let num_entries: u16 = u16::from(byte2) << 8 | u16::from(byte3);
                    let color = ColorId { raw_id: byte4 };
                    self.insert_many(usize::from(num_entries) - 1, color);

                    Some(color)
                } else {
                    unreachable!("two bits have more than four values?!")
                }
            }
        })
    }
}
