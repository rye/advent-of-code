pub type Intermediate = String;
pub type Output = usize;

const RANGE: core::ops::Range<usize> = 1_usize..10_000_000_usize;

const fn is_zero(byte: u8) -> bool {
	byte == 0_u8
}

const fn is_upper_nibble_zero(byte: u8) -> bool {
	(byte >> 4) == 0_u8
}

const fn leading5(data: [u8; 16]) -> bool {
	is_zero(data[0]) && is_zero(data[1]) && is_upper_nibble_zero(data[2])
}

const fn leading6(data: [u8; 16]) -> bool {
	is_zero(data[0]) && is_zero(data[1]) && is_zero(data[2])
}

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(input.lines().map(str::to_string).collect())
}

#[must_use]
pub fn part_one(stub: &Intermediate) -> Option<Output> {
	RANGE
		.map(|n| (n, md5::compute(format!("{stub}{n}"))))
		.find(|n| leading5((n.1).0))
		.map(|tuple| tuple.0)
}

#[must_use]
pub fn part_two(stub: &Intermediate) -> Option<Output> {
	RANGE
		.map(|n| (n, md5::compute(format!("{stub}{n}"))))
		.find(|n| leading6((n.1).0))
		.map(|tuple| tuple.0)
}

daocutil::generate_solver!(solve, =>, self);
