use std::collections::HashMap;

pub type Intermediate = Layout;
pub type Output = usize;

pub fn parse(input: &str) -> Result<Intermediate, LayoutParseError> {
	input.parse()
}

pub fn part_one(intermediate: &Intermediate) -> Option<Output> {
	let mut layout = intermediate.clone();

	let mut prev_layout = None;

	while prev_layout.is_none() || layout != prev_layout.unwrap() {
		prev_layout = Some(layout.clone());
		layout = layout.advance(transition, Layout::occupied_neighbors, 4_usize);
	}

	Some(
		layout
			.cells
			.values()
			.filter(|cell| **cell == Occupied)
			.count(),
	)
}

pub fn part_two(intermediate: &Intermediate) -> Option<Output> {
	let mut layout = intermediate.clone();

	let mut prev_layout = None;

	while prev_layout.is_none() || layout != prev_layout.unwrap() {
		prev_layout = Some(layout.clone());
		layout = layout.advance(transition, Layout::visible_occupied_neighbors, 5_usize);
	}

	Some(
		layout
			.cells
			.values()
			.filter(|cell| **cell == Occupied)
			.count(),
	)
}

fn transition<F>(
	layout: &Layout,
	coords: Coords,
	cell: &CellState,
	occupied_neighbors_fn: F,
	crowding_threshold: usize,
) -> CellState
where
	F: Fn(&Layout, Coords) -> usize,
{
	let neighbors = occupied_neighbors_fn(layout, coords);

	match cell {
		Floor => Floor,
		Empty => {
			if neighbors == 0 {
				Occupied
			} else {
				Empty
			}
		}
		Occupied => {
			if neighbors >= crowding_threshold {
				Empty
			} else {
				Occupied
			}
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum CellState {
	Floor,
	Empty,
	Occupied,
}

type Coords = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Layout {
	cells: HashMap<Coords, CellState>,
	width: usize,
	height: usize,
}

use CellState::{Empty, Floor, Occupied};

const DELTAS: [(i32, i32); 8] = [
	(-1, -1),
	(-1, 0),
	(-1, 1),
	(0, -1),
	(0, 1),
	(1, -1),
	(1, 0),
	(1, 1),
];

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum LayoutParseError {
	#[error("unexpected character")]
	UnexpectedChar(char),
}

impl core::str::FromStr for Layout {
	type Err = LayoutParseError;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let mut cells = HashMap::new();
		let mut width = 0;
		let mut height = 0;

		for (row_idx, line) in input.trim_end().split('\n').enumerate() {
			for (col_idx, ch) in line.chars().enumerate() {
				let cell = match ch {
					'L' => Empty,
					'.' => Floor,
					'#' => Occupied,
					_ => return Err(LayoutParseError::UnexpectedChar(ch)),
				};

				cells.insert((row_idx, col_idx), cell);
				width = width.max(col_idx + 1);
			}
			height = height.max(row_idx + 1);
		}

		Ok(Layout {
			cells,
			width,
			height,
		})
	}
}

impl Layout {
	fn new(width: usize, height: usize) -> Layout {
		Layout {
			width,
			height,
			cells: HashMap::new(),
		}
	}

	fn advance<TF, ONF>(
		&self,
		transition: TF,
		occupied_neighbors_fn: ONF,
		crowding_threshold: usize,
	) -> Layout
	where
		TF: Fn(&Layout, Coords, &CellState, ONF, usize) -> CellState,
		ONF: Fn(&Layout, Coords) -> usize + Copy,
	{
		let mut new_layout = Layout::new(self.width, self.height);

		for row_idx in 0..self.height {
			for col_idx in 0..self.width {
				let coords = (row_idx, col_idx);
				let cell = self.cells.get(&coords).unwrap();

				let new_cell = transition(
					self,
					coords,
					cell,
					occupied_neighbors_fn,
					crowding_threshold,
				);
				new_layout.cells.insert(coords, new_cell);
			}
		}

		new_layout
	}

	fn occupied_neighbors(&self, coords: Coords) -> usize {
		DELTAS.iter().fold(0, |count, &delta| {
			if let Some(visible_coords) = self.delta_n(coords, delta, 1) {
				if let Some(Occupied) = self.cells.get(&visible_coords) {
					count + 1
				} else {
					count
				}
			} else {
				count
			}
		})
	}

	fn visible_occupied_neighbors(&self, coords: Coords) -> usize {
		DELTAS.iter().fold(0, |count, &delta| {
			if let Some(true) = (1..).find_map(|n| {
				if let Some(visible_coords) = self.delta_n(coords, delta, n) {
					match self.cells.get(&visible_coords) {
						Some(Occupied) => Some(true),
						Some(Empty) | None => Some(false),
						_ => None,
					}
				} else {
					Some(false)
				}
			}) {
				count + 1
			} else {
				count
			}
		})
	}

	fn delta_n(
		&self,
		(row_idx, col_idx): Coords,
		(row_delta, col_delta): (i32, i32),
		n: i32,
	) -> Option<Coords> {
		let (row_idx, col_idx) = (
			i32::try_from(row_idx).unwrap() + row_delta * n,
			i32::try_from(col_idx).unwrap() + col_delta * n,
		);

		if row_idx >= 0
			&& col_idx >= 0
			&& row_idx < i32::try_from(self.height).unwrap()
			&& col_idx < i32::try_from(self.width).unwrap()
		{
			Some((
				usize::try_from(row_idx).unwrap(),
				usize::try_from(col_idx).unwrap(),
			))
		} else {
			None
		}
	}
}

#[test]
fn layout_parse() {
	let s: &str = "L.L\n.L.\nL.#";

	let expected = {
		let mut map = HashMap::new();

		map.insert((0, 0), CellState::Empty);
		map.insert((0, 2), CellState::Empty);
		map.insert((1, 1), CellState::Empty);
		map.insert((2, 0), CellState::Empty);

		map.insert((0, 1), CellState::Floor);
		map.insert((1, 0), CellState::Floor);
		map.insert((1, 2), CellState::Floor);
		map.insert((2, 1), CellState::Floor);

		map.insert((2, 2), CellState::Occupied);

		map
	};

	assert_eq!(
		s.parse(),
		Ok(Layout {
			cells: expected,
			width: 3,
			height: 3
		})
	);
}

daocutil::generate_solver!(solve, =>, self);
