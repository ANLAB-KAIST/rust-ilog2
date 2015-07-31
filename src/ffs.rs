use bitops::*;

pub fn ffs<T: Bitops>(x: T) -> u8 {
    (lsb(x) + 1) as u8
}

pub fn lsb<T: Bitops>(x: T) -> i8 {
    if x != T::zero() {
        (x.trailing_zeros() as i8)
    } else {
        -1i8
    }
}