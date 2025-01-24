#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use crate::*;
use std::ops::Deref;
use std::slice::Iter;

#[derive(Debug)]
pub struct SliceReader<'a> {
    slice: &'a [u8],
    pos: usize,
}

impl<'a> From<&'a [u8]> for SliceReader<'a> {
    fn from(value: &'a [u8]) -> Self {
        SliceReader {
            slice: value,
            pos: 0,
        }
    }
}

impl<'a, N: generic_array::ArrayLength> From<&'a mut ArrayU8<N>> for SliceReader<'a> {
    fn from(value: &'a mut ArrayU8<N>) -> SliceReader<'a> {
        SliceReader {
            slice: value.as_mut_slice(),
            pos: 0,
        }
    }
}

impl<'a> SliceReader<'a> {
    pub fn from_array(slice: &'a [u8]) -> Self {
        SliceReader { slice, pos: 0 }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn peek_u8(&mut self) -> u8 {
        self.slice[self.pos]
    }

    pub fn peek_u16(&mut self) -> u16 {
        u16::from_be_bytes([self.slice[self.pos], self.slice[self.pos + 1]])
    }

    pub fn peek_u32(&mut self) -> u32 {
        u32::from_be_bytes([
            self.slice[self.pos],
            self.slice[self.pos + 1],
            self.slice[self.pos + 2],
            self.slice[self.pos + 3],
        ])
    }

    pub fn peek_u64(&mut self) -> u64 {
        u64::from_be_bytes([
            self.slice[self.pos],
            self.slice[self.pos + 1],
            self.slice[self.pos + 2],
            self.slice[self.pos + 3],
            self.slice[self.pos + 4],
            self.slice[self.pos + 5],
            self.slice[self.pos + 6],
            self.slice[self.pos + 7],
        ])
    }

    pub fn read_u8(&mut self) -> u8 {
        self.pos += 1;
        self.slice[self.pos - 1]
    }

    pub fn read_u16(&mut self) -> u16 {
        self.pos += 2;
        u16::from_be_bytes([self.slice[self.pos - 2], self.slice[self.pos - 1]])
    }

    pub fn read_u32(&mut self) -> u32 {
        self.pos += 4;
        u32::from_be_bytes([
            self.slice[self.pos - 4],
            self.slice[self.pos - 3],
            self.slice[self.pos - 2],
            self.slice[self.pos - 1],
        ])
    }

    pub fn read_u64(&mut self) -> u64 {
        self.pos += 8;
        u64::from_be_bytes([
            self.slice[self.pos - 8],
            self.slice[self.pos - 7],
            self.slice[self.pos - 6],
            self.slice[self.pos - 5],
            self.slice[self.pos - 4],
            self.slice[self.pos - 3],
            self.slice[self.pos - 2],
            self.slice[self.pos - 1],
        ])
    }

    pub fn iter_from_current_pos(&self) -> Iter<u8> {
        self.slice[self.pos..].iter()
    }

    pub fn skip(&mut self, n: usize) {
        self.pos += n;
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub fn as_ref(&self) -> &[u8] {
        self.slice
    }

    pub fn read_slice(&mut self, len: usize) -> &'a [u8] {
        self.pos += len;
        &self.slice[self.pos - len..self.pos]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_slice_reader() {
        let slice = [
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
            16u8,
        ];
        let mut reader = SliceReader::from_array(&slice);
        assert_eq!(
            reader.slice,
            [
                0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                15u8, 16u8
            ]
        );
        assert_eq!(reader.pos(), 0);
        assert_eq!(reader.peek_u8(), 0u8);
        assert_eq!(reader.read_u8(), 0);
        assert_eq!(reader.pos(), 1);
        assert_eq!(reader.peek_u16(), 1u16 << 8 | 2u16);
        assert_eq!(reader.read_u16(), 1u16 << 8 | 2u16);
        assert_eq!(reader.pos(), 3);
        assert_eq!(
            reader.peek_u32(),
            3u32 << 24 | 4u32 << 16 | 5u32 << 8 | 6u32
        );
        assert_eq!(
            reader.read_u32(),
            3u32 << 24 | 4u32 << 16 | 5u32 << 8 | 6u32
        );
        assert_eq!(reader.pos(), 7);
        assert_eq!(
            reader.peek_u64(),
            7u64 << 56
                | 8u64 << 48
                | 9u64 << 40
                | 10u64 << 32
                | 11u64 << 24
                | 12u64 << 16
                | 13u64 << 8
                | 14u64
        );
        assert_eq!(
            reader.read_u64(),
            7u64 << 56
                | 8u64 << 48
                | 9u64 << 40
                | 10u64 << 32
                | 11u64 << 24
                | 12u64 << 16
                | 13u64 << 8
                | 14u64
        );
        assert_eq!(reader.pos(), 15);
        assert_eq!(reader.slice, reader.as_ref());
        reader.set_pos(1);
        assert_eq!(reader.pos, 1);
        reader.skip(2);
        assert_eq!(reader.pos, 3);
        assert_eq!(reader.read_slice(2), &slice[3..5]);
        assert_eq!(reader.pos(), 5);
        assert_eq!(reader.read_slice(4), &slice[5..9]);
        assert_eq!(reader.pos(), 9);
    }
}
