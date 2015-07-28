extern crate ilog2;

fn main() {
	test_msb();
	test_lsb();
	test_rounddown();
	test_roundup();
	println!("Test Successful!");
}

fn test_msb() {
	assert_eq!(ilog2::msb(128u8), 7i8);
	assert_eq!(ilog2::msb(1u8), 0i8);
	assert_eq!(ilog2::msb(0u8), -1i8);
	assert_eq!(ilog2::msb(128u8 + 1u8), 7i8);
	
	assert_eq!(ilog2::msb(32768u16), 15i8);
	assert_eq!(ilog2::msb(128u16), 7i8);
	assert_eq!(ilog2::msb(1u16), 0i8);
	assert_eq!(ilog2::msb(0u16), -1i8);
	assert_eq!(ilog2::msb(128u16 + 1u16), 7i8);
	assert_eq!(ilog2::msb(32768u16 + 1u16), 15i8);
	
	assert_eq!(ilog2::msb(2147483648u32), 31i8);
	assert_eq!(ilog2::msb(128u32), 7i8);
	assert_eq!(ilog2::msb(1u32), 0i8);
	assert_eq!(ilog2::msb(0u32), -1i8);
	assert_eq!(ilog2::msb(128u32 + 1u32), 7i8);
	assert_eq!(ilog2::msb(32768u32 + 1u32), 15i8);
	assert_eq!(ilog2::msb(2147483648u32 + 1u32), 31i8);
	
	assert_eq!(ilog2::msb(9223372036854775808u64), 63i8);
	assert_eq!(ilog2::msb(128u64), 7i8);
	assert_eq!(ilog2::msb(1u64), 0i8);
	assert_eq!(ilog2::msb(0u64), -1i8);
	assert_eq!(ilog2::msb(128u64 + 1u64), 7i8);
	assert_eq!(ilog2::msb(32768u64 + 1u64), 15i8);
	assert_eq!(ilog2::msb(2147483648u64 + 1u64), 31i8);
	assert_eq!(ilog2::msb(9223372036854775808u64 + 1u64), 63i8);
}

fn test_lsb() {
	assert_eq!(ilog2::lsb(128u8), 7i8);
	assert_eq!(ilog2::lsb(1u8), 0i8);
	assert_eq!(ilog2::lsb(0u8), -1i8);
	assert_eq!(ilog2::lsb(128u8 + 1u8), 0i8);
	
	assert_eq!(ilog2::lsb(32768u16), 15i8);
	assert_eq!(ilog2::lsb(128u16), 7i8);
	assert_eq!(ilog2::lsb(1u16), 0i8);
	assert_eq!(ilog2::lsb(0u16), -1i8);
	assert_eq!(ilog2::lsb(128u16 + 1u16), 0i8);
	assert_eq!(ilog2::lsb(32768u16 + 1u16), 0i8);
	
	assert_eq!(ilog2::lsb(2147483648u32), 31i8);
	assert_eq!(ilog2::lsb(128u32), 7i8);
	assert_eq!(ilog2::lsb(1u32), 0i8);
	assert_eq!(ilog2::lsb(0u32), -1i8);
	assert_eq!(ilog2::lsb(128u32 + 1u32), 0i8);
	assert_eq!(ilog2::lsb(32768u32 + 1u32), 0i8);
	assert_eq!(ilog2::lsb(2147483648u32 + 1u32), 0i8);
	
	assert_eq!(ilog2::lsb(9223372036854775808u64), 63i8);
	assert_eq!(ilog2::lsb(128u64), 7i8);
	assert_eq!(ilog2::lsb(1u64), 0i8);
	assert_eq!(ilog2::lsb(0u64), -1i8);
	assert_eq!(ilog2::lsb(128u64 + 1u64), 0i8);
	assert_eq!(ilog2::lsb(32768u64 + 1u64), 0i8);
	assert_eq!(ilog2::lsb(2147483648u64 + 1u64), 0i8);
	assert_eq!(ilog2::lsb(9223372036854775808u64 + 1u64), 0i8);
}

fn test_rounddown() {
	const U8_MIN : u8 = 1u8;
	const U16_MIN : u16 = 1u16;
	const U32_MIN : u32 = 1u32;
	const U64_MIN : u64 = 1u64;
	
	const U8_MID : u8 = 0x80u8;
	const U16_MID : u16 = 0x8000u16;
	const U32_MID : u32 = 0x80000000u32;
	const U64_MID : u64 = 0x8000000000000000u64;
	
	const U8_MAX : u8 = 0xFFu8;
	const U16_MAX : u16 = 0xFFFFu16;
	const U32_MAX : u32 = 0xFFFFFFFFu32;
	const U64_MAX : u64 = 0xFFFFFFFFFFFFFFFFu64;
	
	assert_eq!(ilog2::rounddown_pow2(U8_MIN), 1);
	assert_eq!(ilog2::rounddown_pow2(U16_MIN), 1);
	assert_eq!(ilog2::rounddown_pow2(U32_MIN), 1);
	assert_eq!(ilog2::rounddown_pow2(U64_MIN), 1);
	
	assert_eq!(ilog2::rounddown_pow2(U8_MID + U8_MIN), U8_MID);
	assert_eq!(ilog2::rounddown_pow2(U16_MID + U16_MIN), U16_MID);
	assert_eq!(ilog2::rounddown_pow2(U32_MID + U32_MIN), U32_MID);
	assert_eq!(ilog2::rounddown_pow2(U64_MID + U64_MIN), U64_MID);
	
	assert_eq!(ilog2::rounddown_pow2(U8_MAX), U8_MID);
	assert_eq!(ilog2::rounddown_pow2(U16_MAX), U16_MID);
	assert_eq!(ilog2::rounddown_pow2(U32_MAX), U32_MID);
	assert_eq!(ilog2::rounddown_pow2(U64_MAX), U64_MID);
}

fn test_roundup() {
	const U8_MIN : u8 = 1u8;
	const U16_MIN : u16 = 1u16;
	const U32_MIN : u32 = 1u32;
	const U64_MIN : u64 = 1u64;
	
	const U8_MID : u8 = 0x80u8;
	const U16_MID : u16 = 0x8000u16;
	const U32_MID : u32 = 0x80000000u32;
	const U64_MID : u64 = 0x8000000000000000u64;
	
	const U8_MAX : u8 = 0xFFu8;
	const U16_MAX : u16 = 0xFFFFu16;
	const U32_MAX : u32 = 0xFFFFFFFFu32;
	const U64_MAX : u64 = 0xFFFFFFFFFFFFFFFFu64;
	
	assert_eq!(ilog2::roundup_pow2(U8_MIN), 1);
	assert_eq!(ilog2::roundup_pow2(U16_MIN), 1);
	assert_eq!(ilog2::roundup_pow2(U32_MIN), 1);
	assert_eq!(ilog2::roundup_pow2(U64_MIN), 1);
	
	assert_eq!(ilog2::roundup_pow2((U8_MID >> 1) + U8_MIN), U8_MID);
	assert_eq!(ilog2::roundup_pow2((U16_MID >> 1) + U16_MIN), U16_MID);
	assert_eq!(ilog2::roundup_pow2((U32_MID >> 1) + U32_MIN), U32_MID);
	assert_eq!(ilog2::roundup_pow2((U64_MID >> 1) + U64_MIN), U64_MID);
	
	assert_eq!(ilog2::roundup_pow2(U8_MAX >> 1), U8_MID);
	assert_eq!(ilog2::roundup_pow2(U16_MAX >> 1), U16_MID);
	assert_eq!(ilog2::roundup_pow2(U32_MAX >> 1), U32_MID);
	assert_eq!(ilog2::roundup_pow2(U64_MAX >> 1), U64_MID);
}