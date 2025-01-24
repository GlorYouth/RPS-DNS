#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::*;
use generic_array::typenum::U2;
use generic_array::{ArrayLength, GenericArray};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct ArrayU8<N: ArrayLength> {
    data: GenericArray<u8, N>,
}

impl<N: ArrayLength> ArrayU8<N> {
    const SIZE: usize = N::USIZE;

    pub fn new() -> ArrayU8<N> {
        ArrayU8 {
            data: GenericArray::default(),
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.data.to_vec()
    }

    pub fn from_bytes(b: &[u8]) -> ArrayU8<N> {
        let mut arr = GenericArray::default();
        arr.copy_from_slice(b);
        ArrayU8 { data: arr }
    }

    pub fn from_reader(reader: &mut SliceReader) -> ArrayU8<N> {
        let mut arr = GenericArray::default();
        for i in 0..N::USIZE {
            arr[i] = reader.read_u8();
        }
        ArrayU8 { data: arr }
    }
}

impl ArrayU8<U2> {
    pub fn from_u16(u: u16) -> ArrayU8<U2> {
        ArrayU8 {
            data: GenericArray::from(u.to_be_bytes()),
        }
    }
}

impl Index<u8> for ArrayU8<U2> {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        self.data.index(index as usize)
    }
}

impl IndexMut<u8> for ArrayU8<U2> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        self.data.index_mut(index as usize)
    }
}

impl<N: ArrayLength> From<GenericArray<u8, N>> for ArrayU8<N> {
    fn from(value: GenericArray<u8, N>) -> Self {
        ArrayU8 {
            data: GenericArray::from(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_array_u8() {
        let mut arr = ArrayU8::<U2>::new();
        assert_eq!(arr.to_vec(), vec![0, 0]);
        arr[0] = 1;
        let arr: ArrayU8<U2> = ArrayU8::from_bytes(arr.as_slice());
        assert_eq!(arr.to_vec(), vec![1, 0]);
        let arr: ArrayU8<U2> = ArrayU8::from_reader(&mut SliceReader::from(&[0, 1][..]));
        assert_eq!(arr.to_vec(), vec![0, 1]);
        let arr: ArrayU8<U2> = ArrayU8::from_u16(0x10_01);
        assert_eq!(arr.to_vec(), vec![0x10, 0x01]);
    }
}
