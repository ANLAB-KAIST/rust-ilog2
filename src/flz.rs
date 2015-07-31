use bitops::*;

pub fn flz<T: Bitops>(x: T) -> u8 {
    (msz(x) + 1) as u8
}

pub fn msz<T: Bitops>(x: T) -> i8 {
    let bits = bit_length::<T>() as i8;
    if x != bit_mask::<T>() {
        (bits -1i8 - (x.leading_ones() as i8))
    } else {
        -1i8
    }
}