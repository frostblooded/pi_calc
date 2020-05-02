pub type BigNum = rug::Float;

pub fn new_num(precision: u32, n: u64) -> BigNum {
    BigNum::with_val(precision, n)
}
