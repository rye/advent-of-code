use crate::{PartSolve, Solver, export_solver};

#[derive(Default)]
struct Solution {
	grid: Option<Vec<Vec<char>>>,
}

#[derive(Clone, Copy)]
enum Direction {
	Up,
	Left,
	Down,
	Right,
	UpLeft,
	DownLeft,
	DownRight,
	UpRight,
}

impl Direction {
	const fn is_leftwards(&self) -> bool {
		matches!(
			self,
			Direction::Left | Direction::UpLeft | Direction::DownLeft
		)
	}

	const fn is_rightwards(&self) -> bool {
		matches!(
			self,
			Direction::Right | Direction::UpRight | Direction::DownRight
		)
	}

	const fn is_upwards(&self) -> bool {
		matches!(self, Direction::Up | Direction::UpLeft | Direction::UpRight)
	}

	const fn is_downwards(&self) -> bool {
		matches!(
			self,
			Direction::Down | Direction::DownLeft | Direction::DownRight
		)
	}
}

const ALL_DIRS: [Direction; 8] = [
	Direction::Up,
	Direction::UpLeft,
	Direction::Left,
	Direction::DownLeft,
	Direction::Down,
	Direction::DownRight,
	Direction::Right,
	Direction::UpRight,
];

fn neighbor_dirs_for_pos(
	(x, y): (usize, usize),
	grid: &[Vec<char>],
	range: usize,
) -> impl Iterator<Item = Direction> {
	let height = grid.len();
	let width = grid[0].len();
	debug_assert!(grid.iter().all(|row| row.len() == width));

	let all_dirs_min = (range, range);
	let all_dirs_max = (width - 1 - range, height - 1 - range);

	ALL_DIRS.into_iter().filter(move |dir| {
		!(x < all_dirs_min.0 && dir.is_leftwards() || x > all_dirs_max.0 && dir.is_rightwards())
			&& !(y < all_dirs_min.1 && dir.is_upwards() || y > all_dirs_max.1 && dir.is_downwards())
	})
}

impl PartSolve for Solution {
	fn parse(&mut self, grid: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		self.grid = Some(grid.lines().map(|line| line.chars().collect()).collect());
		Ok(Box::new(()))
	}

	fn part_one(&self, _intermediate: &Box<dyn core::any::Any>) -> Option<String> {
		let Some(grid) = &self.grid else {
			return None;
		};

		let mut seeds: Vec<((usize, usize), Direction)> =
			Vec::with_capacity(grid.len() * grid[0].len());

		let height = grid.len();
		let width = grid[0].len();

		for y in 0..height {
			for x in 0..width {
				if grid[y][x] != 'X' {
					continue;
				}

				for neighbor in neighbor_dirs_for_pos((x, y), grid, 3) {
					let direction = neighbor;

					let next_in_dir = match direction {
						Direction::Right => grid[y][x + 1],
						Direction::Left => grid[y][x - 1],
						Direction::Down => grid[y + 1][x],
						Direction::Up => grid[y - 1][x],
						Direction::UpLeft => grid[y - 1][x - 1],
						Direction::UpRight => grid[y - 1][x + 1],
						Direction::DownLeft => grid[y + 1][x - 1],
						Direction::DownRight => grid[y + 1][x + 1],
					};

					if next_in_dir == 'M' {
						seeds.push(((x, y), direction));
					}
				}
			}
		}

		let finds = seeds
			.into_iter()
			.filter(|(x_pos, direction)| {
				let (x, y) = x_pos;

				let (a, s): (Option<char>, Option<char>) = match (x, y, &direction) {
					(x, y, Direction::Left) => (Some(grid[*y][*x - 2]), Some(grid[*y][*x - 3])),
					(x, y, Direction::Right) => (Some(grid[*y][*x + 2]), Some(grid[*y][*x + 3])),
					(x, y, Direction::Up) => (Some(grid[*y - 2][*x]), Some(grid[*y - 3][*x])),
					(x, y, Direction::Down) => (Some(grid[*y + 2][*x]), Some(grid[*y + 3][*x])),
					(x, y, Direction::UpLeft) => (Some(grid[*y - 2][*x - 2]), Some(grid[*y - 3][*x - 3])),
					(x, y, Direction::UpRight) => (Some(grid[*y - 2][*x + 2]), Some(grid[*y - 3][*x + 3])),
					(x, y, Direction::DownLeft) => (Some(grid[*y + 2][*x - 2]), Some(grid[*y + 3][*x - 3])),
					(x, y, Direction::DownRight) => (Some(grid[*y + 2][*x + 2]), Some(grid[*y + 3][*x + 3])),
				};

				match (a, s) {
					(Some('A'), Some('S')) => true,
					(_, _) => false,
				}
			})
			.count();

		Some(finds.to_string())
	}

	fn part_two(&self, _intermediate: &Box<dyn core::any::Any>) -> Option<String> {
		let Some(grid) = &self.grid else {
			return None;
		};

		let height = grid.len();
		let width = grid[0].len();

		let mut sum: usize = 0_usize;

		for y in 1..height - 1 {
			for x in 1..width - 1 {
				if grid[y][x] != 'A' {
					continue;
				}

				let corners = [
					grid[y - 1][x - 1],
					grid[y - 1][x + 1],
					grid[y + 1][x + 1],
					grid[y + 1][x - 1],
				];

				if corners.iter().filter(|&&c| c == 'M').count() != 2 {
					continue;
				}

				if corners.iter().filter(|&&c| c == 'S').count() != 2 {
					continue;
				}

				let top_left = corners[0];
				let top_right = corners[1];
				let bottom_right = corners[2];
				let bottom_left = corners[3];

				// Check that M and S are in the correct positions
				match (top_left, top_right, bottom_right, bottom_left) {
					// Tops both M
					('M', 'M', 'S', 'S') |
					// Bottoms both M
					('S', 'S', 'M', 'M') |
					// Lefts both M
					('M', 'S', 'S', 'M') |
					// Rights both M
					('S', 'M', 'M', 'S') => sum += 1,
					_ => {},
				}
			}
		}

		Some(sum.to_string())
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution::default())));

#[test]
fn part_one() {
	let mut solver = Solution::default();

	let intermediate = solver.parse(include_str!("day04.example.in.txt")).unwrap();

	assert_eq!(Some("18".to_string()), solver.part_one(&intermediate));
}

#[test]
fn part_two() {
	let mut solver = Solution::default();

	let intermediate = solver.parse(include_str!("day04.example.in.txt")).unwrap();

	assert_eq!(Some("9".to_string()), solver.part_two(&intermediate));
}
