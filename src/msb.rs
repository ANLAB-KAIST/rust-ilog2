use bitops::*;

pub trait MSB {
    fn msb(x: Self) -> i8;
}

pub fn msb<T: MSB>(x: T) -> i8 {
    T::msb(x)
}

macro_rules! msb_impl {
    ( $t:ty) => {
        impl MSB for $t { fn msb(x: $t) -> i8 {
	    	let mut found = false;
			let mut max_index = 0;
			let bitlen = bit_length::<$t>();
			
			for index in 0..bitlen{
				let temp = x >> index;
				if temp != 0 {
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
	    }}
    };
}

msb_impl!(u8);
msb_impl!(u16);
msb_impl!(u32);
msb_impl!(u64);
