use bitops::*;

pub fn lsb<T: Bitops>(x: T) -> i8 {
    let mut found = false;
    let mut least_index = T::bit_length();
    let bitlen = T::bit_length();

    for index in 0..bitlen {
        let temp = x << (bitlen - 1 -index);
        if temp != T::zero() {
            found = true;
            if least_index > index {
                least_index = index;
            }
        }
    }

    if found {
        least_index as i8
    } else {
        -1i8
    }
}
