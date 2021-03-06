use crate::big_num::*;

pub struct FactorialCalculator {
    cache: Vec<BigNum>,
}

impl FactorialCalculator {
    pub fn new(precision: u32, n: u64) -> Self {
        let mut cache_builder: Vec<BigNum> = Vec::with_capacity(n as usize);
        cache_builder.push(new_num(precision, 1));

        for i in 1..=n {
            cache_builder.push(&cache_builder[(i - 1) as usize] * new_num(precision, i));
        }

        FactorialCalculator {
            cache: cache_builder,
        }
    }

    #[inline]
    pub fn get(&self, i: u64) -> &BigNum {
        &self.cache[i as usize]
    }
}
