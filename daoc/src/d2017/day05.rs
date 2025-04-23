pub type Intermediate = Vec<i32>;
pub type Output = usize;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(
		input
			.lines()
			.map(str::parse)
			.collect::<Result<Vec<_>, _>>()?,
	)
}

#[must_use]
pub fn part_one(program: &Intermediate) -> Option<Output> {
	let mut program: Vec<i32> = program.clone();
	let mut offset = 0;

	let mut counter: usize = 0_usize;

	loop {
		step(&mut program, &mut offset, Mode::PartOne);
		counter += 1;

		if offset >= program.len() {
			break;
		}
	}

	Some(counter)
}

#[must_use]
pub fn part_two(program: &Intermediate) -> Option<Output> {
	let mut program: Vec<i32> = program.clone();
	let mut offset = 0;

	let mut counter: usize = 0_usize;

	loop {
		step(&mut program, &mut offset, Mode::PartTwo);
		counter += 1;

		if offset >= program.len() {
			break;
		}
	}

	Some(counter)
}

enum Mode {
	PartOne,
	PartTwo,
}

crate::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("../../inputs/2017/examples/day05.example.in.txt"),
	Some(5)
);

fn step(program: &mut [i32], offset: &mut usize, mode: Mode) {
	let jump_size = program[*offset];
	match (mode, jump_size) {
		(Mode::PartOne, _) => program[*offset] += 1,
		(Mode::PartTwo, sz) if sz >= 3 => program[*offset] -= 1,
		(Mode::PartTwo, sz) if sz < 3 => program[*offset] += 1,
		(Mode::PartTwo, _) => unreachable!(),
	}

	*offset = (i32::try_from(*offset)
		.expect("offset should be greater than zero and less than i32::MAX as usize")
		+ jump_size)
		.unsigned_abs() as usize;
}

crate::test_example!(
	part_two_example,
	parse,
	part_two,
	include_str!("../../inputs/2017/examples/day05.example.in.txt"),
	Some(10)
);

crate::generate_solver!(solve, =>, self);
