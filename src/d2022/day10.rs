use core::{
	array::TryFromSliceError,
	fmt::{self, Debug, Display, Formatter},
	num::ParseIntError,
	str::FromStr,
};

pub enum Instruction {
	Noop,
	Addx(i32),
}

impl FromStr for Instruction {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match &s[0..4] {
			"noop" => Ok(Self::Noop),
			"addx" => s[5..].parse().map(Self::Addx),
			_ => unreachable!(),
		}
	}
}

#[derive(PartialEq)]
pub enum Output {
	PartOne(u64),
	PartTwo(String),
}

pub type Intermediate = Vec<Instruction>;

impl Display for Output {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::PartOne(u32) => write!(f, "{u32}"),
			Self::PartTwo(string) => {
				writeln!(f)?;
				write!(f, "{string}")?;
				Ok(())
			}
		}
	}
}

impl Debug for Output {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::PartOne(u32) => write!(f, "{u32:?}"),
			Self::PartTwo(string) => write!(f, "{string:?}"),
		}
	}
}

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(
		input
			.lines()
			.map(str::parse)
			.collect::<Result<Vec<_>, _>>()?,
	)
}

fn values_during_and_after(instructions: &[Instruction]) -> (Vec<i32>, Vec<i32>) {
	let mut reg_x = 1_i32;

	let mut values_during = vec![reg_x];
	let mut values_after = vec![reg_x];

	for instruction in instructions {
		match instruction {
			Instruction::Noop => {
				// Noop takes one cycle and has no effect.
				values_during.push(reg_x);
				values_after.push(reg_x);
			}
			Instruction::Addx(v) => {
				// Addx takes two cycles to complete.

				// First cycle: push current values.
				values_during.push(reg_x);
				values_after.push(reg_x);

				// Second cycle: reg value is updated after the during collection completes.
				values_during.push(reg_x);
				reg_x += v;
				values_after.push(reg_x);
			}
		}
	}

	(values_during, values_after)
}

#[test]
fn values_during_and_after_short_example() {
	use Instruction::{Addx, Noop};
	let instructions = vec![Noop, Addx(3), Addx(-5)];

	let (values_during, values_after) = values_during_and_after(&instructions);

	assert_eq!(values_during, vec![1, 1, 1, 1, 4, 4]);
	assert_eq!(values_after, vec![1, 1, 1, 4, 4, -1]);

	assert_eq!(values_during[1], 1);
	assert_eq!(values_after[1], 1);

	assert_eq!(values_during[2], 1);
	assert_eq!(values_after[2], 1);

	assert_eq!(values_during[3], 1);
	assert_eq!(values_after[3], 4);

	assert_eq!(values_during[4], 4);
	assert_eq!(values_after[4], 4);

	assert_eq!(values_during[5], 4);
	assert_eq!(values_after[5], -1);
}

#[must_use]
pub fn part_one(instructions: &Intermediate) -> Option<Output> {
	let (values_during, _values_after) = values_during_and_after(instructions);

	Some(Output::PartOne(
		[20, 60, 100, 140, 180, 220]
			.map(|cycle: i32| values_during[cycle.unsigned_abs() as usize] * cycle)
			.iter()
			.sum::<i32>()
			.try_into()
			.expect("failed to convert sum to u64"),
	))
}

crate::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("examples/day10-longer.example.in.txt"),
	Some(Output::PartOne(13140))
);

fn render_crt_buffer(buffer: [[bool; 40]; 6]) -> String {
	use core::fmt::Write;

	let mut string: String = String::default();

	for row in buffer {
		for cell in row {
			write!(string, "{}", if cell { "#" } else { "." }).expect("could not write cell");
		}
		writeln!(string).expect("could not write line terminator");
	}

	string
}

#[must_use]
pub fn part_two(instructions: &Intermediate) -> Option<Output> {
	#[allow(clippy::similar_names)]
	let (during, after) = values_during_and_after(instructions);

	assert_eq!(during.len(), 241);
	assert_eq!(after.len(), 241);

	let mut scan_output: Vec<bool> = vec![];

	for cycle in 1..=240 {
		let cycle: i32 = cycle;
		let value = during[cycle.unsigned_abs() as usize];

		let position: i32 = (cycle - 1) % 40;

		if (value - 1..=value + 1).contains(&position) {
			scan_output.push(true);
		} else {
			scan_output.push(false);
		}
	}

	scan_output
		.chunks_exact(40)
		.map(|chunk| -> Result<[bool; 40], _> { chunk.try_into() })
		.collect::<Result<Vec<[bool; 40]>, TryFromSliceError>>()
		.expect("could not split into 40-slices")
		.try_into()
		.ok()
		.map(render_crt_buffer)
		.map(Output::PartTwo)
}

#[test]
fn part_two_example() {
	let input = include_str!("examples/day10-longer.example.in.txt");

	assert_eq!(
		part_two(&parse(input).expect("failed to parse")),
		Some(Output::PartTwo(
			include_str!("examples/day10-longer-image.example.in.txt").to_string()
		))
	);
}

crate::generate_solver!(solve, =>, self);
