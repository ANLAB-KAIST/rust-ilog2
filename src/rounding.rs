
use bitops::*;
use ffs::*;
use fls::*;

pub fn rounddown_pow2<T: Bitops>(x: T) -> T {
    let msb_index = msb(x);
    let one = T::one();
    let bitlen = bit_length::<T>();
    assert!(msb_index < bitlen as i8);
    assert!(msb_index >= 0);
    one << (msb_index as usize)
}

pub fn roundup_pow2<T: Bitops>(x: T) -> T {
    let mut msb_index = msb(x);
    let lsb_index = lsb(x);
    let one = T::one();
    let bitlen = bit_length::<T>();
    assert!(msb_index >= 0 && lsb_index >= 0);
    if msb_index != lsb_index {
        msb_index += 1;
    }
    assert!(msb_index < bitlen as i8);
    assert!(msb_index >= 0);
    one << (msb_index as usize)
}
