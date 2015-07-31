#Integer Logarithm Library for Rust

##API List

1. roundup_pow2(x) for u8 ... u64, returns same integer type.
2. rounddown_pow2(x) for u8 ... u64, returns same integer type.
3. bit_length<u8 ... u64>(), returns usize
4. bit_mask<u8 ... u64>(), returns self-typed 0xFF...FF
5. lsb(x) for u8 ... u64, returns i8, -1i8 on error.
6. msb(x) for u8 ... u64, returns i8, -1i8 on error.
* msb: Find first 1 bit from msb position. Return its index (index starts from lsb).
* lsb: Find first 1 bit from lsb position. Return its index (index starts from lsb).
