/**
@args 
*/
pub fn lsb_u8(x: u8) -> i8 {
	let mut found = false;
	let mut least_index = 8;
	
	for index in 0..8 {
		let temp = x << (7 -index);
		if temp != 0 {
			found = true;
			if least_index > index {
				least_index = index;
			}
		}
	}
	
	if found {
		least_index
	} else {
		-1i8
	}
}

pub fn lsb_u16(x: u16) -> i8 {
	let mut found = false;
	let mut least_index = 16;
	
	for index in 0..16 {
		let temp = x << (15 - index);
		if temp != 0 {
			found = true;
			if least_index > index {
				least_index = index;
			}
		}
	}
	
	if found {
		least_index
	} else {
		-1i8
	}
}

pub fn lsb_u32(x: u32) -> i8 {
	let mut found = false;
	let mut least_index = 32;
	
	for index in 0..32 {
		let temp = x << index;
		if temp != 0 {
			found = true;
			if least_index > (31 -index) {
				least_index = index;
			}
		}
	}
	
	if found {
		least_index
	} else {
		-1i8
	}
}

pub fn lsb_u64(x: u64) -> i8 {
	let mut found = false;
	let mut least_index = 64;
	
	for index in 0..64 {
		let temp = x << (63 -index);
		if temp != 0 {
			found = true;
			if least_index > index {
				least_index = index;
			}
		}
	}
	
	if found {
		least_index
	} else {
		-1i8
	}
}