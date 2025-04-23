pub fn binary_hone(instructions: &str, left: char, right: char) -> usize {
	let mut range: core::ops::Range<usize> =
		0..2_usize.pow(u32::try_from(instructions.len()).unwrap());

	for instruction in instructions.chars() {
		let midpoint = usize::midpoint(range.start, range.end);

		match instruction {
			c if c == left => range = range.start..midpoint,
			c if c == right => range = midpoint..range.end,
			_ => panic!("strange instruction"),
		}
	}

	range.start
}

pub fn seat_id_from_bsp(bsp: &str) -> usize {
	let fb: &str = &bsp[0..7];
	let lr: &str = &bsp[7..10];

	let row: usize = binary_hone(fb, 'F', 'B');
	let column: usize = binary_hone(lr, 'L', 'R');

	row * 2_usize.pow(u32::try_from(lr.len()).unwrap()) + column
}

pub type Intermediate = std::collections::BTreeSet<usize>;
pub type Output = usize;

pub fn parse(data: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(data.lines().map(seat_id_from_bsp).collect())
}

pub fn part_one(seat_ids: &Intermediate) -> Option<Output> {
	seat_ids
		.iter()
		.next_back()
		.map(std::borrow::ToOwned::to_owned)
}

pub fn part_two(seat_ids: &Intermediate) -> Option<Output> {
	let min: usize = *seat_ids.iter().next().unwrap();
	let max: usize = *seat_ids.iter().next_back().unwrap();

	(min..=max).find(|seat_id| !seat_ids.contains(seat_id))
}

#[cfg(test)]
mod tests;

crate::generate_solver!(solve, =>, self);
