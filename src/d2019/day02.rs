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
	let program = {
		let mut program = program.clone();
		program[1] = 12;
		program[2] = 2;
		program
	};

	let mut intcode: intcode::Intcode = intcode::Intcode::from(program);
	intcode = intcode.run();
	let final_memory = intcode.data();

	Some(final_memory[0])
}

#[must_use]
pub fn part_two(program: &Intermediate) -> Option<Output> {
	let mut done = false;
	let mut pair = (0, 0);

	for noun in 0..=100 {
		for verb in 0..=100 {
			let program = {
				let mut program = program.clone();
				program[1] = noun;
				program[2] = verb;
				program
			};

			let output = intcode::Intcode::from(program).run().data()[0];

			if output == 19_690_720 {
				done = true;
				pair = (noun, verb);
				break;
			}
		}
		if done {
			break;
		}
	}

	Some(pair.0 * 100 + pair.1)
}

crate::generate_solver!(solve, =>, self);
