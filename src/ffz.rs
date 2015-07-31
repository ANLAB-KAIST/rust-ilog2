use bitops::*;

pub fn ffz<T: Bitops>(x: T) -> i8 {
    lsz(x) + 1
}

pub fn lsz<T: Bitops>(x: T) -> i8 {
    if x != bit_mask::<T>() {
        (x.trailing_ones() as i8)
    } else {
        -1i8
    }
}