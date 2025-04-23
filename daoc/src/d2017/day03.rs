use std::collections::{BTreeMap, BTreeSet};

pub type Intermediate = usize;
pub type Output = usize;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(input.trim().parse()?)
}

#[must_use]
pub fn part_one(input: &Intermediate) -> Option<Output> {
	let nth: Option<(i32, i32)> = GridIter::default().nth(*input - 1);

	match nth {
		Some((x, y)) => Some(x.unsigned_abs() as usize + y.unsigned_abs() as usize),
		_ => None,
	}
}

#[must_use]
pub fn part_two(input: &Intermediate) -> Option<Output> {
	grid_values().find(|n| n > input)
}

daocutil::test_example!(part_one_1, parse, part_one, "1", Some(0));
daocutil::test_example!(part_one_12, parse, part_one, "12", Some(3));
daocutil::test_example!(part_one_23, parse, part_one, "23", Some(2));
daocutil::test_example!(part_one_1024, parse, part_one, "1024", Some(31));

struct GridIter {
	set: BTreeSet<(i32, i32)>,
	last: (i32, i32),
	dir: (i32, i32),
}

impl Default for GridIter {
	fn default() -> Self {
		let mut set = BTreeSet::new();
		set.insert((0, 0));
		Self {
			set,
			last: (0, 0),
			dir: (1, 0),
		}
	}
}

impl Iterator for GridIter {
	type Item = (i32, i32);

	fn next(&mut self) -> Option<Self::Item> {
		let cur = self.last;
		let next = (self.last.0 + self.dir.0, self.last.1 + self.dir.1);
		assert!(!self.set.contains(&next));

		self.set.insert(next);
		self.last = next;

		if !self
			.set
			.contains(&(next.0 + -self.dir.1, next.1 + self.dir.0))
		{
			self.dir = (-self.dir.1, self.dir.0);
		}

		Some(cur)
	}
}

#[test]
fn gen_grid_iter() {
	let mut gen_grid = GridIter::default();
	assert_eq!(gen_grid.next(), Some((0, 0)));
	assert_eq!(gen_grid.next(), Some((1, 0)));
	assert_eq!(gen_grid.next(), Some((1, 1)));
	assert_eq!(gen_grid.next(), Some((0, 1)));
	assert_eq!(gen_grid.next(), Some((-1, 1)));
	assert_eq!(gen_grid.next(), Some((-1, 0)));
	assert_eq!(gen_grid.next(), Some((-1, -1)));
	assert_eq!(gen_grid.next(), Some((0, -1)));
}

fn neighbors(pos: &(i32, i32)) -> impl Iterator<Item = (i32, i32)> + '_ {
	(-1..=1)
		.flat_map(move |offset_y| (-1..=1).map(move |offset_x| (pos.0 + offset_x, pos.1 + offset_y)))
		.filter(move |pos_t| pos_t != pos)
}

#[test]
fn neighbors_iter() {
	let mut neighbors = neighbors(&(0, 0));
	assert_eq!(neighbors.next(), Some((-1, -1)));
	assert_eq!(neighbors.next(), Some((0, -1)));
	assert_eq!(neighbors.next(), Some((1, -1)));
	assert_eq!(neighbors.next(), Some((-1, 0)));
	assert_eq!(neighbors.next(), Some((1, 0)));
	assert_eq!(neighbors.next(), Some((-1, 1)));
	assert_eq!(neighbors.next(), Some((0, 1)));
	assert_eq!(neighbors.next(), Some((1, 1)));
	assert_eq!(neighbors.next(), None);
}

fn grid_values() -> impl Iterator<Item = usize> {
	let initial_state: BTreeMap<(i32, i32), usize> = [((0, 0), 1)].into_iter().collect();

	GridIter::default().scan(initial_state, |scratch, new_loc| -> Option<usize> {
		let value_to_insert: usize = neighbors(&new_loc)
			.map(|neighbor_pos| scratch.get(&neighbor_pos).unwrap_or(&0))
			.sum();

		Some(*scratch.entry(new_loc).or_insert(value_to_insert))
	})
}

#[test]
fn grid_values_iter() {
	let mut grid_values = grid_values();
	assert_eq!(grid_values.next(), Some(1));
	assert_eq!(grid_values.next(), Some(1));
	assert_eq!(grid_values.next(), Some(2));
	assert_eq!(grid_values.next(), Some(4));
	assert_eq!(grid_values.next(), Some(5));
}

daocutil::generate_solver!(solve, =>, self);
