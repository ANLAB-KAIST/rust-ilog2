use bitops::*;

pub fn fls<T: Bitops>(x: T) -> i8 {
    msb(x) + 1
}

pub fn msb<T: Bitops>(x: T) -> i8 {
    let bits = bit_length::<T>() as i8;
    let leading_zeros = x.leading_zeros() as i8;
    if x != T::zero() {
        (bits -1i8 - leading_zeros)
    } else {
        -1i8
    }
}