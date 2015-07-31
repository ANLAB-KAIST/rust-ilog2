extern crate ilog2;

fn main() {
    test_bitops();
    test_msb();
    test_lsb();
    test_msz();
    test_lsz();
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
    const U8_MIN : u8 = 1u8;
    const U16_MIN : u16 = 1u16;
    const U32_MIN : u32 = 1u32;
    const U64_MIN : u64 = 1u64;
    
    for x in 0..ilog2::bit_length::<u8>() as usize {
        let left = U8_MIN << x;
        let right = U8_MIN;
        assert_eq!(ilog2::msb(left | right), x as i8); 
    }
    assert_eq!(ilog2::msb(0u8), -1i8);
    
    for x in 0..ilog2::bit_length::<u16>() as usize {
        let left = U16_MIN << x;
        let right = U16_MIN;
        assert_eq!(ilog2::msb(left | right), x as i8); 
    }
    assert_eq!(ilog2::msb(0u16), -1i8);
    
    for x in 0..ilog2::bit_length::<u32>() as usize {
        let left = U32_MIN << x;
        let right = U32_MIN;
        assert_eq!(ilog2::msb(left | right), x as i8); 
    }
    assert_eq!(ilog2::msb(0u32), -1i8);
    
    for x in 0..ilog2::bit_length::<u64>() as usize {
        let left = U64_MIN << x;
        let right = U64_MIN;
        assert_eq!(ilog2::msb(left | right), x as i8); 
    }
    assert_eq!(ilog2::msb(0u64), -1i8);
}

fn test_lsb() {
    const U8_MIN : u8 = 1u8;
    const U16_MIN : u16 = 1u16;
    const U32_MIN : u32 = 1u32;
    const U64_MIN : u64 = 1u64;
    
    for x in 0..ilog2::bit_length::<u8>() as usize {
        let left = U8_MIN << (ilog2::bit_length::<u8>() - 1);
        let right = U8_MIN << x;
        assert_eq!(ilog2::lsb(left | right), x as i8); 
    }
    assert_eq!(ilog2::lsb(0u8), -1i8);
    
    for x in 0..ilog2::bit_length::<u16>() as usize {
        let left = U16_MIN << (ilog2::bit_length::<u16>() - 1);
        let right = U16_MIN << x;
        assert_eq!(ilog2::lsb(left | right), x as i8); 
    }
    assert_eq!(ilog2::lsb(0u16), -1i8);
    
    for x in 0..ilog2::bit_length::<u32>() as usize {
        let left = U32_MIN << (ilog2::bit_length::<u32>() - 1);
        let right = U32_MIN << x;
        assert_eq!(ilog2::lsb(left | right), x as i8); 
    }
    assert_eq!(ilog2::lsb(0u32), -1i8);
    
    for x in 0..ilog2::bit_length::<u64>() as usize {
        let left = U64_MIN << (ilog2::bit_length::<u64>() - 1);
        let right = U64_MIN << x;
        assert_eq!(ilog2::lsb(left | right), x as i8); 
    }
    assert_eq!(ilog2::lsb(0u64), -1i8);
}

fn test_msz() {
    const U8_MIN : u8 = 1u8;
    const U16_MIN : u16 = 1u16;
    const U32_MIN : u32 = 1u32;
    const U64_MIN : u64 = 1u64;
    
    const U8_MAX : u8 = 0xFFu8;
    const U16_MAX : u16 = 0xFFFFu16;
    const U32_MAX : u32 = 0xFFFFFFFFu32;
    const U64_MAX : u64 = 0xFFFFFFFFFFFFFFFFu64;
    
    for x in 0..ilog2::bit_length::<u8>() as usize {
        let left = U8_MIN << x;
        let right = U8_MIN;
        assert_eq!(ilog2::msz(!(left | right)), x as i8); 
    }
    assert_eq!(ilog2::msz(U8_MAX), -1i8);
    
    for x in 0..ilog2::bit_length::<u16>() as usize {
        let left = U16_MIN << x;
        let right = U16_MIN;
        assert_eq!(ilog2::msz(!(left | right)), x as i8); 
    }
    assert_eq!(ilog2::msz(U16_MAX), -1i8);
    
    for x in 0..ilog2::bit_length::<u32>() as usize {
        let left = U32_MIN << x;
        let right = U32_MIN;
        assert_eq!(ilog2::msz(!(left | right)), x as i8); 
    }
    assert_eq!(ilog2::msz(U32_MAX), -1i8);
    
    for x in 0..ilog2::bit_length::<u64>() as usize {
        let left = U64_MIN << x;
        let right = U64_MIN;
        assert_eq!(ilog2::msz(!(left | right)), x as i8); 
    }
    assert_eq!(ilog2::msz(U64_MAX), -1i8);
}

fn test_lsz() {
    const U8_MIN : u8 = 1u8;
    const U16_MIN : u16 = 1u16;
    const U32_MIN : u32 = 1u32;
    const U64_MIN : u64 = 1u64;
    
    const U8_MAX : u8 = 0xFFu8;
    const U16_MAX : u16 = 0xFFFFu16;
    const U32_MAX : u32 = 0xFFFFFFFFu32;
    const U64_MAX : u64 = 0xFFFFFFFFFFFFFFFFu64;
    
    for x in 0..ilog2::bit_length::<u8>() as usize {
        let left = U8_MIN << (ilog2::bit_length::<u8>() - 1);
        let right = U8_MIN << x;
        assert_eq!(ilog2::lsz(!(left | right)), x as i8); 
    }
    assert_eq!(ilog2::lsz(U8_MAX), -1i8);
    
    for x in 0..ilog2::bit_length::<u16>() as usize {
        let left = U16_MIN << (ilog2::bit_length::<u16>() - 1);
        let right = U16_MIN << x;
        assert_eq!(ilog2::lsz(!(left | right)), x as i8); 
    }
    assert_eq!(ilog2::lsz(U16_MAX), -1i8);
    
    for x in 0..ilog2::bit_length::<u32>() as usize {
        let left = U32_MIN << (ilog2::bit_length::<u32>() - 1);
        let right = U32_MIN << x;
        assert_eq!(ilog2::lsz(!(left | right)), x as i8); 
    }
    assert_eq!(ilog2::lsz(U32_MAX), -1i8);
    
    for x in 0..ilog2::bit_length::<u64>() as usize {
        let left = U64_MIN << (ilog2::bit_length::<u64>() - 1);
        let right = U64_MIN << x;
        assert_eq!(ilog2::lsz(!(left | right)), x as i8); 
    }
    assert_eq!(ilog2::lsz(U64_MAX), -1i8);
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