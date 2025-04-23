const END: u32 = 30_000_000;

pub type Intermediate = Vec<u32>;
type Output = u32;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(
		input
			.trim()
			.split(',')
			.map(|i| i.parse().expect("invalid input"))
			.collect(),
	)
}

#[allow(clippy::ptr_arg)]
pub fn part_one(intermediate: &Intermediate) -> Option<Output> {
	let mut history = vec![0u32; END as usize];
	let mut last = intermediate[0];
	for turn in 0..u32::try_from(intermediate.len()).unwrap() {
		history[last as usize] = turn;
		last = intermediate[turn as usize];
	}

	for turn in u32::try_from(intermediate.len()).unwrap()..2020 {
		let stored = history[last as usize];
		history[last as usize] = turn;
		last = if stored == 0 { 0 } else { turn - stored };
	}

	Some(last)
}

#[allow(clippy::ptr_arg)]
pub fn part_two(intermediate: &Intermediate) -> Option<Output> {
	let mut history = vec![0u32; END as usize];
	let mut last = intermediate[0];
	for turn in 0..u32::try_from(intermediate.len()).unwrap() {
		history[last as usize] = turn;
		last = intermediate[turn as usize];
	}

	for turn in u32::try_from(intermediate.len()).unwrap()..END {
		let stored = history[last as usize];
		history[last as usize] = turn;
		last = if stored == 0 { 0 } else { turn - stored };
	}

	Some(last)
}

daocutil::generate_solver!(solve, =>, self);
