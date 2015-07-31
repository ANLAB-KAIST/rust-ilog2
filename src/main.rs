extern crate ilog2;

fn main() {
    test_bitops();
    test_msb();
    test_fls();
    test_lsb();
    test_ffs();
    test_rounddown();
    test_roundup();
    println!("Test Successful!");
}

fn test_bitops() {
    assert_eq!(ilog2::bit_length::<u8>(), 8usize);
    assert_eq!(ilog2::bit_length::<u16>(), 16usize);
    assert_eq!(ilog2::bit_length::<u32>(), 32usize);
    assert_eq!(ilog2::bit_length::<u64>(), 64usize);
    
    const U8_CONST1 : u8 = 0b10101010;
    const U16_CONST1 : u16 = 0b1010101010101010;
    const U32_CONST1 : u32 = 0b10101010101010101010101010101010;
    const U64_CONST1 : u64 = 0b1010101010101010101010101010101010101010101010101010101010101010;
    
    const U8_CONST2 : u8 = 0b01010101;
    const U16_CONST2 : u16 = 0b0101010101010101;
    const U32_CONST2 : u32 = 0b01010101010101010101010101010101;
    const U64_CONST2 : u64 = 0b0101010101010101010101010101010101010101010101010101010101010101;
    
    assert_eq!(ilog2::bit_mask::<u8>() & U8_CONST1, U8_CONST1);
    assert_eq!(ilog2::bit_mask::<u16>() & U16_CONST1, U16_CONST1);
    assert_eq!(ilog2::bit_mask::<u32>() & U32_CONST1, U32_CONST1);
    assert_eq!(ilog2::bit_mask::<u64>() & U64_CONST1, U64_CONST1);
    
    assert_eq!(ilog2::bit_mask::<u8>() & U8_CONST2, U8_CONST2);
    assert_eq!(ilog2::bit_mask::<u16>() & U16_CONST2, U16_CONST2);
    assert_eq!(ilog2::bit_mask::<u32>() & U32_CONST2, U32_CONST2);
    assert_eq!(ilog2::bit_mask::<u64>() & U64_CONST2, U64_CONST2);
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

fn test_fls() {
    assert_eq!(ilog2::fls(128u8), 8i8);
    assert_eq!(ilog2::fls(1u8), 1i8);
    assert_eq!(ilog2::fls(0u8), 0i8);
    assert_eq!(ilog2::fls(128u8 + 1u8), 8i8);
    
    assert_eq!(ilog2::fls(32768u16), 16i8);
    assert_eq!(ilog2::fls(128u16), 8i8);
    assert_eq!(ilog2::fls(1u16), 1i8);
    assert_eq!(ilog2::fls(0u16), 0i8);
    assert_eq!(ilog2::fls(128u16 + 1u16), 8i8);
    assert_eq!(ilog2::fls(32768u16 + 1u16), 16i8);
    
    assert_eq!(ilog2::fls(2147483648u32), 32i8);
    assert_eq!(ilog2::fls(128u32), 8i8);
    assert_eq!(ilog2::fls(1u32), 1i8);
    assert_eq!(ilog2::fls(0u32), 0i8);
    assert_eq!(ilog2::fls(128u32 + 1u32), 8i8);
    assert_eq!(ilog2::fls(32768u32 + 1u32), 16i8);
    assert_eq!(ilog2::fls(2147483648u32 + 1u32), 32i8);
    
    assert_eq!(ilog2::fls(9223372036854775808u64), 64i8);
    assert_eq!(ilog2::fls(128u64), 8i8);
    assert_eq!(ilog2::fls(1u64), 1i8);
    assert_eq!(ilog2::fls(0u64), 0i8);
    assert_eq!(ilog2::fls(128u64 + 1u64), 8i8);
    assert_eq!(ilog2::fls(32768u64 + 1u64), 16i8);
    assert_eq!(ilog2::fls(2147483648u64 + 1u64), 32i8);
    assert_eq!(ilog2::fls(9223372036854775808u64 + 1u64), 64i8);
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

fn test_ffs() {
    assert_eq!(ilog2::ffs(128u8), 8i8);
    assert_eq!(ilog2::ffs(1u8), 1i8);
    assert_eq!(ilog2::ffs(0u8), 0i8);
    assert_eq!(ilog2::ffs(128u8 + 1u8), 1i8);
    
    assert_eq!(ilog2::ffs(32768u16), 16i8);
    assert_eq!(ilog2::ffs(128u16), 8i8);
    assert_eq!(ilog2::ffs(1u16), 1i8);
    assert_eq!(ilog2::ffs(0u16), 0i8);
    assert_eq!(ilog2::ffs(128u16 + 1u16), 1i8);
    assert_eq!(ilog2::ffs(32768u16 + 1u16), 1i8);
    
    assert_eq!(ilog2::ffs(2147483648u32), 32i8);
    assert_eq!(ilog2::ffs(128u32), 8i8);
    assert_eq!(ilog2::ffs(1u32), 1i8);
    assert_eq!(ilog2::ffs(0u32), 0i8);
    assert_eq!(ilog2::ffs(128u32 + 1u32), 1i8);
    assert_eq!(ilog2::ffs(32768u32 + 1u32), 1i8);
    assert_eq!(ilog2::ffs(2147483648u32 + 1u32), 1i8);
    
    assert_eq!(ilog2::ffs(9223372036854775808u64), 64i8);
    assert_eq!(ilog2::ffs(128u64), 8i8);
    assert_eq!(ilog2::ffs(1u64), 1i8);
    assert_eq!(ilog2::ffs(0u64), 0i8);
    assert_eq!(ilog2::ffs(128u64 + 1u64), 1i8);
    assert_eq!(ilog2::ffs(32768u64 + 1u64), 1i8);
    assert_eq!(ilog2::ffs(2147483648u64 + 1u64), 1i8);
    assert_eq!(ilog2::ffs(9223372036854775808u64 + 1u64), 1i8);
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