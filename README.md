#Integer Logarithm Library for Rust

##API List

1. roundup_pow2(x), rounddown_pow2(x) for u8 ... u64, returns same integer type.
1. bit_length<u8 ... u64>(), returns usize
1. bit_mask<u8 ... u64>(), returns self-typed 0xFF...FF
1. msb(x), lsb(x), msz(x), lsz(x) for u8 ... u64, returns i8, -1i8 on error. Index starts from 0.
1. ffs(x), fls(x), ffz(x), flz(x) for u8 ... u64, returns u8, 0u8 means error. Index starts from 1.

* msb: Find first 1 bit from msb position. Return its index (index starts from lsb).
* lsb: Find first 1 bit from lsb position. Return its index (index starts from lsb).
* msz, lsz: Find 0 like above.
* ffs, fls, ffz, flz: +1 to lsb, msb, lsz, msz
