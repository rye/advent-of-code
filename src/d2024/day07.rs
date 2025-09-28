use itertools::{Itertools, repeat_n};

use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Operation {
	Add,
	Mul,
	Concat,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Symbol {
	Literal(u64),
	Op(Operation),
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Expression(Vec<Symbol>);

impl Expression {
	fn eval(self) -> u64 {
		let result: Option<(u64, Option<Operation>)> =
			self.0.iter().fold(None, |acc, sym| match (acc, sym) {
				// "First" literal -> load that as the value.
				(None, Symbol::Literal(n)) => Some((*n, None)),
				(Some((val, None)), Symbol::Op(op)) => Some((val, Some(*op))),
				(Some((val, Some(op))), Symbol::Literal(n)) => match op {
					Operation::Add => Some((val + n, None)),
					Operation::Mul => Some((val * n, None)),
					Operation::Concat => Some((
						val * 10u64.pow(u32::try_from(n.to_string().len()).unwrap()) + n,
						None,
					)),
				},
				_ => unreachable!("should be impossible to reach this point!"),
			});

		result.unwrap().0
	}
}

impl core::fmt::Display for Expression {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		for symbol in &self.0 {
			match symbol {
				Symbol::Literal(n) => write!(f, "{n}")?,

				Symbol::Op(op) => match op {
					Operation::Add => write!(f, " + ")?,
					Operation::Mul => write!(f, " * ")?,
					Operation::Concat => write!(f, " || ")?,
				},
			}
		}

		Ok(())
	}
}

fn generate_possible_expressions(
	numbers: &[u64],
	allowed_operations: Vec<Operation>,
) -> impl Iterator<Item = Expression> {
	repeat_n(allowed_operations, numbers.len() - 1)
		.multi_cartesian_product()
		.map(|op_string| {
			debug_assert_eq!(numbers.len(), op_string.len() + 1);

			let iter_max = numbers.len();
			let mut equation = Expression(Vec::with_capacity(iter_max * 2));

			for n in 0..iter_max {
				if n == 0 {
					// start with 0 + first number
					equation.0.push(Symbol::Literal(0));
					equation.0.push(Symbol::Op(Operation::Add));

					// then just add the first number
					equation.0.push(Symbol::Literal(numbers[0]));
				} else {
					// for future runs, grab an operator and then the next number
					equation.0.push(Symbol::Op(op_string[n - 1]));
					equation.0.push(Symbol::Literal(numbers[n]));
				}
			}

			equation
		})
}

impl PartSolve for Solution {
	fn parse(&mut self, equations: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		let equations: Vec<(u64, Vec<u64>)> = equations
			.lines()
			.map(|line| {
				let (lhs, rhs) = {
					let mut split = line.split(':');
					let lhs: u64 = split
						.next()
						.map(|str| str.parse().expect("invalid lhs"))
						.expect("missing lhs");

					let rhs: Vec<u64> = split
						.next()
						.expect("missing rhs")
						.split_whitespace()
						.map(|piece| piece.parse().expect("invalid rhs"))
						.collect();

					(lhs, rhs)
				};

				(lhs, rhs)
			})
			.collect();

		Ok(Box::new(equations))
	}

	fn part_one(&self, equations: &Box<dyn core::any::Any>) -> Option<String> {
		let equations = equations.downcast_ref::<Vec<(u64, Vec<u64>)>>()?;

		let mut sum: u64 = 0;

		for (lhs, rhs_numbers) in equations {
			for equation in
				generate_possible_expressions(rhs_numbers, vec![Operation::Add, Operation::Mul])
			{
				if equation.eval() == *lhs {
					sum += lhs;
					break;
				}
			}
		}

		Some(sum.to_string())
	}

	fn part_two(&self, equations: &Box<dyn core::any::Any>) -> Option<String> {
		let equations = equations.downcast_ref::<Vec<(u64, Vec<u64>)>>()?;

		let mut sum: u64 = 0;

		for (lhs, rhs_numbers) in equations {
			for equation in generate_possible_expressions(
				rhs_numbers,
				vec![Operation::Add, Operation::Mul, Operation::Concat],
			) {
				if equation.eval() == *lhs {
					sum += lhs;
					break;
				}
			}
		}

		Some(sum.to_string())
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day07.example.in.txt", part_one, literal "3749");

part_test!(part_two, Solution, file "day07.example.in.txt", part_two, literal "11387");
