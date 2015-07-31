use bitops::*;

pub fn ffs<T: Bitops>(x: T) -> i8 {
    lsb(x) + 1
}

pub fn lsb<T: Bitops>(x: T) -> i8 {
    let trailing_zeros = x.trailing_zeros() as i8;
    if x != T::zero() {
        (trailing_zeros)
    } else {
        -1i8
    }
}