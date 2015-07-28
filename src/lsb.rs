use bitops::*;

pub trait LSB {
    fn lsb(x: Self) -> i8;
}

pub fn lsb<T: LSB>(x: T) -> i8 {
    T::lsb(x)
}

macro_rules! lsb_impl {
    ( $t:ty) => {
        impl LSB for $t { fn lsb(x: $t) -> i8 {
	    	let mut found = false;
			let mut least_index = bit_length::<$t>();
			let bitlen = bit_length::<$t>();
			
			for index in 0..bitlen {
				let temp = x << (bitlen - 1 -index);
				if temp != 0 {
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
	    }}
    };
}

lsb_impl!(u8);
lsb_impl!(u16);
lsb_impl!(u32);
lsb_impl!(u64);
