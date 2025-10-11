use itertools::Itertools;

pub type Intermediate = Vec<Vec<u32>>;
pub type Output = u32;

/// # Errors
pub fn parse(spreadsheet: &str) -> anyhow::Result<Intermediate> {
	Ok(
		spreadsheet
			.lines()
			.map(|line| {
				line
					.split_whitespace()
					.map(str::parse)
					.collect::<Result<Vec<u32>, _>>()
			})
			.collect::<Result<Vec<Vec<u32>>, _>>()?,
	)
}

#[must_use]
pub fn part_one(spreadsheet: &Intermediate) -> Option<Output> {
	Some(
		spreadsheet
			.iter()
			.map(|line| (line.iter().min(), line.iter().max()))
			.map(|(min, max)| max.unwrap_or(&0) - min.unwrap_or(&0))
			.sum(),
	)
}

#[must_use]
pub fn part_two(spreadsheet: &Intermediate) -> Option<Output> {
	Some(
		spreadsheet
			.iter()
			.filter_map(|line| {
				line
					.iter()
					.permutations(2)
					.find(|pair| pair[0] % pair[1] == 0)
					.map(|pair| pair[0] / pair[1])
			})
			.sum(),
	)
}

crate::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("examples/day02-1.example.in.txt"),
	Some(18)
);

crate::test_example!(
	part_two_example,
	parse,
	part_two,
	include_str!("examples/day02-2.example.in.txt"),
	Some(9)
);

crate::generate_solver!(solve, =>, self);
