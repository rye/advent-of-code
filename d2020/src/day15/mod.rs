const END: u32 = 30_000_000;

type Intermediate = Vec<u32>;
type Solution = u32;

pub fn parse(input: &str) -> Intermediate {
	input
		.trim()
		.split(',')
		.map(|i| i.parse().expect("invalid input"))
		.collect()
}

#[allow(clippy::ptr_arg)]
pub fn part_one(intermediate: &Intermediate) -> Option<Solution> {
	let mut history = vec![0u32; END as usize];
	let mut last = intermediate[0];
	for turn in 0..intermediate.len() as u32 {
		history[last as usize] = turn as u32;
		last = intermediate[turn as usize];
	}

	for turn in intermediate.len() as u32..2020 {
		let stored = history[last as usize];
		history[last as usize] = turn;
		last = if stored == 0 { 0 } else { turn - stored };
	}

	Some(last)
}

#[allow(clippy::ptr_arg)]
pub fn part_two(intermediate: &Intermediate) -> Option<Solution> {
	let mut history = vec![0u32; END as usize];
	let mut last = intermediate[0];
	for turn in 0..intermediate.len() as u32 {
		history[last as usize] = turn as u32;
		last = intermediate[turn as usize];
	}

	for turn in intermediate.len() as u32..END {
		let stored = history[last as usize];
		history[last as usize] = turn;
		last = if stored == 0 { 0 } else { turn - stored };
	}

	Some(last)
}
