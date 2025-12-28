use core::str::FromStr;

use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution;

struct Grid {
	rolls: Vec<Vec<bool>>,
}

impl Grid {
	fn char_to_roll_value(c: char) -> bool {
		match c {
			'@' => true,
			'.' => false,
			_ => false,
		}
	}

	fn roll_exists_at(&self, x: i32, y: i32) -> bool {
		if x < 0 || y < 0 {
			return false;
		}

		let x = x as usize;
		let y = y as usize;

		if y >= self.rolls.len() || x >= self.rolls[y].len() {
			return false;
		}

		self.rolls[y][x]
	}

	fn num_accessible_rolls(&self) -> usize {
		let mut num_accessible_rolls = 0;

		// The forklifts can only access a roll of paper if there are *fewer than four rolls of paper in the eight adjacent positions*.

		for (y, row) in self.rolls.iter().enumerate() {
			for (x, &cell) in row.iter().enumerate() {
				// If current cell contains a roll of paper, check its neighbors using util::neighbors::get().
				if cell {
					let mut num_adjacent_rolls = 0;

					for (neighbor_x, neighbor_y) in crate::util::neighbors::get(&(x as i32, y as i32)) {
						if self.roll_exists_at(neighbor_x, neighbor_y) {
							num_adjacent_rolls += 1;
						}
					}

					if num_adjacent_rolls < 4 {
						num_accessible_rolls += 1;
					}
				}
			}
		}

		num_accessible_rolls
	}
}

impl FromStr for Grid {
	type Err = anyhow::Error;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		// In the grid, @ represents a roll, . represents an empty space.

		let rolls = str
			.lines()
			.map(|line| line.chars().map(Grid::char_to_roll_value).collect())
			.collect();

		Ok(Grid { rolls })
	}
}

impl PartSolve for Solution {
	fn parse(&mut self, input: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		Ok(Box::new(Grid::from_str(input)?))
	}

	fn part_one(&self, grid: &dyn core::any::Any) -> Option<String> {
		let grid: &Grid = grid.downcast_ref::<Grid>()?;
		Some(grid.num_accessible_rolls().to_string())
	}

	fn part_two(&self, _intermediate: &dyn core::any::Any) -> Option<String> {
		None
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day04.example.in.txt", part_one, literal "13");
part_test!(part_two, Solution, file "day04.example.in.txt", part_two, None);
