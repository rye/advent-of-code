use std::collections::{BTreeMap, btree_map::Entry};

pub type Intermediate = Vec<Direction>;
pub type Output = usize;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
	North,
	South,
	East,
	West,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct PVec([i32; 2]);

impl core::ops::Add<PVec> for PVec {
	type Output = PVec;
	fn add(self, other: PVec) -> Self::Output {
		PVec([self.0[0] + other.0[0], self.0[1] + other.0[1]])
	}
}

impl Direction {
	fn to_pvec(self) -> PVec {
		match self {
			Self::North => PVec([0, 1]),
			Self::South => PVec([0, -1]),
			Self::East => PVec([1, 0]),
			Self::West => PVec([-1, 0]),
		}
	}
}

impl From<char> for Direction {
	fn from(c: char) -> Direction {
		match c {
			'^' => Direction::North,
			'v' => Direction::South,
			'>' => Direction::East,
			'<' => Direction::West,
			_ => unreachable!(),
		}
	}
}

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(
		input
			.chars()
			.filter(|c| !c.is_whitespace())
			.map(Into::into)
			.collect(),
	)
}

#[must_use]
pub fn part_one(directions: &Intermediate) -> Option<Output> {
	let positions: Vec<PVec> = directions
		.iter()
		.copied()
		.map(Direction::to_pvec)
		.scan(PVec([0, 0]), |pos, cur| {
			let old = *pos;
			*pos = *pos + cur;
			Some(old)
		})
		.collect();

	let mut map: BTreeMap<PVec, usize> = BTreeMap::new();

	for position in positions {
		match map.entry(position) {
			Entry::Occupied(mut e) => {
				e.insert(e.get() + 1);
			}
			Entry::Vacant(e) => {
				e.insert(1);
			}
		}
	}

	Some(map.keys().len())
}

#[must_use]
pub fn part_two(directions: &Intermediate) -> Option<Output> {
	let chunks: Vec<[Direction; 2]> = directions
		.chunks_exact(2)
		.map(|pair| [pair[0], pair[1]])
		.collect();

	let santa_track: Vec<PVec> = chunks
		.iter()
		.map(|pair| &pair[0])
		.copied()
		.map(Direction::to_pvec)
		.scan(PVec([0, 0]), |pos, cur| {
			let old = *pos;
			*pos = *pos + cur;
			Some(old)
		})
		.collect();

	let robo_santa_track: Vec<PVec> = chunks
		.iter()
		.map(|pair| &pair[1])
		.copied()
		.map(Direction::to_pvec)
		.scan(PVec([0, 0]), |pos, cur| {
			let old = *pos;
			*pos = *pos + cur;
			Some(old)
		})
		.collect();

	let mut map: BTreeMap<PVec, usize> = BTreeMap::new();

	for track in [santa_track, robo_santa_track] {
		for position in track {
			match map.entry(position) {
				Entry::Occupied(mut e) => {
					e.insert(e.get() + 1);
				}
				Entry::Vacant(e) => {
					e.insert(1);
				}
			}
		}
	}

	Some(map.keys().len())
}

daocutil::generate_solver!(solve, =>, self);
