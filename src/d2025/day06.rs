use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution;

enum Operation {
	Add,
	Mul,
}

impl core::str::FromStr for Operation {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"+" => Ok(Operation::Add),
			"*" => Ok(Operation::Mul),
			_ => Err(anyhow::anyhow!("Unknown operation: {}", s)),
		}
	}
}

struct Homework {
	num_rows: Vec<Vec<u32>>,
	operators: Vec<Operation>,
}

impl Homework {
	fn solve(&self) -> u64 {
		debug_assert!(
			self
				.num_rows
				.iter()
				.all(|row| row.len() == self.operators.len())
		);

		(0..self.num_rows[0].len())
			.map(|col_idx| {
				let mut result = self.num_rows[0][col_idx] as u64;
				let operator = &self.operators[col_idx];

				for num_rows in &self.num_rows[1..] {
					let value = num_rows[col_idx] as u64;
					match operator {
						Operation::Add => result += value,
						Operation::Mul => result *= value,
					}
				}

				result
			})
			.sum()
	}
}

impl PartSolve for Solution {
	fn parse(&mut self, input: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		let lines = input.lines().map(str::trim).collect::<Vec<_>>();

		let (operator_line, number_rows) = lines.split_last().unwrap();

		let num_rows = number_rows
			.iter()
			.map(|line| {
				line
					.split_whitespace()
					.map(|num_str| num_str.parse::<u32>())
					.collect::<Result<Vec<u32>, _>>()
			})
			.collect::<Result<Vec<Vec<u32>>, _>>()?;

		let operators = operator_line
			.split_whitespace()
			.map(|op_str| op_str.parse::<Operation>())
			.collect::<Result<Vec<Operation>, _>>()?;

		let homework = Homework {
			num_rows,
			operators,
		};

		Ok(Box::new(homework))
	}

	fn part_one(&self, homework: &dyn core::any::Any) -> Option<String> {
		let homework = homework.downcast_ref::<Homework>()?;
		Some(homework.solve().to_string())
	}

	fn part_two(&self, _intermediate: &dyn core::any::Any) -> Option<String> {
		None
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day06.example.in.txt", part_one, literal "4277556");
part_test!(part_two, Solution, file "day06.example.in.txt", part_two, None);
