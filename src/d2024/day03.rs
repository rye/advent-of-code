use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution {
	memory: Option<String>,
}

enum Command {
	Mul(i32, i32),
	Do,
	Dont,
}

impl Solution {
	fn parse_mul(str: &str) -> Option<Command> {
		// For this mul to be valid, the next character must be '(', followed by a sequence of digits, followed by a ',', followed by another sequence of digits, followed by a ')'.

		// First, check the next character. If it's not a (, we're done.
		let Some('(') = str.chars().next() else {
			return None;
		};

		// We have a left paren, find its closing paren. If we don't have one, we're done.
		let closing_paren_offset = str.find(')')?;

		// Scan the range between them for illegal characters.
		if str[1..closing_paren_offset]
			.find(|c: char| !c.is_ascii_digit() && c != ',')
			.is_some()
		{
			return None;
		}

		// At this point, range 1..closing_paren_offset is known to contain only digits or commas.
		// Double check it contains at least one comma:

		let comma_offset = str[1..closing_paren_offset].find(',')?;

		// Let "a" be the text to the left of the comma, "b" be to the right:

		let a = &str[1..closing_paren_offset][0..comma_offset];
		let b = &str[1..closing_paren_offset][comma_offset + 1..];

		let a = a.parse::<i32>().ok()?;
		let b = b.parse::<i32>().ok()?;

		Some(Command::Mul(a, b))
	}

	fn parse_don_t(str: &str) -> Option<Command> {
		let mut chars = str.chars();
		match (chars.next(), chars.next()) {
			(Some('('), Some(')')) => Some(Command::Dont),
			_ => None,
		}
	}

	fn parse_do(str: &str) -> Option<Command> {
		let mut chars = str.chars();
		match (chars.next(), chars.next()) {
			(Some('('), Some(')')) => Some(Command::Do),
			_ => None,
		}
	}
}

impl PartSolve for Solution {
	fn parse(&mut self, memory: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		self.memory = Some(memory.to_string());
		Ok(Box::new(()))
	}

	fn part_one(&self, _intermediate: &Box<dyn core::any::Any>) -> Option<String> {
		let Some(memory) = &self.memory else {
			return None;
		};

		let result = memory
			.match_indices("mul")
			.filter_map(|(abs_idx, str)| {
				let remainder = &memory[abs_idx + str.len()..];
				Solution::parse_mul(remainder)
			})
			.map(|cmd| match cmd {
				Command::Mul(a, b) => a * b,
				_ => 0_i32,
			})
			.sum::<i32>()
			.to_string();

		Some(result)
	}

	fn part_two(&self, _intermediate: &Box<dyn core::any::Any>) -> Option<String> {
		let Some(memory) = &self.memory else {
			return None;
		};

		let mul_locs = memory.match_indices("mul");
		let do_locs = memory.match_indices("do");
		let don_t_locs = memory.match_indices("don't");

		let mut possible_command_locs: Vec<(usize, &str)> = vec![mul_locs, do_locs, don_t_locs]
			.into_iter()
			.flatten()
			.collect();

		possible_command_locs.sort_by(|(a_idx, a_cmd_str), (b_idx, b_cmd_str)| {
			a_idx.cmp(b_idx).then_with(|| a_cmd_str.cmp(b_cmd_str))
		});

		let result = possible_command_locs
			.into_iter()
			.filter_map(|(abs_idx, str)| match str {
				"mul" => {
					let remainder = &memory[abs_idx + str.len()..];
					Self::parse_mul(remainder)
				}
				"don't" => {
					let remainder = &memory[abs_idx + str.len()..];
					Self::parse_don_t(remainder)
				}
				"do" => {
					let remainder = &memory[abs_idx + str.len()..];
					Self::parse_do(remainder)
				}
				_ => None,
			})
			.fold((0_i32, true), |(sum, enabled), cmd| match cmd {
				Command::Mul(a, b) => (if enabled { sum + a * b } else { sum }, enabled),
				Command::Dont => (sum, false),
				Command::Do => (sum, true),
			})
			.0
			.to_string();

		Some(result)
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution::default())));

part_test!(part_one, Solution::default(), file "day03.example-1.in.txt", part_one, literal "161");

part_test!(part_two, Solution::default(), file "day03.example-2.in.txt", part_two, literal "48");
