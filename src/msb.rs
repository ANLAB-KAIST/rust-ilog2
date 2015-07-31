use bitops::*;

pub fn msb<T: Bitops>(x: T) -> i8 {
    let mut found = false;
    let mut max_index = 0;
    let bitlen = T::bit_length();

    for index in 0..bitlen{
        let temp = x >> index;
        if temp != T::zero() {
            found = true;
            if max_index < index {
                max_index = index;
            }
        }
    }
    
    if found {
        max_index as i8
    } else {
        -1i8
    }
}
