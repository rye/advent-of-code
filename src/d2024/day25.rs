use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution;

impl PartSolve for Solution {
	fn parse(&mut self, _input: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		Ok(Box::new(()))
	}

	fn part_one(&self, _intermediate: &Box<dyn core::any::Any>) -> Option<String> {
		None
	}

	fn part_two(&self, _intermediate: &Box<dyn core::any::Any>) -> Option<String> {
		None
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, None, part_one, None);
part_test!(part_two, Solution, None, part_two, None);
