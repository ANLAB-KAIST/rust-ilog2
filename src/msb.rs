pub fn msb_u8(x: u8) -> i8 {
	let mut found = false;
	let mut max_index = 0;
	
	for index in 0..8 {
		let temp = x >> index;
		if temp != 0 {
			found = true;
			if max_index < index {
				max_index = index;
			}
		}
	}
	
	if found {
		max_index
	} else {
		-1i8
	}
}

pub fn msb_u16(x: u16) -> i8 {
	let mut found = false;
	let mut max_index = 0;
	
	for index in 0..16 {
		let temp = x >> index;
		if temp != 0 {
			found = true;
			if max_index < index {
				max_index = index;
			}
		}
	}
	
	if found {
		max_index
	} else {
		-1i8
	}
}

pub fn msb_u32(x: u32) -> i8 {
	let mut found = false;
	let mut max_index = 0;
	
	for index in 0..32 {
		let temp = x >> index;
		if temp != 0 {
			found = true;
			if max_index < index {
				max_index = index;
			}
		}
	}
	
	if found {
		max_index
	} else {
		-1i8
	}
}

pub fn msb_u64(x: u64) -> i8 {
	let mut found = false;
	let mut max_index = 0;
	
	for index in 0..64 {
		let temp = x >> index;
		if temp != 0 {
			found = true;
			if max_index < index {
				max_index = index;
			}
		}
	}
	
	if found {
		max_index
	} else {
		-1i8
	}
}