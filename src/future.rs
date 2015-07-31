extern crate num;

pub trait Future {
    fn leading_ones(self) -> u32;
    fn trailing_ones(self) -> u32;
}

impl<T: num::PrimInt + num::Unsigned> Future for T {
    fn leading_ones(self) -> u32 {
        let inverse = !self;
        inverse.leading_zeros()
    }
    fn trailing_ones(self) -> u32 {
        let inverse = !self;
        inverse.trailing_zeros()
    }
}