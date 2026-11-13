use itertools::Itertools;

use super::intcode::Intcode;

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

#[allow(clippy::many_single_char_names)]
#[must_use]
pub fn part_one(program: &Intermediate) -> Option<Output> {
	let permutations = (0..5).permutations(5);

	let mut max: Option<i32> = None;

	for permutation in permutations {
		// Setting for A is 0.

		// Program takes inputs in this order: phase setting, value.
		let [mut a, mut b, mut c, mut d, mut e] = [
			Intcode::from(program.clone()),
			Intcode::from(program.clone()),
			Intcode::from(program.clone()),
			Intcode::from(program.clone()),
			Intcode::from(program.clone()),
		];

		a.input(permutation[0]);
		a.input(0);
		b.input(permutation[1]);
		c.input(permutation[2]);
		d.input(permutation[3]);
		e.input(permutation[4]);

		// Run A, feed into B
		let mut a = a.run();
		b.input(a.output().expect("expected output from A"));

		// Run B, feed into C
		let mut b = b.run();
		c.input(b.output().expect("expected output from B"));

		// Run C, feed into D
		let mut c = c.run();
		d.input(c.output().expect("expected output from C"));

		// Run D, feed into E
		let mut d = d.run();
		e.input(d.output().expect("expected output from D"));

		let mut e = e.run();
		let output = e.output().expect("expected output from E");

		println!("Setting {permutation:?} produces output {output}");

		if let Some(max_so_far) = max {
			if output > max_so_far {
				max.replace(output);
			}
		} else {
			max.replace(output);
		}
	}

	max
}

#[allow(clippy::many_single_char_names)]
#[must_use]
pub fn part_two(program: &Intermediate) -> Option<Output> {
	let permutations = (5..10).permutations(5);

	let mut max: Option<i32> = None;

	for permutation in permutations {
		// Setting for A is 0.

		// Program takes inputs in this order: phase setting, value.

		let [mut a, mut b, mut c, mut d, mut e] = [
			Intcode::from(program.clone()),
			Intcode::from(program.clone()),
			Intcode::from(program.clone()),
			Intcode::from(program.clone()),
			Intcode::from(program.clone()),
		];

		// Seed each of the amplifiers with their settings...
		a.input(permutation[0]);
		a.input(0);
		b.input(permutation[1]);
		c.input(permutation[2]);
		d.input(permutation[3]);
		e.input(permutation[4]);

		let mut to_thrusters = None;

		loop {
			// Run A, feed into B
			let a_output = a.run_til_next_output().output();

			if let Some(a_output) = a_output {
				b.input(a_output);
			} else {
				break;
			}

			// Run B, feed into C
			let b_output = b
				.run_til_next_output()
				.output()
				.expect("expected output from B");
			c.input(b_output);

			// Run C, feed into D
			let c_output = c
				.run_til_next_output()
				.output()
				.expect("expected output from C");
			d.input(c_output);

			// Run D, feed into E
			let d_output = d
				.run_til_next_output()
				.output()
				.expect("expected output from D");
			e.input(d_output);

			let e_output = e
				.run_til_next_output()
				.output()
				.expect("expected output from E");
			let output = e_output;

			to_thrusters = Some(output);

			a.input(output);
		}

		let output = to_thrusters.expect("loop through thrusters should have occurred at least once!");

		if let Some(max_so_far) = max {
			if output > max_so_far {
				max.replace(output);
			}
		} else {
			max.replace(output);
		}
	}

	max
}

crate::test_example!(
	part_two_139629729,
	parse,
	part_two,
	crate::example!("2019/day07.example01.in.txt"),
	Some(139_629_729)
);

crate::test_example!(
	part_two_18216,
	parse,
	part_two,
	crate::example!("2019/day07.example02.in.txt"),
	Some(18_216)
);

crate::generate_solver!(solve, =>, self);
