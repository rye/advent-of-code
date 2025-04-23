use super::intcode;

pub type Intermediate = Vec<i32>;
pub type Output = i32;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let program: Vec<i32> = input
		.lines()
		.flat_map(|line| -> Vec<i32> { line.split(',').map(|n| n.parse::<i32>().unwrap()).collect() })
		.collect();

	Ok(program)
}

#[must_use]
pub fn part_one(program: &Intermediate) -> Option<Output> {
	let program = program.clone();
	let mut intcode: intcode::Intcode = intcode::Intcode::from(program);
	intcode.input(1);
	intcode = intcode.run();

	let mut outputs: Vec<i32> = Vec::new();

	while let Some(output) = intcode.output() {
		outputs.push(output);
	}

	Some(*outputs.last().expect("expected an output"))
}

#[must_use]
pub fn part_two(program: &Intermediate) -> Option<Output> {
	let program = program.clone();
	let mut intcode: intcode::Intcode = intcode::Intcode::from(program);
	intcode.input(5);
	intcode = intcode.run();

	let mut outputs: Vec<i32> = Vec::new();

	while let Some(output) = intcode.output() {
		outputs.push(output);
	}

	Some(*outputs.last().expect("expected an output"))
}

daocutil::generate_solver!(solve, =>, self);
