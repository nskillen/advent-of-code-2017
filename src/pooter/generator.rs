use std::iter::{Iterator};

pub struct Generator {
    val: u64,
    factor: u64,
    modulo: u64,
}

impl Generator {
    pub fn new(seed: u64, factor: u64) -> Self {
        Self {
            val: seed,
            factor: factor,
            modulo: 2147483647
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.val = self.val * self.factor % self.modulo;
        Some(self.val)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::max_value(), None)
    }
}

