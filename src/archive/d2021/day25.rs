#![allow(dead_code, unused)]

use std::collections::{BTreeMap, BTreeSet};

#[derive(PartialEq, Clone, Copy)]
enum Direction {
	East,
	South,
}

#[derive(thiserror::Error, Debug)]
enum DirectionParseError {
	#[error("unrecognized character `{0}`")]
	UnrecognizedChar(char),
}

impl TryFrom<char> for Direction {
	type Error = DirectionParseError;
	fn try_from(char: char) -> Result<Self, Self::Error> {
		match char {
			'v' => Ok(Direction::South),
			'>' => Ok(Direction::East),
			_ => Err(DirectionParseError::UnrecognizedChar(char)),
		}
	}
}

#[derive(Copy, Clone)]
pub struct Space<const WIDTH: usize, const HEIGHT: usize> {
	contents: [[Option<Direction>; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> core::fmt::Display for Space<WIDTH, HEIGHT> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		for y in 0..HEIGHT {
			for x in 0..WIDTH {
				write!(
					f,
					"{}",
					match self.contents[y][x] {
						Some(Direction::East) => '>',
						Some(Direction::South) => 'v',
						None => '.',
					}
				)?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

impl<const WIDTH: usize, const HEIGHT: usize> core::str::FromStr for Space<WIDTH, HEIGHT> {
	type Err = core::convert::Infallible;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		let entries: Vec<((usize, usize), Direction)> = str
			.lines()
			.enumerate()
			.flat_map(|(y, line)| {
				line
					.chars()
					.enumerate()
					.map(move |(x, char)| ((x, y), char))
					.filter_map(|((x, y), char)| match Direction::try_from(char) {
						Ok(d) => Some((d, (x, y))),
						_ => None,
					})
					.map(|(d, (x, y))| ((x, y), d))
			})
			.collect();

		let mut contents = [[None; WIDTH]; HEIGHT];

		for ((x, y), direction) in entries {
			contents[y][x] = Some(direction);
		}

		Ok(Self { contents })
	}
}

impl<const WIDTH: usize, const HEIGHT: usize> Space<WIDTH, HEIGHT> {
	fn next_east(x: usize) -> usize {
		let mut x = x;

		x += 1;

		while x >= WIDTH {
			x -= WIDTH;
		}

		x
	}

	fn next_south(y: usize) -> usize {
		let mut y = y;

		y += 1;

		while y >= HEIGHT {
			y -= HEIGHT;
		}

		y
	}

	fn iterate(self) -> (Self, usize) {
		let mut contents = self.contents;

		let mut moves = 0;

		//println!("{}", self);

		let leading_east_edge: Vec<(usize, usize)> = (0..HEIGHT)
			.flat_map(|y| (0..WIDTH).map(move |x| (WIDTH - 1 - x, y)))
			.filter(|(x, y)| contents[*y][*x] == Some(Direction::East))
			.filter(|&(x, y)| contents[y][Self::next_east(x)].is_none())
			.collect();

		for (x, y) in leading_east_edge {
			contents[y][Self::next_east(x)] = contents[y][x];
			contents[y][x] = None;
			moves += 1;
		}

		let leading_south_edge: Vec<(usize, usize)> = (0..WIDTH)
			.flat_map(|x| (0..HEIGHT).map(move |y| (x, HEIGHT - 1 - y)))
			.filter(|(x, y)| contents[*y][*x] == Some(Direction::South))
			.filter(|&(x, y)| contents[Self::next_south(y)][x].is_none())
			.collect();

		for (x, y) in leading_south_edge {
			contents[Self::next_south(y)][x] = contents[y][x];
			contents[y][x] = None;
			moves += 1;
		}

		(Self { contents }, moves)
	}
}

pub type Intermediate = Space<139, 137>;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(input.parse().expect("failed to parse input"))
}

type Output = usize;

#[must_use]
pub fn part_one(floor: &Intermediate) -> Option<Output> {
	let mut space: Intermediate = (*floor);

	let mut counter = 0;

	loop {
		let (new_space, count) = space.iterate();

		counter += 1;

		if count == 0 {
			break;
		}

		space = new_space;
	}

	Some(counter)
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}

crate::generate_solver!(solve, =>, self);
