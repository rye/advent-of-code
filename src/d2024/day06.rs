use std::collections::HashSet;

use crate::{PartSolve, Solver, export_solver};

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
	North,
	East,
	South,
	West,
}

impl Direction {
	fn right(&self) -> Self {
		match self {
			Direction::North => Direction::East,
			Direction::East => Direction::South,
			Direction::South => Direction::West,
			Direction::West => Direction::North,
		}
	}
}

#[derive(Clone)]
enum MapValue {
	Empty,
	Obstruction,
	Visited,
	Guard,
}

#[derive(Default)]
struct Solution;

#[derive(Clone)]
struct Grid {
	values: Vec<Vec<MapValue>>,
}

impl core::str::FromStr for Grid {
	type Err = anyhow::Error;

	fn from_str(str: &str) -> anyhow::Result<Self> {
		let values: Vec<Vec<MapValue>> = str
			.lines()
			.map(|line| {
				line
					.trim()
					.chars()
					.filter_map(|c| match c {
						'.' => Some(MapValue::Empty),
						'#' => Some(MapValue::Obstruction),
						'^' => Some(MapValue::Guard),
						_ => None,
					})
					.collect()
			})
			.collect();

		Ok(Grid { values })
	}
}

#[derive(Clone)]
struct SolverState {
	grid: Grid,
	position: Option<(usize, usize)>,
	direction: Direction,
}

impl From<&Grid> for SolverState {
	fn from(grid: &Grid) -> Self {
		let grid = grid.clone();

		let position = grid.values.iter().enumerate().find_map(|(y, row)| {
			row
				.iter()
				.position(|v| matches!(v, MapValue::Guard))
				.map(|x| (x, y))
		});

		SolverState {
			grid,
			position,
			direction: Direction::North,
		}
	}
}

impl SolverState {
	fn next_block_pos(&self) -> Option<(usize, usize)> {
		// If we already exited the grid, there is no "next" block.
		let (x, y) = self.position?;

		match self.direction {
			Direction::North => {
				if y == 0 {
					None
				} else {
					Some((x, y - 1))
				}
			}
			Direction::East => {
				if x + 1 >= self.grid.values[0].len() {
					None
				} else {
					Some((x + 1, y))
				}
			}
			Direction::South => {
				if y + 1 >= self.grid.values.len() {
					None
				} else {
					Some((x, y + 1))
				}
			}
			Direction::West => {
				if x == 0 {
					None
				} else {
					Some((x - 1, y))
				}
			}
		}
	}

	fn next_block_type(&self) -> Option<MapValue> {
		let (x, y) = self.next_block_pos()?;

		Some(self.grid.values[y][x].clone())
	}

	fn get_value_at_pos(&self, pos: (usize, usize)) -> Option<&MapValue> {
		let (x, y) = pos;
		self.grid.values.get(y).and_then(|row| row.get(x))
	}

	fn turn_right(&mut self) {
		self.direction = self.direction.right();
	}

	fn mark_current_spot_visited(&mut self) {
		if let Some((x, y)) = self.position {
			self.grid.values[y][x] = MapValue::Visited;
		}
	}

	fn count_visited(&self) -> usize {
		self
			.grid
			.values
			.iter()
			.map(|row| {
				row
					.iter()
					.filter(|v| matches!(v, MapValue::Visited))
					.count()
			})
			.sum()
	}

	fn forms_loop(&self) -> bool {
		let mut working_state = self.clone();

		let mut seen_positions = HashSet::new();

		while let Some(pos) = working_state.position {
			if !seen_positions.insert((pos, working_state.direction.clone())) {
				// We've seen this position before, so we found a loop.
				return true;
			}

			// Continue moving in the current direction.
			working_state.tick();
		}

		false
	}

	fn tick(&mut self) {
		if let Some(MapValue::Obstruction) = self.next_block_type() {
			self.turn_right();
			return;
		}

		self.mark_current_spot_visited();
		self.position = self.next_block_pos();
	}

	fn all_visited_positions(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
		self
			.grid
			.values
			.iter()
			.enumerate()
			.flat_map(|(y, row_values)| {
				row_values.iter().enumerate().filter_map(move |(x, v)| {
					if matches!(v, MapValue::Visited) {
						Some((x, y))
					} else {
						None
					}
				})
			})
	}
}

fn neighbors((x, y): (usize, usize)) -> HashSet<(usize, usize)> {
	[
		(x.checked_sub(1), y.checked_sub(1)),
		(Some(x), y.checked_sub(1)),
		(x.checked_add(1), y.checked_sub(1)),
		(x.checked_sub(1), Some(y)),
		(Some(x), Some(y)),
		(x.checked_add(1), Some(y)),
		(x.checked_sub(1), y.checked_add(1)),
		(Some(x), y.checked_add(1)),
		(x.checked_add(1), y.checked_add(1)),
	]
	.iter()
	.filter_map(|(x, y)| x.and_then(|x| y.map(|y| (x, y))))
	.collect()
}

impl PartSolve for Solution {
	fn parse(&mut self, input: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		Ok(Box::new(input.parse::<Grid>()?))
	}

	fn part_one(&self, grid: &Box<dyn core::any::Any>) -> Option<String> {
		let grid = grid.downcast_ref::<Grid>()?;

		let mut state = SolverState::from(grid);

		while state.position.is_some() {
			state.tick();
		}

		Some(state.count_visited().to_string())
	}

	fn part_two(&self, grid: &Box<dyn core::any::Any>) -> Option<String> {
		let grid = grid.downcast_ref::<Grid>()?;

		let mut state = SolverState::from(grid);

		while state.position.is_some() {
			state.tick();
		}

		let mut candidates: HashSet<(usize, usize)> = HashSet::new();

		let visited_positions: HashSet<(usize, usize)> = state.all_visited_positions().collect();

		for visited_position in visited_positions {
			for neighbor in neighbors(visited_position) {
				if let Some(MapValue::Empty | MapValue::Visited) = state.get_value_at_pos(neighbor) {
					candidates.insert(neighbor);
				}
			}
		}

		let candidates_that_form_a_loop = candidates
			.iter()
			.filter(|blocker_pos| {
				let mut test_grid = grid.clone();
				test_grid.values[blocker_pos.1][blocker_pos.0] = MapValue::Obstruction;

				let test_state = SolverState::from(&test_grid);

				if test_state.forms_loop() {
					return true;
				}

				false
			})
			.count();

		Some(candidates_that_form_a_loop.to_string())
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

#[test]
fn part_one() {
	let mut solver = Solution;

	let intermediate = solver.parse(include_str!("day06.example.in.txt")).unwrap();

	assert_eq!(Some("41".to_string()), solver.part_one(&intermediate));
}

#[test]
fn part_two() {
	let mut solver = Solution;

	let intermediate = solver.parse(include_str!("day06.example.in.txt")).unwrap();

	assert_eq!(Some("6".to_string()), solver.part_two(&intermediate));
}
