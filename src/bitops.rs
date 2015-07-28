//Thanks to sanxiyn at irc.orinzer.org

pub trait Bitops {
    fn bit_length() -> usize;
}

impl Bitops for u8 {
    fn bit_length() -> usize { 8 }
}

impl Bitops for u16 {	
    fn bit_length() -> usize { 16 }
}

impl Bitops for u32 {
    fn bit_length() -> usize { 32 }
}

impl Bitops for u64 {
    fn bit_length() -> usize { 64 }
}

pub fn bit_length<T: Bitops>() -> usize {
    T::bit_length()
}
 