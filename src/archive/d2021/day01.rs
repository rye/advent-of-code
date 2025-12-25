pub type Intermediate = Vec<u16>;

pub fn parse(input: &str) -> Result<Intermediate, core::num::ParseIntError> {
	input.lines().map(str::parse).collect()
}

type Output = usize;

pub fn part_one(readings: &Intermediate) -> Option<Output> {
	Some(
		readings
			.windows(2)
			.map(<[u16; 2]>::try_from)
			.filter_map(Result::ok)
			.filter(|[a, b]| a < b)
			.count(),
	)
}

pub fn part_two(readings: &Intermediate) -> Option<Output> {
	Some(
		readings
			.windows(3)
			.map(|window| window.iter().sum())
			.collect::<Box<[u16]>>()
			.windows(2)
			.map(<[u16; 2]>::try_from)
			.filter_map(Result::ok)
			.filter(|[a, b]| a < b)
			.count(),
	)
}

crate::generate_solver!(solve, =>, self);
