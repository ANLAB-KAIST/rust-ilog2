extern crate num;
use extension::*;

pub trait Bitops: num::PrimInt + num::Unsigned + PrimIntExt {
    fn bit_length() -> usize;
    fn bit_mask() -> Self;
}

impl Bitops for u8 {
    fn bit_length() -> usize { 8usize }
    fn bit_mask() -> u8 { 0xFFu8 }
}

impl Bitops for u16 {	
    fn bit_length() -> usize { 16usize }
    fn bit_mask() -> u16 { 0xFFFFu16 }
}

impl Bitops for u32 {
    fn bit_length() -> usize { 32usize }
    fn bit_mask() -> u32 { 0xFFFFFFFFu32 }
}

impl Bitops for u64 {
    fn bit_length() -> usize { 64usize }
    fn bit_mask() -> u64 { 0xFFFFFFFFFFFFFFFFu64 }
}

pub fn bit_length<T: Bitops>() -> usize {
    T::bit_length()
}

pub fn bit_mask<T: Bitops>() -> T {
    T::bit_mask()
}
 