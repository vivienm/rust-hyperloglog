use std::iter::FusedIterator;

use bitvec::{field::BitField, order::Lsb0, slice::Chunks, vec::BitVec};

use crate::precision::Precision;

const CHUNK_SIZE: usize = 6;

#[inline]
fn bit_index(index: usize) -> usize {
    index * CHUNK_SIZE
}

#[derive(Clone, Debug)]
pub struct Registers {
    precision: Precision,
    len: usize,
    bits: BitVec,
}

impl Registers {
    pub fn with_precision(precision: Precision) -> Self {
        let len = 1 << precision.get();
        Self {
            precision,
            len,
            bits: BitVec::repeat(false, bit_index(len)),
        }
    }
}

impl<'a> FusedIterator for Iter<'a> {}

impl Registers {
    #[inline]
    pub fn precision(&self) -> Precision {
        self.precision
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn update(&mut self, index: usize, value: u8) {
        let bit_slice = self
            .bits
            .get_mut(bit_index(index)..bit_index(index + 1))
            .expect("invalid index");
        if bit_slice.load::<u8>() < value {
            debug_assert!(value < 1 << CHUNK_SIZE);
            bit_slice.store(value);
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            chunks: self.bits.chunks(CHUNK_SIZE),
        }
    }

    pub fn clear(&mut self) {
        self.bits.set_elements(0)
    }

    pub fn is_empty(&self) -> bool {
        !self.bits.any()
    }

    pub fn merge_from_unchecked(&mut self, rhs: &Self) {
        for (self_chunk, rhs_chunk) in self
            .bits
            .chunks_mut(CHUNK_SIZE)
            .zip(rhs.bits.chunks(CHUNK_SIZE))
        {
            let self_value: u8 = self_chunk.load();
            let rhs_value: u8 = rhs_chunk.load();
            if self_value < rhs_value {
                self_chunk.store(rhs_value);
            }
        }
    }

    pub fn merge_unchecked(&self, rhs: &Self) -> Self {
        let mut new = self.clone();
        new.merge_from_unchecked(rhs);
        new
    }
}

#[derive(Debug)]
pub struct Iter<'a> {
    chunks: Chunks<'a, usize, Lsb0>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.chunks.next().map(|chunk| chunk.load())
    }
}
