use msb::*;
use lsb::*;

pub fn rounddown_pow2_u8(x: u8) -> u8
{
	let msb_index = msb_u8(x);
	assert!(msb_index >= 0);
	1u8 << msb_index
}

pub fn rounddown_pow2_u16(x: u16) -> u16
{
	let msb_index = msb_u16(x);
	assert!(msb_index >= 0);
	1u16 << msb_index
}

pub fn rounddown_pow2_u32(x: u32) -> u32
{
	let msb_index = msb_u32(x);
	assert!(msb_index >= 0);
	1u32 << msb_index
}

pub fn rounddown_pow2_u64(x: u64) -> u64
{
	let msb_index = msb_u64(x);
	assert!(msb_index >= 0);
	1u64 << msb_index
}

///---

pub fn roundup_pow2_u8(x: u8) -> u8
{
	let mut msb_index = msb_u8(x);
	let lsb_index = lsb_u8(x);
	assert!(msb_index >= 0 && lsb_index >= 0);
	if msb_index != lsb_index {
		msb_index += 1;
	}
	assert!(msb_index < 8);
	1u8 << msb_index
}

pub fn roundup_pow2_u16(x: u16) -> u16
{
	let mut msb_index = msb_u16(x);
	let lsb_index = lsb_u16(x);
	assert!(msb_index >= 0 && lsb_index >= 0);
	if msb_index != lsb_index {
		msb_index += 1;
	}
	assert!(msb_index < 16);
	1u16 << msb_index
}

pub fn roundup_pow2_u32(x: u32) -> u32
{
	let mut msb_index = msb_u32(x);
	let lsb_index = lsb_u32(x);
	assert!(msb_index >= 0 && lsb_index >= 0);
	if msb_index != lsb_index {
		msb_index += 1;
	}
	assert!(msb_index < 32);
	1u32 << msb_index
}

pub fn roundup_pow2_u64(x: u64) -> u64
{
	let mut msb_index = msb_u64(x);
	let lsb_index = lsb_u64(x);
	assert!(msb_index >= 0 && lsb_index >= 0);
	if msb_index != lsb_index {
		msb_index += 1;
	}
	assert!(msb_index < 64);
	1u64 << msb_index
}