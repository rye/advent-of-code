pub type Intermediate = Vec<Move>;
pub type Output = u32;

/// Parses the input data and returns an `Intermediate` type.
///
/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let moves = input
		.trim()
		.split(", ")
		.map(str::parse)
		.collect::<anyhow::Result<Vec<Move>>>()?;

	Ok(moves)
}

/// Processes the parsed intermediate data and determines the solution for part one.
#[must_use]
pub fn part_one(moves: &Intermediate) -> Option<Output> {
	let mut direction = (0, 1);
	let mut pos = (0, 0);

	for m in moves {
		direction = m.dir.apply_to_tuple(direction);
		pos = (
			pos.0 + direction.0 * i32::from(m.dist),
			pos.1 + direction.1 * i32::from(m.dist),
		);
	}

	Some(taxicab_distance((0, 0), pos))
}

/// Processes the parsed intermediate data and determines the solution for part one.
#[must_use]
pub fn part_two(moves: &Intermediate) -> Option<Output> {
	let mut direction = (0, 1);
	let mut pos = (0, 0);

	let mut visit_counts: BTreeMap<(i32, i32), u32> = BTreeMap::new();

	for m in moves {
		direction = m.dir.apply_to_tuple(direction);

		for _ in 0..m.dist {
			visit_counts.entry(pos).and_modify(|c| *c += 1).or_insert(1);

			if visit_counts[&pos] == 2 {
				return Some(taxicab_distance((0, 0), pos));
			}

			pos = (pos.0 + direction.0, pos.1 + direction.1);
		}
	}

	None
}

crate::generate_solver!(solve, =>, self);

use std::collections::BTreeMap;

enum Direction {
	Left,
	Right,
}

impl Direction {
	fn apply_to_tuple(&self, (x, y): (i32, i32)) -> (i32, i32) {
		match self {
			Self::Left => (-y, x),
			Self::Right => (y, -x),
		}
	}
}

pub struct Move {
	dir: Direction,
	dist: u8,
}

impl core::str::FromStr for Move {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> anyhow::Result<Self> {
		let dir = match s.chars().next() {
			Some('L') => Direction::Left,
			Some('R') => Direction::Right,
			_ => return Err(anyhow::anyhow!("Invalid direction")),
		};
		let dist = s[1..].parse()?;
		Ok(Self { dir, dist })
	}
}

crate::test_example!(part_one_r2_l3, parse, part_one, "R2, L3", Some(5));

crate::test_example!(part_one_r2_r2_r2, parse, part_one, "R2, R2, R2", Some(2));

crate::test_example!(
	part_one_r5_l5_r5_r3,
	parse,
	part_one,
	"R5, L5, R5, R3",
	Some(12)
);

#[allow(clippy::cast_sign_loss)]
fn taxicab_distance((x0, y0): (i32, i32), (x1, y1): (i32, i32)) -> u32 {
	((x1 - x0).abs() + (y1 - y0).abs()) as u32
}

crate::test_example!(
	part_two_r8_r4_r4_r8,
	parse,
	part_two,
	"R8, R4, R4, R8",
	Some(4)
);
