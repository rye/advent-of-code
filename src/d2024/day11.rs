use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution;

#[derive(Clone, Debug)]
struct RockLine {
	stones: Vec<u64>,
}

impl RockLine {
	fn blink(&mut self) {
		let mut new_stones = Vec::new();

		for stone in &self.stones {
			if *stone == 0 {
				new_stones.push(1);
			} else if (stone.ilog10() + 1) % 2 == 0 {
				let s = stone.to_string();
				let mid = s.len() / 2;
				let (left, right) = s.split_at(mid);
				let left: u64 = left.parse().unwrap_or(0);
				let right: u64 = right.parse().unwrap_or(0);
				new_stones.push(left);
				new_stones.push(right);
			} else {
				new_stones.push(stone * 2024);
			}
		}

		self.stones = new_stones;
	}
}

#[cfg(test)]
mod rock_line {
	use super::*;

	#[test]
	fn test_blink() {
		let mut line = RockLine {
			stones: vec![0, 1, 10, 99, 999],
		};
		line.blink();
		assert_eq!(vec![1, 2024, 1, 0, 9, 9, 2021976], line.stones);
	}
}

impl PartSolve for Solution {
	fn parse(&mut self, arrangement: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		let stones = arrangement
			.split_ascii_whitespace()
			.map(|rock| rock.parse::<u64>().expect("failed to parse rock number"))
			.collect();

		Ok(Box::new(RockLine { stones }))
	}

	fn part_one(&self, rock_line: &Box<dyn core::any::Any>) -> Option<String> {
		let rock_line = rock_line.downcast_ref::<RockLine>()?;
		let mut rock_line: RockLine = (*rock_line).to_owned();

		for _ in 0..25 {
			rock_line.blink();
		}

		Some(rock_line.stones.len().to_string())
	}

	fn part_two(&self, _intermediate: &Box<dyn core::any::Any>) -> Option<String> {
		None
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day11.example.in.txt", part_one, literal 55312);
part_test!(part_two, Solution, None, part_two, None);
