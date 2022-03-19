use std::cmp::max;

use crate::precision::Precision;

#[derive(Clone, Debug)]
pub struct Registers {
    precision: Precision,
    values: Vec<u8>,
}

impl Registers {
    pub fn with_precision(precision: Precision) -> Self {
        let num_registers = 1 << precision.get();
        Self {
            precision,
            values: vec![0; num_registers],
        }
    }
}

impl Registers {
    #[inline]
    pub fn precision(&self) -> Precision {
        self.precision
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn update(&mut self, index: usize, value: u8) {
        let current_p = self.values.get_mut(index).unwrap();
        if *current_p < value {
            *current_p = value;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.values.iter().copied()
    }

    pub fn clear(&mut self) {
        for value in &mut self.values {
            *value = 0;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values.iter().all(|value| *value == 0)
    }

    pub fn merge_from_unchecked(&mut self, rhs: &Self) {
        for (self_value_p, rhs_value_p) in self.values.iter_mut().zip(rhs.values.iter()) {
            *self_value_p = max(*self_value_p, *rhs_value_p);
        }
    }

    pub fn merge_unchecked(&self, rhs: &Self) -> Self {
        Self {
            precision: self.precision,
            values: self
                .values
                .iter()
                .zip(rhs.values.iter())
                .map(|(self_value_p, other_value_p)| max(*self_value_p, *other_value_p))
                .collect(),
        }
    }
}
