extern crate ilog2;

fn main() {
	test_msb();
	test_lsb();
	println!("Test Successful!");
}

fn test_msb() {
	assert_eq!(ilog2::msb_u8(128u8), 7i8);
	assert_eq!(ilog2::msb_u8(1u8), 0i8);
	assert_eq!(ilog2::msb_u8(0u8), -1i8);
	assert_eq!(ilog2::msb_u8(128u8 + 1u8), 7i8);
	
	assert_eq!(ilog2::msb_u16(32768u16), 15i8);
	assert_eq!(ilog2::msb_u16(128u16), 7i8);
	assert_eq!(ilog2::msb_u16(1u16), 0i8);
	assert_eq!(ilog2::msb_u16(0u16), -1i8);
	assert_eq!(ilog2::msb_u16(128u16 + 1u16), 7i8);
	assert_eq!(ilog2::msb_u16(32768u16 + 1u16), 15i8);
	
	assert_eq!(ilog2::msb_u32(2147483648u32), 31i8);
	assert_eq!(ilog2::msb_u32(128u32), 7i8);
	assert_eq!(ilog2::msb_u32(1u32), 0i8);
	assert_eq!(ilog2::msb_u32(0u32), -1i8);
	assert_eq!(ilog2::msb_u32(128u32 + 1u32), 7i8);
	assert_eq!(ilog2::msb_u32(32768u32 + 1u32), 15i8);
	assert_eq!(ilog2::msb_u32(2147483648u32 + 1u32), 31i8);
	
	assert_eq!(ilog2::msb_u64(9223372036854775808u64), 63i8);
	assert_eq!(ilog2::msb_u64(128u64), 7i8);
	assert_eq!(ilog2::msb_u64(1u64), 0i8);
	assert_eq!(ilog2::msb_u64(0u64), -1i8);
	assert_eq!(ilog2::msb_u64(128u64 + 1u64), 7i8);
	assert_eq!(ilog2::msb_u64(32768u64 + 1u64), 15i8);
	assert_eq!(ilog2::msb_u64(2147483648u64 + 1u64), 31i8);
	assert_eq!(ilog2::msb_u64(9223372036854775808u64 + 1u64), 63i8);
}

fn test_lsb() {
	assert_eq!(ilog2::lsb_u8(128u8), 7i8);
	assert_eq!(ilog2::lsb_u8(1u8), 0i8);
	assert_eq!(ilog2::lsb_u8(0u8), -1i8);
	assert_eq!(ilog2::lsb_u8(128u8 + 1u8), 0i8);
	
	assert_eq!(ilog2::lsb_u16(32768u16), 15i8);
	assert_eq!(ilog2::lsb_u16(128u16), 7i8);
	assert_eq!(ilog2::lsb_u16(1u16), 0i8);
	assert_eq!(ilog2::lsb_u16(0u16), -1i8);
	assert_eq!(ilog2::lsb_u16(128u16 + 1u16), 0i8);
	assert_eq!(ilog2::lsb_u16(32768u16 + 1u16), 0i8);
	
	assert_eq!(ilog2::lsb_u32(2147483648u32), 31i8);
	assert_eq!(ilog2::lsb_u32(128u32), 7i8);
	assert_eq!(ilog2::lsb_u32(1u32), 0i8);
	assert_eq!(ilog2::lsb_u32(0u32), -1i8);
	assert_eq!(ilog2::lsb_u32(128u32 + 1u32), 0i8);
	assert_eq!(ilog2::lsb_u32(32768u32 + 1u32), 0i8);
	assert_eq!(ilog2::lsb_u32(2147483648u32 + 1u32), 0i8);
	
	assert_eq!(ilog2::lsb_u64(9223372036854775808u64), 63i8);
	assert_eq!(ilog2::lsb_u64(128u64), 7i8);
	assert_eq!(ilog2::lsb_u64(1u64), 0i8);
	assert_eq!(ilog2::lsb_u64(0u64), -1i8);
	assert_eq!(ilog2::lsb_u64(128u64 + 1u64), 0i8);
	assert_eq!(ilog2::lsb_u64(32768u64 + 1u64), 0i8);
	assert_eq!(ilog2::lsb_u64(2147483648u64 + 1u64), 0i8);
	assert_eq!(ilog2::lsb_u64(9223372036854775808u64 + 1u64), 0i8);
}