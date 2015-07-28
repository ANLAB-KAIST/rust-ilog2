
use bitops::*;
use msb::*;
use lsb::*;

pub trait Rounding {
    fn rounddown_pow2(x: Self) -> Self;
    fn roundup_pow2(x: Self) -> Self;
}

pub fn rounddown_pow2<T: Rounding>(x: T) -> T {
    T::rounddown_pow2(x)
}

pub fn roundup_pow2<T: Rounding>(x: T) -> T {
    T::roundup_pow2(x)
}

macro_rules! rounding_impl {
    ( $t:ty) => {
        impl Rounding for $t {
        	fn rounddown_pow2(x: $t) -> $t {
		    	let msb_index = msb(x);
				let one: $t = 1;
				assert!(msb_index >= 0);
				one << msb_index
		    }
        	fn roundup_pow2(x: $t) -> $t {
		    	let mut msb_index = msb(x);
				let lsb_index = lsb(x);
				let one: $t = 1;
				let bitlen = bit_length::<$t>();
				assert!(msb_index >= 0 && lsb_index >= 0);
				if msb_index != lsb_index {
					msb_index += 1;
				}
				assert!(msb_index < bitlen as i8);
				one << msb_index
		    }
       	}
    };
}

rounding_impl!(u8);
rounding_impl!(u16);
rounding_impl!(u32);
rounding_impl!(u64);