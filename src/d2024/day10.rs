use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution;

struct Grid(Vec<Vec<u8>>);

type Path = Vec<(usize, usize)>;

impl Grid {
	// Hiking trails never include diagonal steps, only up/down/left/right.
	//
	// A legal hiking trail move is one where you increase by a height of exactly 1 at each step.
	fn legal_moves(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
		let mut moves = Vec::new();
		let height = self.0[y][x];

		if x > 0 && self.0[y][x - 1] == height + 1 {
			moves.push((x - 1, y));
		}

		if x < self.0[0].len() - 1 && self.0[y][x + 1] == height + 1 {
			moves.push((x + 1, y));
		}

		if y > 0 && self.0[y - 1][x] == height + 1 {
			moves.push((x, y - 1));
		}

		if y < self.0.len() - 1 && self.0[y + 1][x] == height + 1 {
			moves.push((x, y + 1));
		}

		moves
	}

	fn score_trailhead(&self, start: (usize, usize)) -> u32 {
		// Here, we'll do DFS from the trailhead, looking for a 9, building a "path" as we go.

		let mut stack: Vec<((usize, usize), Path)> = Vec::with_capacity(self.0.len() * self.0[0].len());
		let mut visited: HashSet<(usize, usize)> = HashSet::new();

		let mut paths: BTreeMap<(usize, usize), BTreeSet<Path>> = BTreeMap::new();

		stack.push((start, Vec::new()));

		while let Some(((x, y), path)) = stack.pop() {
			let mut path_amended = path.clone();

			path_amended.push((x, y));

			// If goal, add path to paths.
			if self.0[y][x] == 9 {
				// We've reached our goal. Add the amended path to the set of paths, then don't enqueue anything.
				paths.entry((x, y)).or_default().insert(path_amended);
				continue;
			}

			visited.insert((x, y));

			for (nx, ny) in self.legal_moves((x, y)) {
				if !visited.contains(&(nx, ny)) {
					stack.push(((nx, ny), path_amended.clone()));
				}
			}
		}

		if false {
			for (dest, paths) in &paths {
				println!(
					"Found {} unique paths from trailhead ({},{}) to destination ({},{})",
					paths.len(),
					start.0,
					start.1,
					dest.0,
					dest.1
				);

				for path in paths {
					self.display_path(path);
					println!();
				}
			}
		}

		u32::try_from(paths.keys().count()).unwrap()
	}

	fn display_path(&self, path: &Path) {
		let points: HashSet<(usize, usize)> = path.iter().copied().collect();

		for (y, row) in self.0.iter().enumerate() {
			for (x, &height) in row.iter().enumerate() {
				if points.contains(&(x, y)) {
					print!("{height}");
				} else {
					print!(".");
				}
			}
			println!();
		}
	}
}

impl PartSolve for Solution {
	fn parse(&mut self, map: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		let grid: Vec<Vec<u8>> = map
			.lines()
			.map(|line| {
				line
					.chars()
					.map(|c| match c {
						'.' => u8::MAX,
						'0'..='9' => u8::try_from(c.to_digit(10).unwrap()).unwrap(),
						_ => panic!("Unknown character in input: {c}"),
					})
					.collect()
			})
			.collect();

		Ok(Box::new(Grid(grid)))
	}

	fn part_one(&self, grid: &Box<dyn core::any::Any>) -> Option<String> {
		let grid = grid.downcast_ref::<Grid>()?;

		// Find trailheads (positions with height 0).
		let mut trailheads: Vec<(usize, usize)> = Vec::new();

		for (y, row) in grid.0.iter().enumerate() {
			for (x, &height) in row.iter().enumerate() {
				if height == 0 {
					trailheads.push((x, y));
				}
			}
		}

		Some(
			trailheads
				.iter()
				.map(|trailhead| grid.score_trailhead(*trailhead))
				.sum::<u32>()
				.to_string(),
		)
	}

	fn part_two(&self, _intermediate: &Box<dyn core::any::Any>) -> Option<String> {
		None
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day10.example.in.txt", part_one, literal "36");
part_test!(part_two, Solution, file "day10.example.in.txt", part_two, None);
