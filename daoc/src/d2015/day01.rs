pub enum Direction {
	Up,
	Down,
}

impl From<&Direction> for i32 {
	fn from(direction: &Direction) -> Self {
		match direction {
			Direction::Down => -1,
			Direction::Up => 1,
		}
	}
}

pub type Intermediate = Vec<Direction>;
pub type Output = i32;

/// Parses the input data and returns an `Intermediate` type.
///
/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(
		input
			.chars()
			.filter_map(|c| match c {
				'(' => Some(Direction::Up),
				')' => Some(Direction::Down),
				c if c.is_whitespace() => None,
				_ => panic!("unexpected character {c}"),
			})
			.collect(),
	)
}

/// Processes the parsed intermediate data and determines the solution for part one.
#[must_use]
pub fn part_one(directions: &Intermediate) -> Option<Output> {
	Some(directions.iter().map(i32::from).sum())
}

/// Processes the parsed intermediate data and determines the solution for part one.
#[must_use]
pub fn part_two(directions: &Intermediate) -> Option<Output> {
	let mut floor = 0;

	for (index, direction) in directions.iter().enumerate() {
		floor += i32::from(direction);

		if floor < 0 {
			let offset = i32::try_from(index).unwrap() + 1;
			return Some(offset);
		}
	}

	None
}

crate::generate_solver!(solve, =>, self);
