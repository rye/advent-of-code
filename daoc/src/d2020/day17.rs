use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos(i8, i8, i8, i8);

pub type Intermediate = HashSet<Pos>;
type Output = u64;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(
		input
			.lines()
			.enumerate()
			.flat_map(|(y, line)| {
				line
					.chars()
					.enumerate()
					.filter(|&(_x, c)| c == '#')
					.map(move |(x, _c)| Pos(i8::try_from(x).unwrap(), i8::try_from(y).unwrap(), 0, 0))
			})
			.collect(),
	)
}

#[cfg(test)]
mod parse {
	use super::{Pos, parse};

	#[test]
	fn example() {
		let input = ".#.\n..#\n###";

		assert_eq!(
			parse(input),
			Ok(
				vec![
					Pos(1, 0, 0, 0),
					Pos(2, 1, 0, 0),
					Pos(0, 2, 0, 0),
					Pos(1, 2, 0, 0),
					Pos(2, 2, 0, 0),
				]
				.into_iter()
				.collect()
			)
		);
	}
}

#[rustfmt::skip]
static NEIGHBOR_D: [(i8, i8, i8, i8); 80] = [
	// dw=-1
	(-1, -1, -1, -1), (-1, -1,  0, -1), (-1, -1,  1, -1),
	(-1,  0, -1, -1), (-1,  0,  0, -1), (-1,  0,  1, -1),
	(-1,  1, -1, -1), (-1,  1,  0, -1), (-1,  1,  1, -1),
	( 0, -1, -1, -1), ( 0, -1,  0, -1), ( 0, -1,  1, -1),
	( 0,  0, -1, -1), ( 0,  0,  0, -1), ( 0,  0,  1, -1),
	( 0,  1, -1, -1), ( 0,  1,  0, -1), ( 0,  1,  1, -1),
	( 1, -1, -1, -1), ( 1, -1,  0, -1), ( 1, -1,  1, -1),
	( 1,  0, -1, -1), ( 1,  0,  0, -1), ( 1,  0,  1, -1),
	( 1,  1, -1, -1), ( 1,  1,  0, -1), ( 1,  1,  1, -1),

	// dw=0
	(-1, -1, -1,  0), (-1, -1,  0,  0), (-1, -1,  1,  0),
	(-1,  0, -1,  0), (-1,  0,  0,  0), (-1,  0,  1,  0),
	(-1,  1, -1,  0), (-1,  1,  0,  0), (-1,  1,  1,  0),
	( 0, -1, -1,  0), ( 0, -1,  0,  0), ( 0, -1,  1,  0),
	( 0,  0, -1,  0),                   ( 0,  0,  1,  0),
	( 0,  1, -1,  0), ( 0,  1,  0,  0), ( 0,  1,  1,  0),
	( 1, -1, -1,  0), ( 1, -1,  0,  0), ( 1, -1,  1,  0),
	( 1,  0, -1,  0), ( 1,  0,  0,  0), ( 1,  0,  1,  0),
	( 1,  1, -1,  0), ( 1,  1,  0,  0), ( 1,  1,  1,  0),

	// dw=+1
	(-1, -1, -1,  1), (-1, -1,  0,  1), (-1, -1,  1,  1),
	(-1,  0, -1,  1), (-1,  0,  0,  1), (-1,  0,  1,  1),
	(-1,  1, -1,  1), (-1,  1,  0,  1), (-1,  1,  1,  1),
	( 0, -1, -1,  1), ( 0, -1,  0,  1), ( 0, -1,  1,  1),
	( 0,  0, -1,  1), ( 0,  0,  0,  1), ( 0,  0,  1,  1),
	( 0,  1, -1,  1), ( 0,  1,  0,  1), ( 0,  1,  1,  1),
	( 1, -1, -1,  1), ( 1, -1,  0,  1), ( 1, -1,  1,  1),
	( 1,  0, -1,  1), ( 1,  0,  0,  1), ( 1,  0,  1,  1),
	( 1,  1, -1,  1), ( 1,  1,  0,  1), ( 1,  1,  1,  1),
];

fn count_neighbors_3d(active: &HashSet<Pos>) -> HashMap<Pos, usize> {
	let mut neighbors = HashMap::new();
	for Pos(x, y, z, _) in active {
		for (dx, dy, dz, _dw) in &NEIGHBOR_D[26..52] {
			*neighbors.entry(Pos(x + dx, y + dy, z + dz, 0)).or_insert(0) += 1;
		}
	}
	neighbors
}

fn count_neighbors_4d(active: &HashSet<Pos>) -> HashMap<Pos, usize> {
	let mut neighbors = HashMap::new();
	for Pos(x, y, z, w) in active {
		for (dx, dy, dz, dw) in &NEIGHBOR_D {
			*neighbors
				.entry(Pos(x + dx, y + dy, z + dz, w + dw))
				.or_insert(0) += 1;
		}
	}
	neighbors
}

fn process<CountingFn>(mut active: HashSet<Pos>, counting_fn: CountingFn) -> usize
where
	CountingFn: Fn(&HashSet<Pos>) -> HashMap<Pos, usize>,
{
	for _ in 0..6 {
		active = counting_fn(&active)
			.iter()
			.filter(|&(pos, &n)| n == 3 || (n == 2 && active.contains(pos)))
			.map(|(&pos, _)| pos)
			.collect();
	}

	active.len()
}

pub fn part_one(active_cells: &Intermediate) -> Option<Output> {
	let active: HashSet<Pos> = active_cells.clone();

	let count = process(active, count_neighbors_3d);

	Some(count as u64)
}

#[cfg(test)]
mod part_one {
	use super::{parse, part_one};

	#[test]
	fn example() {
		let input = ".#.\n..#\n###";

		assert_eq!(part_one(&parse(input).unwrap()), Some(112));
	}
}

pub fn part_two(active_cells: &Intermediate) -> Option<Output> {
	let active: HashSet<Pos> = active_cells.clone();

	let count = process(active, count_neighbors_4d);

	Some(count as u64)
}

#[cfg(test)]
mod part_two {
	use super::{parse, part_two};

	#[test]
	fn example() {
		let input = ".#.\n..#\n###";

		assert_eq!(part_two(&parse(input).unwrap()), Some(848));
	}
}

daocutil::generate_solver!(solve, =>, self);
