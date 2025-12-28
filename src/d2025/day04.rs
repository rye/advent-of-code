use core::str::FromStr;

use crate::{PartSolve, Solver, export_solver, part_test, util::neighbors};

#[derive(Default)]
struct Solution;

#[derive(Clone)]
struct Grid {
	rolls: Vec<Vec<bool>>,
}

impl Grid {
	const fn char_to_roll_value(c: char) -> bool {
		matches!(c, '@')
	}

	fn roll_exists_at(&self, x: i32, y: i32) -> bool {
		if x < 0 || y < 0 {
			return false;
		}

		let x = usize::try_from(x).unwrap();
		let y = usize::try_from(y).unwrap();

		if y >= self.rolls.len() || x >= self.rolls[y].len() {
			return false;
		}

		self.rolls[y][x]
	}

	fn accessible_rolls(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
		// The forklifts can only access a roll of paper if there are *fewer than four rolls of paper in the eight adjacent positions*.
		self.rolls.iter().enumerate().flat_map(move |(y, row)| {
			row.iter().enumerate().filter_map(move |(x, &cell)| {
				if cell {
					let mut num_adjacent_rolls = 0;

					let x_i32 = i32::try_from(x).expect("x out of range");
					let y_i32 = i32::try_from(y).expect("y out of range");

					for (neighbor_x, neighbor_y) in neighbors::get(&(x_i32, y_i32)) {
						if self.roll_exists_at(neighbor_x, neighbor_y) {
							num_adjacent_rolls += 1;
						}
					}

					if num_adjacent_rolls < 4 {
						return Some((x, y));
					}
				}

				None
			})
		})
	}

	fn num_accessible_rolls(&self) -> usize {
		self.accessible_rolls().count()
	}

	fn remove_accessible_rolls_once(&mut self) -> usize {
		let mut removed_count = 0;
		let accessible_rolls: Vec<(usize, usize)> = self.accessible_rolls().collect();

		for (x, y) in accessible_rolls {
			self.rolls[y][x] = false;
			removed_count += 1;
		}

		removed_count
	}

	fn remove_all_accessible_rolls_iteratively(&mut self) -> usize {
		let mut total_removed = 0;

		loop {
			let removed_this_round = self.remove_accessible_rolls_once();
			if removed_this_round == 0 {
				break;
			}
			total_removed += removed_this_round;
		}

		total_removed
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

	fn part_two(&self, grid: &dyn core::any::Any) -> Option<String> {
		let grid: &Grid = grid.downcast_ref::<Grid>()?;
		let mut grid = grid.clone();
		Some(grid.remove_all_accessible_rolls_iteratively().to_string())
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day04.example.in.txt", part_one, literal "13");
part_test!(part_two, Solution, file "day04.example.in.txt", part_two, literal "43");
