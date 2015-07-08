extern crate ilog2;

fn main() {
	println!("val: {}", ilog2::msb_u8(128u8));
	println!("val: {}", ilog2::lsb_u8(128u8 + 4u8));
}