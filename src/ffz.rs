use bitops::*;

pub fn ffz<T: Bitops>(x: T) -> u8 {
    (lsz(x) + 1) as u8
}

pub fn lsz<T: Bitops>(x: T) -> i8 {
    if x != bit_mask::<T>() {
        (x.trailing_ones() as i8)
    } else {
        -1i8
    }
}